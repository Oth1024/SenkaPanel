use rocket::{self, launch, routes};

use crate::{
    routes::{Login::post_login, Regist::post_regist},
};

pub mod common_definitions;
pub mod guards;
pub mod protocol;
pub mod routes;
pub mod tools;

#[launch]
fn start_up() -> _ {
    initialize();
    rocket::build().mount("/", routes![post_login, post_regist])
}

fn initialize() {
    //初始化配置
    
    // 初始化日志
    // initialize_logger();

    // 初始化数据库
    // initialize_database_client(sqlite_file_path, max_connection_count);
}
