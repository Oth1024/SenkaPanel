use tokio::process;

pub struct ProcessInfo {
    
    pub id: i32,

    pub process: process::Command
}