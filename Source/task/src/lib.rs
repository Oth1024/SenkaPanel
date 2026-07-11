use common::senka_error::{self, SenkaError, SenkaErrorCode};
use process_manager::get_process_manager;

pub mod task;

/// 通过TaskId开启Task
pub async fn start_task(id: u32) -> Result<(), SenkaError> {
    // 执行 task
    match task::get_task_detail(id).await {
        Ok(mut task) => {
            let result = get_process_manager().start(task.command.clone(), task.args.clone(), false);
            if let Ok(process_info_id) = result {
                task.associated_process = Some(process_info_id);
            }
            else {
                return Err(result.err().unwrap());
            }
            Ok(())
        }
        Err(senka_error) => return Err(senka_error),
    }
}

/// 通过TaskId停止Task
pub async fn stop_task(id: u32) {
    match task::get_task_detail(id).await {
        Ok(task) => {
            if let Some(associated_process) = task.associated_process {
                let _ = get_process_manager().kill(associated_process);
            }
        }
        Err(_) => (),
    }
    return;
}

/// 通过TasnId重启Task
pub async fn restart_task(id: u32) -> Result<(), SenkaError> {
    match task::get_task_detail(id).await {
        Ok(task) => {
            if let Some(associated_process) = task.associated_process {
                let _ = get_process_manager().restart_by_id(associated_process);
                Ok(())
            }
            else {
                start_task(id).await
            }
        }
        Err(senka_error) => return Err(senka_error),
    }
}