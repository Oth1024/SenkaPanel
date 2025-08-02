use rocket::{self, fs::FileServer, launch, routes};
use std::path::Path;

pub mod Protocol;
pub mod Routes;
pub mod Executor;
pub mod Once;

use Routes::Shell::ssh_stream;

#[launch]
fn start_up() -> _ {
    rocket::build()

    // Websocket
    .mount("/", routes!(ssh_stream))
}
