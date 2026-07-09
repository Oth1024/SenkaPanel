use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Duration;

use chrono::{DateTime, Utc};
use common::senka_error::{SenkaError, SenkaErrorCode};
use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    process::{self, Child, ChildStdin, ChildStdout, ChildStderr},
    sync::broadcast,
    sync::mpsc,
    task::JoinHandle,
    time,
};
use std::process::Stdio;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use sysinfo::{Pid, System};

use crate::ProcessHandle::Managed;

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
/// 从 Child 中提取的原始输入输出管道
struct PipeDispatcher {
    stdin: Option<Arc<tokio::sync::Mutex<ChildStdin>>>,
    stdout_reader: Option<BufReader<ChildStdout>>,
    stderr_reader: Option<BufReader<ChildStderr>>,

    stdin_rx: mpsc::Receiver<String>,
    stdout_tx: broadcast::Sender<String>,
    stderr_tx: broadcast::Sender<String>
}

impl PipeDispatcher {
    fn new(stdin_rx: mpsc::Receiver<String>, stdout_tx: broadcast::Sender<String>, stderr_tx: broadcast::Sender<String>) -> Self {
        PipeDispatcher {
            stdin: None,
            stdout_reader: None,
            stderr_reader: None,
            stdin_rx,
            stdout_tx,
            stderr_tx
        }
    }

    fn update(&mut self, stdin: ChildStdin, stdout: ChildStdout, stderr: ChildStderr) {
        self.stdin.replace(Arc::new(tokio::sync::Mutex::new(stdin)));
        self.stdout_reader.replace(BufReader::new(stdout));
        self.stderr_reader.replace(BufReader::new(stderr));
    }
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub status: ProcessStatus,
    pub command: String,
    pub args: Vec<String>,
    pub auto_restart: bool,
    pub creator: String,
    pub created_time: DateTime<Utc>,

    process: ProcessHandle,
    stdin_tx: Option<mpsc::Sender<String>>,
    stdout_rx: Option<broadcast::Receiver<String>>,
    stderr_rx: Option<broadcast::Receiver<String>>,
    pipe_dispatcher: Option<PipeDispatcher>,
}

impl ProcessInfo {
    /// 外部创建时使用start方法
    /// 单独使用new方法不会启动监控
    /// 也不会订阅输入输出管道
    pub fn new(
        command: String, 
        args: Vec<String>, 
        auto_restart: bool, 
        creator: String,
    ) -> Self {
        // 构造时创建消息通道，后续monitor循环中使用
        let (stdin_tx, stdin_rx) = mpsc::channel::<String>(64);
        let (stdout_tx, stdout_rx) = broadcast::channel::<String>(64);
        let (stderr_tx, stderr_rx) = broadcast::channel::<String>(64);
        ProcessInfo {
            status: ProcessStatus::None,
            command,
            args,
            auto_restart,
            creator,
            created_time: Utc::now(),
            process: ProcessHandle::None,
            stdin_tx: Some(stdin_tx),
            stdout_rx: Some(stdout_rx),
            stderr_rx: Some(stderr_rx),
            pipe_dispatcher: Some(PipeDispatcher::new(stdin_rx, stdout_tx, stderr_tx)),
        }
    }

    /// 通过非托管进程创建ProcessInfo，这种情况下不支持输入输出控制
    /// 适用于需要管理进程的生命周期，但是不关系进程的输入输出的情况
    pub fn from_dued(
        pid: u32,
        command: String,
        args: Vec<String>,
         auto_restart: bool,
         creator: String) -> Self {
        let pid_entitiy = Pid::from_u32(pid);
        let mut system = System::new();
        system.refresh_all();
        let process = system.process(pid_entitiy);
        let status = if process.is_some() { ProcessStatus::Running } else { ProcessStatus::None };
        let mut process_info = ProcessInfo::new(command, args, auto_restart, creator);
        process_info.process = ProcessHandle::Dued(pid_entitiy);
        process_info
    }

    /// 获取std output的一个输出通道
    pub fn get_stdout_recv(&self) -> Option<broadcast::Receiver<String>> {
        self.stdout_rx.as_ref().map(|rx| rx.resubscribe())
    }

    /// 获取std error的一个输出通道
    pub fn get_stderr_recv(&self) -> Option<broadcast::Receiver<String>> {
        self.stderr_rx.as_ref().map(|rx| rx.resubscribe())
    }

    /// 获取std in的一个输入通道
    pub fn get_stdin_sender(&self) -> Option<mpsc::Sender<String>> {
        self.stdin_tx.clone()
    }

