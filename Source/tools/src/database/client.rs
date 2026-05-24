use std::sync::atomic::AtomicBool;

use once_cell::sync::{Lazy, OnceCell};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, Schema};
use common::senka_error::{SenkaError, SenkaErrorCode};
use crate::{
    consts::DEFAULT_DATA_DIRECTORY,
    logger::logger_factory::{self, SenkaLogger},
};

// 连接池单例
static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();
static DATABASE_INITIALIZED: AtomicBool = AtomicBool::new(false);

static LOGGER: Lazy<SenkaLogger> = Lazy::new(|| logger_factory::get_logger("database", "client"));

/// 初始化数据库实例
/// 需要在程序开始时调用此方法
pub async fn initialize_database() {
    if (!DATABASE_INITIALIZED.load(std::sync::atomic::Ordering::Acquire)) {
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
        DATABASE_INITIALIZED.fetch_or(true, std::sync::atomic::Ordering::Acquire);
    }
}

pub async fn get_db<TEntity: EntityTrait>(entity_struct: TEntity) -> &'static DatabaseConnection {
    if DATABASE.get().is_none() {
        initialize_database().await;
        let db = DATABASE.get().unwrap();
        let stmt = Schema::new(DbBackend::Sqlite)
            .create_table_from_entity::<TEntity>(entity_struct)
            .if_not_exists()
            .to_owned();
        db.execute(db.get_database_backend().build(&stmt))
            .await
            .expect("Failed to create table");
    }
    DATABASE.get().unwrap()
}