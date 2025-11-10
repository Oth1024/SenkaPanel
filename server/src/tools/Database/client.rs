use once_cell::sync::OnceCell;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

static DB_CLIENT_POOL: OnceCell<Pool<SqliteConnectionManager>> = OnceCell::new();

// Call this function before logger is used
pub fn initialize_database_client(sqlite_file_path: &str, max_connection_count: u32) {
    let manager = SqliteConnectionManager::file(sqlite_file_path);
    DB_CLIENT_POOL.set(
        Pool::builder()
            .max_size(max_connection_count)
            .build(manager)
            .expect(
                format!(
                    "Sql initialize error with file path[{}] max_connection[{}]!",
                    sqlite_file_path, max_connection_count
                )
                .as_str(),
            ),
    ).unwrap();
}

pub fn get_client() -> &'static Pool<SqliteConnectionManager> {
    DB_CLIENT_POOL.get().expect("Can not get sql client, maybe not initialize")
}