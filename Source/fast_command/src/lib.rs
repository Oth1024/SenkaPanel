use common::senka_error::{self, SenkaError, SenkaErrorCode};
use process_monitor::{PROCESS_MONITOR, ProcessMonitor};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection::SqlxSqlitePoolConnection, EntityTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;
use tools::database::client::get_db;

use crate::fast_command::Column;

pub mod fast_command;

#[derive(Debug, Clone, PartialEq)]
pub struct FastCommand {
    
    pub uuid: String,

    pub command_name: String,

    pub command_value: String,

    pub icon_url: String,

    pub keep_alive: bool,

    pub index: u32,
}

impl FastCommand {
    pub fn from_database_model(model: &fast_command::Model) -> Self {
        Self {
            uuid: model.uuid.clone(),
            command_name: model.command_name.clone(),
            command_value: model.command_value.clone(),
            icon_url: model.icon_url.clone(),
            keep_alive: model.keep_alive,
            index: model.index,
        }
    }

    pub fn to_database_model(&self) -> fast_command::Model {
        fast_command::Model {
            uuid: self.uuid.clone(),
            command_name: self.command_name.clone(),
            command_value: self.command_value.clone(),
            icon_url: self.icon_url.clone(),
            keep_alive: self.keep_alive,
            index: self.index
        }
    }
}

// 增
pub async fn create_fast_command_info(id: u128, command_name: String, command_value: String, icon_url: String, keep_alive: bool, index: u32) {
    let info = fast_command::ActiveModel {
        uuid: Set(Uuid::from_u128(id).to_string()),
        command_name: Set(command_name),
        command_value: Set(command_value),
        icon_url: Set(icon_url),
        keep_alive: Set(keep_alive),
        index: Set(index)
    };
    info.insert(get_db(fast_command::Entity).await).await.unwrap();
}

pub async fn get_all_fast_command_infos() -> Result<Vec<FastCommand>, SenkaError> {
    // 从数据库中获取所有 fast_command_info
    let result = fast_command::Entity::find()
        .all(get_db(fast_command::Entity).await).await;
    if let Ok(models) = result {
        return Ok(Vec::from_iter(models.iter().map(|model|FastCommand::from_database_model(model))));
    }
    Err(SenkaError::null())
}

pub async fn get_fast_command_detail(uuid: String) -> Result<FastCommand, SenkaError> {
    let result = fast_command::Entity::find_by_id(uuid)
        .one(get_db(fast_command::Entity).await).await;
    if let Ok(Some(model)) = result {
        return Ok(FastCommand::from_database_model(&model));
    }
    Err(SenkaError::null())
}

pub async fn delete_fast_command_info(uuid: String) {
    // 从数据库中删除 fast_command_info
    fast_command::Entity::delete_by_id(uuid)
        .exec(get_db(fast_command::Entity).await).await.unwrap();
}

pub async fn update_fast_command_info(fast_command_info: FastCommand) {
    // 更新数据库中的 fast_command_info
    let new_model = (&fast_command_info).to_database_model();
    new_model.into_active_model().update(get_db(fast_command::Entity).await).await.unwrap();
}

pub async fn execute_fast_command(uuid: String) -> Result<(), SenkaError> {
    // 执行 fast_command
    match get_fast_command_detail(uuid).await {
        Ok(fast_command) => {
            // 不是FastCommand，则直接创建进程
            if !&fast_command.keep_alive {
                match ProcessMonitor::start_raw(fast_command.command_value.clone()) {
                    Ok(_) => return Ok(()),
                    Err(senka_error) => Err(senka_error)
                }
            }
            // 否则由ProcessManager托管
            else {
                // TODO 完善Creator
                let _ = PROCESS_MONITOR.start(fast_command.command_value.clone(), String::from(""));
                return Ok(());
            }
        }
        Err(senka_error) => return Err(senka_error),
    }
}