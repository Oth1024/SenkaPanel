use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection::SqlxSqlitePoolConnection};
use uuid::Uuid;
use tools::database::client::DATABASE;

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
    pub fn from_database_model(model: fast_command::Model) -> Self {
        Self {
            uuid: model.uuid,
            command_name: model.command_name,
            command_value: model.command_value,
            icon_url: model.icon_url,
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

pub async fn get_all_fast_command_infos() -> Vec<FastCommandInfo> {
    // 从数据库中获取所有 fast_command_info
    Vec::<FastCommandInfo>::new()
}

pub async fn get_fast_command_detail(uuid: String) -> FastCommandInfo {
    // 从数据库中获取 fast_command_info
    FastCommandInfo {
        uuid,
        command_name: String::new(),
        command_value: String::new(),
        icon_url: String::new(),
        fast_command: false,
        index: 0,
    }
}

pub async fn delete_fast_command_info(uuid: String) {
    // 从数据库中删除 fast_command_info
}

pub async fn update_fast_command_info(fast_command_info: FastCommandInfo) {
    // 更新数据库中的 fast_command_info
}

pub async fn execute_fast_command(uuid: String) {
    // 执行 fast_command
}