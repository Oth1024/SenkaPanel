use common::senka_error::{self, SenkaError, SenkaErrorCode};
use process_manager::{PROCESS_MANAGER, ProcessManager};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection::SqlxSqlitePoolConnection, EntityTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;
use tools::database::client::get_db;

use crate::task::Column;

pub mod task;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    
    pub uuid: String,

    pub task_name: String,

    pub command: String,

    pub icon_url: String,

    pub keep_alive: bool,

    pub associated_process_id: Option<u32>,

    pub index: u32,
}

impl Task {
    pub fn from_database_model(model: &task::Model) -> Self {
        Self {
            uuid: model.uuid.clone(),
            task_name: model.task_name.clone(),
            command: model.command.clone(),
            icon_url: model.icon_url.clone(),
            keep_alive: model.keep_alive,
            associated_process_id: model.associated_process_id, 
            index: model.index,
        }
    }

    pub fn to_database_model(&self) -> task::Model {
        task::Model {
            uuid: self.uuid.clone(),
            task_name: self.task_name.clone(),
            command: self.command.clone(),
            icon_url: self.icon_url.clone(),
            keep_alive: self.keep_alive,
            associated_process_id: self.associated_process_id,  
            index: self.index
        }
    }
}

// 增
pub async fn create_task_info(id: u128, task_name: String, command: String, icon_url: String, keep_alive: bool, associated_process_id: Option<u32>, index: u32) {
    let info = task::ActiveModel {
        uuid: Set(Uuid::from_u128(id).to_string()),
        task_name: Set(task_name),
        command: Set(command),
        icon_url: Set(icon_url),
        keep_alive: Set(keep_alive),
        associated_process_id: Set(associated_process_id),
        index: Set(index)
    };
    info.insert(get_db(task::Entity).await).await.unwrap();
}

pub async fn get_all_task_infos() -> Result<Vec<Task>, SenkaError> {
    // 从数据库中获取所有 task_info
    let result = task::Entity::find()
        .all(get_db(task::Entity).await).await;
    if let Ok(models) = result {
        return Ok(Vec::from_iter(models.iter().map(|model|Task::from_database_model(model))));
    }
    Err(SenkaError::null())
}

pub async fn get_task_detail(uuid: String) -> Result<Task, SenkaError> {
    let result = task::Entity::find_by_id(uuid)
        .one(get_db(task::Entity).await).await;
    if let Ok(Some(model)) = result {
        return Ok(Task::from_database_model(&model));
    }
    Err(SenkaError::null())
}

pub async fn delete_task_info(uuid: String) {
    // 从数据库中删除 task_info
    task::Entity::delete_by_id(uuid)
        .exec(get_db(task::Entity).await).await.unwrap();
}

pub async fn update_task_info(task_info: Task) {
    // 更新数据库中的 task_info
    let new_model = (&task_info).to_database_model();
    new_model.into_active_model().update(get_db(task::Entity).await).await.unwrap();
}

pub async fn execute_task(uuid: String) -> Result<(), SenkaError> {
    // 执行 task
    match get_task_detail(uuid).await {
        Ok(task) => {
            // 不是Task，则直接创建进程
            if !&task.keep_alive {
                match ProcessManager::start_raw(task.command.clone()) {
                    Ok(_) => return Ok(()),
                    Err(senka_error) => Err(senka_error)
                }
            }
            // 否则由ProcessManager托管
            else {
                // TODO 完善Creator
                let _ = PROCESS_MANAGER.start(task.command.clone(), String::from(""));
                return Ok(());
            }
        }
        Err(senka_error) => return Err(senka_error),
    }
}