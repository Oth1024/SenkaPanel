use chrono::{DateTime, Utc};
use tokio::process::Child;
use dashmap::DashMap;

#[derive(Debug)]
pub enum ProcessStatus {
    Running,
    Exited(i32),
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub id: u32,
    pub process: Child,
    pub status: ProcessStatus,
    pub command: String,
    pub creator: String,
    pub created_time: DateTime<Utc>,
}

impl ProcessInfo {
    pub fn new(process: Child, command: String, creator: String) -> Option<Self> {
        if let Some(id) = process.id() {
            return Some(ProcessInfo {
                id: id,
                process,
                status: ProcessStatus::Running,
                command,
                creator,
                created_time: Utc::now(),
            });
        }
        else {
            None
        }
    }
}

pub struct ProcessManager {
    pub process_infos: DashMap<u32, ProcessInfo>
}