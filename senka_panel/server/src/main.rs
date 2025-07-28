use rocket::{self, fs::FileServer, launch, routes};
use std::path::Path;

pub mod Routes;

use Routes::WebSockets::ssh_stream;

#[launch]
fn start_up() -> _ {
    rocket::build()
    .mount("/", routes!(ssh_stream))
}
