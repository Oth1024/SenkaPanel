use once_cell::sync::{Lazy, OnceCell};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

use crate::{
    common_definitions::senka_error::{SenkaError, SenkaErrorCode},
    tools::{
        database::{entities::FromRow, sql_builder::{self, SelectBuilder, SqlBuilder}},
        logger::logger_factory::{self, SenkaLogger},
    },
};

static DB_CLIENT_POOL: OnceCell<Pool<SqliteConnectionManager>> = OnceCell::new();
static LOGGER: Lazy<SenkaLogger> =
    Lazy::new(|| logger_factory::get_logger("tool", "database_client"));

// Call this function before logger is used
/// 在使用数据库之前需要调用此方法
/// # 参数
///     sqlite_file_path:     目标sqlite文件位置,
///     max_connection_count: 最大连接数
pub fn initialize_database_client(sqlite_file_path: &str, max_connection_count: u32) {
    LOGGER.debug(format!("Enter [initialize_database_client] with [sqlite_file_path]=[{}],[max_connection_count]=[{}]", sqlite_file_path, max_connection_count));
    let manager = SqliteConnectionManager::file(sqlite_file_path);
    DB_CLIENT_POOL
        .set(
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
        )
        .unwrap();
    LOGGER.debug_out("initialize_database_client");
}

/// 获取数据库连接池的实例
fn get_pool() -> &'static Pool<SqliteConnectionManager> {
    DB_CLIENT_POOL
        .get()
        .expect("Can not get sql client, maybe not initialize")
}

// 数据库入口实例
pub struct Db;

impl Db {
    /// 查询数据
    /// # 参数
    ///     table_name: 表名称
    pub fn select(table_name: &'static str) -> SelectBuilder {}

    /// 插入数据
    /// # 参数
    ///     table_name: 表名称
    pub fn insert(table_name: &'static str) -> InsertBuilder {}

    /// 更新数据
    /// # 参数
    ///     table_name: 表名称
    pub fn update(table_name: &'static str) -> UpdateBuilder {}

    /// 删除数据
    /// # 参数
    ///     table_name: 表名称
    pub fn delete(table_name: &'static str) -> DeleteBuilder {}

    /// 执行sql语句
    /// # 参数
    ///     sql: sql语句
    pub fn execute_sql(sql: &impl SqlBuilder) -> Result<(), SenkaError> {
        let build_result = sql.build();
        if let Ok(sql_token) = build_result {
            if let Ok(connection) = get_pool().get() {
                let get_statement_success = connection.prepare(sql_token.as_str());
                if let Ok(mut statement) = get_statement_success {
                    let execution_result = statement.raw_execute();
                    if let Ok(_) = execution_result {
                        Ok(())
                    }
                    else {
                        Err(SenkaError::new(SenkaErrorCode::Arguement, execution_result.err().unwrap().to_string()))
                    }
                }
                else {
                    Err(SenkaError::new(SenkaErrorCode::Arguement, get_statement_success.err().unwrap().to_string()))
                }
            }
            else {
                Err(SenkaError::new(SenkaErrorCode::Inner, String::from("Connection error, maybe not initialized")))
            }   
        }
        else {
            return Err(build_result.err().unwrap());
        }
    }

    /// 执行sql语句并等待结果
    /// # 参数
    ///     sql: sql语句
    pub fn execute_sql_with_result<T: FromRow + 'static >(sql: &impl SqlBuilder) -> Result<Vec<T>, SenkaError> {
        let build_result = sql.build();
        if let Ok(sql_token) = build_result {
            if let Ok(connection) = get_pool().get() {
                let get_statement_success = connection.prepare(sql_token.as_str());
                if let Ok(mut statement) = get_statement_success {
                    let execution_result = statement.query_map([], |row| T::from_row(row));
                    if let Ok(rows) = execution_result {
                        
                    }
                    else {
                        Err(SenkaError::new(SenkaErrorCode::Arguement, execution_result.err().unwrap().to_string()))
                    }
                }
                else {
                    Err(SenkaError::new(SenkaErrorCode::Arguement, get_statement_success.err().unwrap().to_string()))
                }
            }
            else {
                Err(SenkaError::new(SenkaErrorCode::Inner, String::from("Connection error, maybe not initialized")))
            }   
        }
        else {
            return Err(build_result.err().unwrap());
        }
    }
}
