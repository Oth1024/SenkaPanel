use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

// 连接池单例
pub static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

/// 在使用数据库之前需要调用此函数
pub fn initialize_database() {}
