use common::senka_error::{SenkaError, SenkaErrorCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection::SqlxSqlitePoolConnection, EntityTrait, QueryFilter};
use uuid::Uuid;
use tools::database::client::DATABASE;

use crate::fast_command::Column;

pub mod fast_command;

#[derive(Debug, Clone, PartialEq)]
pub struct FastCommandInfo {
    
    pub uuid: String,

    pub command_name: String,

    pub command_value: String,

    pub icon_url: String,

    pub fast_command: bool,

    pub index: u32,
}

impl FastCommandInfo {
    pub fn from_database_model(model: &fast_command::Model) -> Self {
        Self {
            uuid: model.uuid.clone(),
            command_name: model.command_name.clone(),
            command_value: model.command_value.clone(),
            icon_url: model.icon_url.clone(),
            fast_command: model.fast_command,
            index: model.index,
        }
    }
}

pub struct FastCommandManager {
    pub fast_command_infos: Vec<FastCommandInfo>,
}

// 增
pub async  fn create_fast_command_info(id: u128, command_name: String, command_value: String, icon_url: String, fast_command: bool, index: u32) {
    let info = fast_command::ActiveModel {
        uuid: Set(Uuid::from_u128(id).to_string()),
        command_name: Set(command_name),
        command_value: Set(command_value),
        icon_url: Set(icon_url),
        fast_command: Set(fast_command),
        index: Set(index)
    };
    info.insert(DATABASE.get().unwrap()).await.unwrap();
}

pub async fn get_all_fast_command_infos() -> Result<Vec<FastCommandInfo>, SenkaError> {
    // 从数据库中获取所有 fast_command_info
    let result = fast_command::Entity::find()
        .all(DATABASE.get().unwrap()).await;
    if let Ok(models) = result {
        return Ok(Vec::from_iter(models.iter().map(|model|FastCommandInfo::from_database_model(model))));
    }
    Err(SenkaError::null())
}

pub async fn get_fast_command_detail(uuid: String) -> Result<FastCommandInfo, SenkaError> {
    let result = fast_command::Entity::find()
        .filter(Column::Uuid.eq(uuid))
        .one(DATABASE.get().unwrap()).await;
    if let Ok(Some(model)) = result {
        return Ok(FastCommandInfo::from_database_model(&model));
    }
    Err(SenkaError::null())
}

pub async fn delete_fast_command_info(uuid: String) {
    // 从数据库中删除 fast_command_info
    fast_command::Entity::delete_many()
        .filter(Column::Uuid.eq(uuid))
        .exec(DATABASE.get().unwrap()).await.unwrap();
}

pub async fn update_fast_command_info(fast_command_info: FastCommandInfo) {
    // 更新数据库中的 fast_command_info
    
}

pub async fn execute_fast_command(uuid: String) {
    // 执行 fast_command
}