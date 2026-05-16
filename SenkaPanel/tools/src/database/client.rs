use once_cell::sync::{Lazy, OnceCell};
use sea_orm::{Database, DatabaseConnection};
use common::senka_error::{SenkaError, SenkaErrorCode};
use crate::{
    consts::DEFAULT_DATA_DIRECTORY,
    logger::logger_factory::{self, SenkaLogger},
};

// 连接池单例
pub static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

static LOGGER: Lazy<SenkaLogger> = Lazy::new(|| logger_factory::get_logger("database", "client"));

/// 初始化数据库实例
/// 需要在程序开始时调用此方法
pub async fn initialize_database() {
    match Database::connect(format!(
        "sqlite://{}db.sqlite?mode=rwc",
        DEFAULT_DATA_DIRECTORY
    ))
    .await
    {
        Ok(db) => {
            DATABASE.set(db).unwrap();
        }
        Err(db_err) => {
            LOGGER.error(&SenkaError::new(SenkaErrorCode::Inner, db_err.to_string()))
        }
    };
}
