use chrono::{DateTime, Utc};
use common::senka_error::{SenkaError, SenkaErrorCode};
use tokio::process::{self, Child};
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
    pub monitored_process: Vec<u32>,
    process_index: u32
}

impl ProcessManager {
    // 创建ProcessManager对象
    fn new() -> Self {
        ProcessManager {
            check_interval: 5,
            process_infos: DashMap::new(),
            monitored_process: Vec::new(),
            process_index: 0
        }
    }

    pub fn start_monitor(&mut self) {

    }

    pub fn start(&mut self, command: String, creator: String) -> Result<u32, SenkaError> {
        let id = self.process_index;
        self.process_index += 1;
        let child_result = process::Command::new(&command).spawn();
        let process_info = match child_result {
            Ok(child) => ProcessInfo::new(Some(child), command, creator),
            Err(_) => ProcessInfo::new(None, command, creator),
        };
        self.process_infos.insert(id, process_info);
        self.monitored_process.push(id);

        Ok(id)
    }

    pub fn kill(&mut self, id: u32) {
        if let Some(mut entry) = self.process_infos.get_mut(&id) {
            if let Some(mut child) = entry.process.take() {
                let _ = child.start_kill();
            }
        }
    }

    pub async fn restart(&mut self, id: u32) {
        let command = match self.process_infos.get(&id) {
            Some(info) => info.command.clone(),
            None => return,
        };

        let mut child_opt = None;
        if let Some(mut entry) = self.process_infos.get_mut(&id) {
            child_opt = entry.process.take();
        }
        if let Some(mut child) = child_opt {
            let _ = child.kill().await;
        }

        if let Some(mut entry) = self.process_infos.get_mut(&id) {
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
    }

    pub fn remove(&mut self, id: u32) -> Result<(), SenkaError> {
        if let Some(entry) = self.process_infos.get(&id) {
            if matches!(entry.status, ProcessStatus::Running) {
                return Err(SenkaError::new(
                    SenkaErrorCode::Inner,
                    format!("process {} is still running, cannot remove", id),
                ));
            }
        }
        self.process_infos.remove(&id);
        self.monitored_process.retain(|&x| x != id);
        Ok(())
    }
}