    // 更新ProcessHandle，同时更新分发
    fn update_process(&mut self, mut process: ProcessHandle) {
        match process {
            // 如果是chlid，则从中提取stdin、stdout、stderr，然后存到dispatcher中
            ProcessHandle::Managed(ref mut child) => {
                if let (Some(stdin), Some(stdout), Some(stderr)) = (
                    child.stdin.take(),
                    child.stdout.take(),
                    child.stderr.take(),
                ) {
                    if let Some(ref mut dispatcher) = self.pipe_dispatcher {
                        dispatcher.update(stdin, stdout, stderr);
                    }
                }
            }
            // 一般不会进入此分支，更新的子进程一定为child
            _ => {}
        }
        self.process = process;
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
                                    }
                                    Ok(None) => {
                                        // 进程在运行，状态设置为Running
                                        process_info.status = ProcessStatus::Running;
                                    }
                                    Err(_) => {
                                        // 获取进程信息异常，认为进程异常/子进程句柄不存在/子进程实体不存在，设置状态为Exited
                                        process_info.status = ProcessStatus::Exited(-1);
                                    }
                                }
                            }
                            ProcessHandle::Dued(pid) => {
                                if let Some(ref entry) = system && entry.process(*pid).is_some() {
                                    process_info.status = ProcessStatus::Running;
                                } else {
                                    process_info.status = ProcessStatus::Exited(-1);
                                    process_info.update_process(ProcessHandle::None);
                                }
                            }
                            ProcessHandle::None => {}
                        }
                        // 先处理输入输出，防止命令执行的太快直接退出
                        // 此时先restart会替换stdin、stdout、stderr管道
                        // 导致之前的输入输出丢失
                        // 处理stdin输入：从stdin_rx接收数据，spawn异步写入，不阻塞轮询
                        if let Some(ref mut dispatcher) = process_info.pipe_dispatcher {
                            if let Some(ref stdin) = dispatcher.stdin {
                                if let Ok(data) = dispatcher.stdin_rx.try_recv() {
                                    let stdin = stdin.clone();
                                    tokio::spawn(async move {
                                        let mut guard = stdin.lock().await;
                                        let _ = guard.write_all(data.as_bytes()).await;
                                        let _ = guard.flush().await;
                                    });
                                }
                            }
                            // 读取stdout输出，写入stdout_tx
                            if let Some(ref mut reader) = dispatcher.stdout_reader {
                                let mut line = String::new();
                                match tokio::time::timeout(Duration::ZERO, reader.read_line(&mut line)).await {
                                    Ok(Ok(n)) if n > 0 => {
                                        let _ = dispatcher.stdout_tx.send(line);
                                    }
                                    _ => {}
                                }
                            }
                            // 读取stderr输出，写入stderr_tx
                            if let Some(ref mut reader) = dispatcher.stderr_reader {
                                let mut line = String::new();
                                match tokio::time::timeout(Duration::ZERO, reader.read_line(&mut line)).await {
                                    Ok(Ok(n)) if n > 0 => {
                                        let _ = dispatcher.stderr_tx.send(line);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        // 检查进程状态
                        // 如果是None，说明进程没有正确创建，则需创建Process
                        // 如果是Running则DONOTHING
                        // 如果是Stopped则DONOTHING
                        // 如果是Exited，则说明进程结束，如果这时检测到auto restart标志则需自动重启
                        let mut do_restart = false;
                        match process_info.status {
                            ProcessStatus::None => do_restart = true,
                            ProcessStatus::Exited(_) if process_info.auto_restart => do_restart = true,
                            _ => {}
                        }
                        if do_restart {
                            let _ = get_process_manager().restart_entry(&mut *process_info);
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
        let process_info = ProcessInfo::new(command, args, auto_restart, creator);
        self.process_infos.insert(id, process_info);
        Ok(id)
    }

    /// 中止一个进程，除了杀死进程实体外，还会将进程状态设置为Stopped，防止自动重启
    pub fn kill(&self, id: u32) {
        if let Some(mut entry) = self.process_infos.get_mut(&id) {
            entry.status = ProcessStatus::Stopped;
            if let ProcessHandle::Managed(mut child) = std::mem::replace(&mut entry.process, ProcessHandle::None) {
                let _ = child.start_kill();
            }
        }
    }

    /// 通过 id 重启进程，供外部调用
    pub fn restart_by_id(&self, id: u32) -> Result<(), SenkaError> {
        match self.process_infos.get_mut(&id) {
            Some(mut entry) => self.restart_entry(&mut *entry),
            None => Err(SenkaError::new(SenkaErrorCode::Arguement, format!("Process with id[{}] not exists", id))),
        }
    }

    /// 直接操作 ProcessInfo 重启进程，供轮询等已持有引用的场景调用
    pub fn restart_entry(&self, entry: &mut ProcessInfo) -> Result<(), SenkaError> {
        let command = entry.command.clone();
        let args = entry.args.clone();
        let old_child = std::mem::replace(&mut entry.process, ProcessHandle::None);

        // 杀掉旧进程放到后台，不阻塞
        tokio::spawn(async move {
            match old_child {
                ProcessHandle::Managed(mut child) => {
                    let _ = child.kill().await;
                }
                ProcessHandle::Dued(pid) => {
                    let _ = System::new_all().process(pid).map(|p| p.kill());
                }
                ProcessHandle::None => {}
            }
        });

        let mut cmd = process::Command::new(&command);
        for arg in &args {
            cmd.arg(arg);
        }
        match cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => {
                entry.update_process(ProcessHandle::Managed(child));
                entry.status = ProcessStatus::Running;
            }
            Err(_) => {
                entry.process = ProcessHandle::None;
                entry.status = ProcessStatus::None;
            }
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