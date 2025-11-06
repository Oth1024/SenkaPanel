use rocket::{self, launch, routes};

use crate::routes::{Login::post_login, Regist::post_regist};

pub mod guards;
pub mod protocol;
pub mod routes;
pub mod tools;

#[launch]
fn start_up() -> _ {
    rocket::build()
    .mount("/", routes![post_login, post_regist])
}
