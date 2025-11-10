use rocket::{self, launch, routes};

use crate::{
    routes::{Login::post_login, Regist::post_regist},
};

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
    // initialize_logger();
    // initialize_database_client(sqlite_file_path, max_connection_count);
}
