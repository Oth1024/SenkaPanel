use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use common::senka_error::{SenkaError, SenkaErrorCode};
use tokio::{process::{self, Child}, task::JoinHandle, time};
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub enum ProcessStatus {
    None,
    Running,
    Exited(i32),
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub process: Option<Child>,
    pub status: ProcessStatus,
    pub command: String,
    pub creator: String,
    pub created_time: DateTime<Utc>
}

impl ProcessInfo {
    pub fn new(process_child: Option<Child>, command: String, creator: String) -> Self {
        let (process, status) = match process_child {
            Some(child) => (Some(child), ProcessStatus::Running),
            None => (None, ProcessStatus::None),
        };

        ProcessInfo {
            process,
            status,
            command,
            creator,
            created_time: Utc::now(),
        }
    }
}

pub static PROCESS_MONITOR: Lazy<ProcessMonitor> = 
    Lazy::new(|| ProcessMonitor::new());

pub struct ProcessMonitor {
    pub process_infos: DashMap<u32, ProcessInfo>,
    check_interval: u32,
    process_index: AtomicU32,
    monitor_handle: Mutex<Option<JoinHandle<()>>>,
}

impl ProcessMonitor {
    // 创建ProcessMonitor对象
    fn new() -> Self {
        ProcessMonitor {
            process_infos: DashMap::new(),
            check_interval: 1,
            process_index: AtomicU32::new(0),
            monitor_handle: Mutex::new(None),
        }
    }

    pub fn start_monitor(&self) {
        let mut handle = self.monitor_handle.lock().unwrap();
        if handle.as_ref().is_some_and(|h| !h.is_finished()) {
            return; // 监控任务已在运行
        }

        let new_handle = tokio::spawn(async {
            loop {
                let ids: Vec<u32> = PROCESS_MONITOR.process_infos.iter()
                    .map(|entry| *entry.key())
                    .collect();

                for id in ids {
                    if let Some(mut entry) = PROCESS_MONITOR.process_infos.get_mut(&id) {
                        if let Some(ref mut child) = entry.process {
                            match child.try_wait() {
                                Ok(Some(exit_status)) => {
                                    entry.status = ProcessStatus::Exited(
                                        exit_status.code().unwrap_or(-1),
                                    );
                                    entry.process = None;
                                }
                                Ok(None) => {} // 进程仍在运行
                                Err(_) => {
                                    entry.status = ProcessStatus::Exited(-1);
                                    entry.process = None;
                                }
                            }
                        }
                    }
                }

                time::sleep(Duration::from_secs(
                    PROCESS_MONITOR.check_interval as u64,
                ))
                .await;
            }
        });

        *handle = Some(new_handle);
    }

    pub fn start_raw(command: String) -> Result<Child, SenkaError> {
        match process::Command::new(&command).spawn() {
            Ok(child) => return Ok(child),
            Err(error) => return Err(SenkaError::new(SenkaErrorCode::Arguement, error.to_string())),
        }
    }

    pub fn start(&self, command: String, creator: String) -> Result<u32, SenkaError> {
        let id = self.process_index.fetch_add(1, Ordering::SeqCst);
        let child_result = process::Command::new(&command).spawn();
        let process_info = match child_result {
            Ok(child) => ProcessInfo::new(Some(child), command, creator),
            Err(_) => ProcessInfo::new(None, command, creator),
        };
        self.process_infos.insert(id, process_info);

        Ok(id)
    }

    pub fn kill(&self, id: u32) {
        if let Some(mut entry) = self.process_infos.get_mut(&id) {
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