use std::sync::atomic::{AtomicU32, Ordering};

use chrono::{DateTime, Utc};
use common::senka_error::{SenkaError, SenkaErrorCode};
use tokio::{process::{self, Child}, task};
use dashmap::{DashMap, DashSet};
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

pub static PROCESS_MANAGER: Lazy<ProcessManager> = 
    Lazy::new(|| ProcessManager::new());

pub struct ProcessManager {
    check_interval: u32,
    pub process_infos: DashMap<u32, ProcessInfo>,
    pub monitored_process: DashSet<u32>,
    process_index: AtomicU32,
}

impl ProcessManager {
    // 创建ProcessManager对象
    fn new() -> Self {
        ProcessManager {
            check_interval: 1,
            process_infos: DashMap::new(),
            monitored_process: DashSet::new(),
            process_index: AtomicU32::new(0),
        }
    }

    pub fn start_monitor(&self) {
        // 创建一个线程，监控monitored_process
        // 获取process实体，并刷新状态
        task::spawn_blocking(f)
        for id in self.monitored_process.iter() {
            let process_info = self.process_infos[id];
        }
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
        self.monitored_process.insert(id);

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
        self.monitored_process.remove(&id);
        Ok(())
    }
}