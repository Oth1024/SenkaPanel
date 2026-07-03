use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use common::senka_error::{SenkaError, SenkaErrorCode};
use tokio::{
    process::{self, Child, ChildStdin, ChildStdout, ChildStderr},
    task::JoinHandle,
    time,
};
use std::process::Stdio;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use sysinfo::{Pid, System};

#[derive(Debug)]
/// 定义进程的状态，监控线程会根据状态判断进程的管理逻辑
pub enum ProcessStatus {
    /// 进程未创建或创建失败，此时接受线程创建
    None,
    /// 线程创建并正在运行，此时仅接受进程状态更新
    Running,
    /// 进程正常或异常退出，退出码的定义与对应系统平台的进程退出码一致，此时接受进程重启
    Exited(i32),
    /// 进程被手动中止，此时不接受进程重启
    Stopped,
}

#[derive(Debug)]
/// 定义进程句柄的类型
pub enum ProcessHandle {
    // 无子进程
    None,
    // 由本进程创建的子进程句柄，支持输入输出控制、状态管理
    Managed(Child),
    // 通过其他方式捕获的进程句柄，仅支持状态管理
    Dued(Pid)
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub process: ProcessHandle,
    pub status: ProcessStatus,
    pub command: String,
    pub args: Vec<String>,
    pub auto_restart: bool,
    pub creator: String,
    pub created_time: DateTime<Utc>,
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
}

impl ProcessInfo {
    pub fn new(process_child: ProcessHandle, command: String, args: Vec<String>, auto_restart: bool, creator: String) -> Self {
        let (process, status, stdin, stdout, stderr) = match process_child {
            ProcessHandle::None => (ProcessHandle::None, ProcessStatus::None, None, None, None),
            ProcessHandle::Managed(mut child) => {
                let stdin = child.stdin.take();
                let stdout = child.stdout.take();
                let stderr = child.stderr.take();
                (ProcessHandle::Managed(child), ProcessStatus::Running, stdin, stdout, stderr)
            }
            ProcessHandle::Dued(pid) => (ProcessHandle::Dued(pid), ProcessStatus::Running, None, None, None)
        };

        ProcessInfo {
            process,
            status,
            command,
            args,
            auto_restart,
            creator,
            created_time: Utc::now(),
            stdin,
            stdout,
            stderr,
        }
    }

    pub fn from_dued(pid: u32, command: String, auto_restart: bool, creator: String) -> Self {
        Self::new(
            ProcessHandle::Dued(Pid::from_u32(pid)),
            command,
            vec![],
            auto_restart,
            creator,
        )
    }
}

static PROCESS_MANAGER: Lazy<ProcessManager> = 
    Lazy::new(|| ProcessManager::new());

pub fn get_process_manager() -> &'static ProcessManager {
    &PROCESS_MANAGER
}

pub struct ProcessManager {
    pub process_infos: DashMap<u32, ProcessInfo>,
    check_interval: u32,
    process_index: AtomicU32,
    monitor_thread: Mutex<Option<JoinHandle<()>>>,
    stop_signal: AtomicBool,
}

impl ProcessManager {
    /// 创建ProcessManager对象，此方法不期待由其他地方调用
    fn new() -> Self {
        ProcessManager {
            process_infos: DashMap::new(),
            check_interval: 1,
            process_index: AtomicU32::new(0),
            monitor_thread: Mutex::new(None),
            stop_signal: AtomicBool::new(false),
        }
    }

    /// 启动一个监控线程，如果该监控线程已存在，则不创建
    pub async fn start_monitor(&self) {
        let mut handle = self.monitor_thread.lock().unwrap();
        if handle.as_ref().is_some_and(|h| !h.is_finished()) {
            return; // 监控任务已在运行
        }

        self.stop_signal.store(false, Ordering::Release);

        let new_handle = tokio::spawn(async {
            loop {
                let ids: Vec<u32> = get_process_manager().process_infos.iter()
                    .map(|entry| *entry.key())
                    .collect();
                let has_dued = get_process_manager().process_infos.iter()
                    .any(|entry| matches!(entry.process, ProcessHandle::Dued(_)));
                let system = if has_dued { Some(System::new()) } else { None };
                for id in ids {
                    if let Some(mut process_info) = get_process_manager().process_infos.get_mut(&id) {
                        // 检查进程实体，并通过进程实体状态更新缓存中的进程信息状态
                        match &mut process_info.process {
                            ProcessHandle::Managed(child) => {
                                match child.try_wait() {
                                    Ok(Some(exit_status)) => {
                                        // 进程退出，将状态设置为Exited
                                        process_info.status = ProcessStatus::Exited(
                                            exit_status.code().unwrap_or(-1),
                                        );
                                        process_info.process = ProcessHandle::None;
                                        process_info.stdin = None;
                                        process_info.stdout = None;
                                        process_info.stderr = None;
                                    }
                                    Ok(None) => {
                                        // 进程在运行，状态设置为Running
                                        process_info.status = ProcessStatus::Running;
                                    }
                                    Err(_) => {
                                        // 获取进程信息异常，认为进程异常/子进程句柄不存在/子进程实体不存在，设置状态为Exited
                                        process_info.status = ProcessStatus::Exited(-1);
                                        process_info.process = ProcessHandle::None;
                                        process_info.stdin = None;
                                        process_info.stdout = None;
                                        process_info.stderr = None;
                                    }
                                }
                            }
                            ProcessHandle::Dued(pid) => {
                                if let Some(ref entry) = system && entry.process(*pid).is_some() {
                                    process_info.status = ProcessStatus::Running;
                                } else {
                                    process_info.status = ProcessStatus::Exited(-1);
                                    process_info.process = ProcessHandle::None;
                                    process_info.stdin = None;
                                    process_info.stdout = None;
                                    process_info.stderr = None;
                                }
                            }
                            ProcessHandle::None => {}
                        }
                        // 检查进程状态
                        // 如果是Exited则重新开始
                        // 如果是Running则DONOTHING
                        // 如果是Stopped则DONOTHING
                        if let ProcessStatus::Exited(_) = process_info.status && process_info.auto_restart {
                            let _ = get_process_manager().restart(id).await;
                        }
                    }
                }

                if get_process_manager().stop_signal.load(Ordering::Acquire) {
                    break;
                }

                time::sleep(Duration::from_secs(
                    get_process_manager().check_interval as u64,
                ))
                .await;
            }
        });

        let _ = handle.insert(new_handle);
    }

