use std::sync::Arc;
use once_cell::sync::OnceCell;
use r2d2::Pool;
use rusqlite::{Connection, Result};
use r2d2_sqlite::SqliteConnectionManager;


// SQLite数据库连接池单例
pub static DB_CLIENT_POOL: OnceCell<Arc<Pool<SqliteConnectionManager>>> = OnceCell::new();

// 在使用该单例前需要初始化该单例
pub fn InitializeDatabaseClient(sqlite_file_path: &str, max_connection_count: u32) {
    let manager = SqliteConnectionManager::file(sqlite_file_path);
    DB_CLIENT_POOL.set(Arc::new(Pool::builder()
        .max_size(max_connection_count)
        .build(manager)
        .unwrap()));
}