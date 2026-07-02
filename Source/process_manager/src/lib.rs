use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use common::senka_error::{SenkaError, SenkaErrorCode};
use tokio::{process::{self, Child}, task::JoinHandle, time};
use dashmap::DashMap;
use once_cell::sync::Lazy;

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
    Orphan()
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub process: ProcessHandle,
    pub status: ProcessStatus,
    pub command: String,
    pub auto_restart: bool,
    pub creator: String,
    pub created_time: DateTime<Utc>
}

impl ProcessInfo {
    pub fn new(process_child: ProcessHandle, command: String, auto_restart: bool, creator: String) -> Self {
        let (process, status) = match process_child {
            Some(child) => (Some(child), ProcessStatus::Running),
            None => (None, ProcessStatus::None),
        };

        ProcessInfo {
            process,
            status,
            command,
            auto_restart,
            creator,
            created_time: Utc::now(),
        }
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
                for id in ids {
                    if let Some(mut process_info) = get_process_manager().process_infos.get_mut(&id) {
                        // 检查进程实体，并通过进程实体状态更新缓存中的进程信息状态
                        if let Some(ref mut child) = process_info.process {
                            match child.try_wait() {
                                Ok(Some(exit_status)) => {
                                    // 进程退出，将状态设置为Exited
                                    process_info.status = ProcessStatus::Exited(
                                        exit_status.code().unwrap_or(-1),
                                    );
                                    process_info.process = None;
                                }
                                Ok(None) => {
                                    // 进程在运行，状态设置为Running
                                    process_info.status = ProcessStatus::Running;
                                }
                                Err(_) => {
                                    // 获取进程信息异常，认为进程异常/子进程句柄不存在/子进程实体不存在，设置状态为Exited
                                    process_info.status = ProcessStatus::Exited(-1);
                                    process_info.process = None;
                                }
                            }
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

    pub fn start(&self, command: String, auto_restart:bool, creator: String) -> Result<u32, SenkaError> {
        let id = self.process_index.fetch_add(1, Ordering::SeqCst);
        let child_result = process::Command::new(&command).spawn();
        let process_info = match child_result {
            Ok(child) => ProcessInfo::new(Some(child), command, auto_restart, creator),
            Err(_) => ProcessInfo::new(None, command, auto_restart, creator),   
        };
        self.process_infos.insert(id, process_info);

        Ok(id)
    }

    /// 中止一个进程，除了杀死进程实体外，还会将进程状态设置为Stopped，防止自动重启
    pub fn kill(&self, id: u32) {
        if let Some(mut entry) = self.process_infos.get_mut(&id) {
            entry.status = ProcessStatus::Stopped;
            if let Some(mut child) = entry.process.take() {
                let _ = child.start_kill();
            }
        }
    }

    pub async fn restart(&self, id: u32) -> Result<(), SenkaError> {
        let command = match self.process_infos.get(&id) {
            Some(info) => info.command.clone(),
            None => return Err(SenkaError::new(SenkaErrorCode::Arguement, format!("Process with id[{}] not exists", id))),
        };

        let old_child = match self.process_infos.get_mut(&id) {
            Some(mut entry) => entry.process.take(),
            None => return Err(SenkaError::new(SenkaErrorCode::Arguement, format!("Process with id[{}] not exists", id))),
        };

        if let Some(mut child) = old_child {
            let _ = child.kill().await;
        }

        match self.process_infos.get_mut(&id) {
            Some(mut entry) => {
                match process::Command::new(&command).spawn() {
                    Ok(child) => {
                        entry.process = Some(child);
                        entry.status = ProcessStatus::Running;
                    }
                    Err(_) => {
                        entry.process = None;
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