    pub fn stop_monitor(&self) {
        self.stop_signal.store(true, Ordering::Release);
        if let Some(handle) = self.monitor_thread.lock().unwrap().take() {
            handle.abort();
        }
    }

    pub fn start(&self, command: String, args: Vec<String>, auto_restart:bool, creator: String) -> Result<u32, SenkaError> {
        let id = self.process_index.fetch_add(1, Ordering::SeqCst);
        let mut cmd = process::Command::new(&command);
        for arg in &args {
            cmd.arg(arg);
        }
        let child_result = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        let process_info = match child_result {
            Ok(child) => ProcessInfo::new(ProcessHandle::Managed(child), command, args, auto_restart, creator),
            Err(_) => ProcessInfo::new(ProcessHandle::None, command, args, auto_restart, creator),   
        };
        self.process_infos.insert(id, process_info);

        Ok(id)
    }

    /// 中止一个进程，除了杀死进程实体外，还会将进程状态设置为Stopped，防止自动重启
    pub fn kill(&self, id: u32) {
        if let Some(mut entry) = self.process_infos.get_mut(&id) {
            entry.status = ProcessStatus::Stopped;
            entry.stdin = None;
            entry.stdout = None;
            entry.stderr = None;
            if let ProcessHandle::Managed(mut child) = std::mem::replace(&mut entry.process, ProcessHandle::None) {
                let _ = child.start_kill();
            }
        }
    }

    pub async fn restart(&self, id: u32) -> Result<(), SenkaError> {
        let (command, args) = match self.process_infos.get(&id) {
            Some(info) => (info.command.clone(), info.args.clone()),
            None => return Err(SenkaError::new(SenkaErrorCode::Arguement, format!("Process with id[{}] not exists", id))),
        };

        let old_child = match self.process_infos.get_mut(&id) {
            Some(mut entry) => std::mem::replace(&mut entry.process, ProcessHandle::None),
            None => return Err(SenkaError::new(SenkaErrorCode::Arguement, format!("Process with id[{}] not exists", id))),
        };

        match old_child {
            ProcessHandle::Managed(mut child) => {
                let _ = child.kill().await;
            }
            ProcessHandle::Dued(pid) => {
                let _ = System::new_all().process(pid).map(|p| p.kill());
            }
            ProcessHandle::None => {}
        }

        match self.process_infos.get_mut(&id) {
            Some(mut entry) => {
                let mut cmd = process::Command::new(&command);
                for arg in &args {
                    cmd.arg(arg);
                }
                match cmd
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn() {
                    Ok(mut child) => {
                        entry.stdin = child.stdin.take();
                        entry.stdout = child.stdout.take();
                        entry.stderr = child.stderr.take();
                        entry.process = ProcessHandle::Managed(child);
                        entry.status = ProcessStatus::Running;
                    }
                    Err(_) => {
                        entry.stdin = None;
                        entry.stdout = None;
                        entry.stderr = None;
                        entry.process = ProcessHandle::None;
                        entry.status = ProcessStatus::None;
                    }
                }
            }
            None => {}
        }

        Ok(())
    }

    pub fn remove(&self, id: u32) -> Result<(), SenkaError> {
        if let Some(process_info_entry) = self.process_infos.get(&id) {
            if matches!(process_info_entry.status, ProcessStatus::Running) {
                return Err(SenkaError::new(
                    SenkaErrorCode::Inner,
                    format!("process {} is still running, cannot remove", id),
                ));
            }
        }
        self.process_infos.remove(&id);
        Ok(())
    }

}