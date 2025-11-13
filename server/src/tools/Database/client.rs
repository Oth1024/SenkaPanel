use sea_orm::{ Database, DatabaseConnection, ConnectOptions };
use once_cell::sync::OnceCell;

// 连接池单例
pub static  DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

/// 在使用数据库之前需要调用此函数
pub fn initialize_database(){
    
}