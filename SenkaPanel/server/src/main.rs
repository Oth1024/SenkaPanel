use rocket::{self, launch, routes};
use tools::{
        consts::DEFAULT_LOG_DIRECTORY, database::client::initialize_database, logger::logger_factory::{initialize_logger}
    };
use crate::{
    routes::{Login::post_login, Regist::post_regist}
};

pub mod guards;
pub mod protocol;
pub mod routes;

#[launch]
async fn start_up() -> _ {
    initialize().await;
    rocket::build().mount("/", routes![post_login, post_regist])
}

async fn initialize() {
    // 初始化日志
    // let logger_config = config_manager::get_config::<LoggerConfig>()
    //     .await
    //     .expect("Cannot get or create logger config.");
    // initialize_logger(
    //     logger_config.log_level,
    //     logger_config.console_output,
    //     logger_config.file_output,
    //     DEFAULT_LOG_DIRECTORY,
    //     logger_config.file_size,
    //     logger_config.max_file_count,
    // );

    // 初始化数据库
    initialize_database().await;
}
