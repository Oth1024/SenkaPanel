use common::senka_error::{self, SenkaError, SenkaErrorCode};
use process_manager::get_process_manager;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection::SqlxSqlitePoolConnection, EntityTrait, IntoActiveModel, QueryFilter};
use tools::database::client::get_db;

use crate::task::Column;

pub mod task;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    
    pub id: u32,

    pub task_name: String,

    pub command: String,

    pub args: Vec<String>,

    pub icon_url: String,

    pub keep_alive: bool,

    pub index: u32,

    pub associated_process: Option<u32>,
}

impl Task {
    pub fn from_database_model(model: &task::Model) -> Self {
        let args: Vec<String> = serde_json::from_str(&model.args).unwrap_or_default();
        Self {
            id: model.id.clone(),
            task_name: model.task_name.clone(),
            command: model.command.clone(),
            args,
            icon_url: model.icon_url.clone(),
            keep_alive: model.keep_alive,
            index: model.index,
            associated_process: model.associated_process,
        }
    }

    pub fn to_database_model(&self) -> task::Model {
        task::Model {
            id: self.id.clone(),
            task_name: self.task_name.clone(),
            command: self.command.clone(),
            args: serde_json::to_string(&self.args).unwrap_or_default(),
            icon_url: self.icon_url.clone(),
            keep_alive: self.keep_alive,
            index: self.index,
            associated_process: self.associated_process,
        }
    }
}

// 增
pub async fn create_task_info(id: u32, task_name: String, command: String, args: Vec<String>, icon_url: String, keep_alive: bool, index: u32, associated_process: Option<u32>) {
    let model = task::Model::new(
        id,
        task_name,
        command,
        args,
        icon_url,
        keep_alive,
        index,
        associated_process,
    );
    model.into_active_model().insert(get_db(task::Entity).await).await.unwrap();
}

pub async fn delete_task_info(id: u32) {
    // 从数据库中删除 task_info
    task::Entity::delete_by_id(id)
        .exec(get_db(task::Entity).await).await.unwrap();
}

pub async fn update_task_info(task_info: Task) {
    // 更新数据库中的 task_info
    let new_model = (&task_info).to_database_model();
    new_model.into_active_model().update(get_db(task::Entity).await).await.unwrap();
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

pub async fn get_task_detail(id: u32) -> Result<Task, SenkaError> {
    let result = task::Entity::find_by_id(id)
        .one(get_db(task::Entity).await).await;
    if let Ok(Some(model)) = result {
        return Ok(Task::from_database_model(&model));
    }
    Err(SenkaError::null())
}

pub async fn execute_task(id: u32, user_name: String) -> Result<(), SenkaError> {
    // 执行 task
    match get_task_detail(id).await {
        Ok(task) => {
            // 不是Task，则直接创建进程
            if !&task.keep_alive {
                // TODO 完善Creator
                match get_process_manager().start(task.command.clone(), task.args.clone(), false, String::from("")) {
                    Ok(_) => return Ok(()),
                    Err(senka_error) => Err(senka_error)
                }
            }
            // 否则由ProcessManager托管
            else {
                // TODO 完善Creator
                let _ = get_process_manager().start(task.command.clone(), task.args.clone(), false, String::from(""));
                return Ok(());
            }
        }
        Err(senka_error) => return Err(senka_error),
    }
}

pub async fn stop_task(id: u32, user_name: String) {

}