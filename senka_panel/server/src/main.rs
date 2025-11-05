use rocket::{self, fs::FileServer, launch, routes};

pub mod Protocol;
pub mod Routes;
pub mod Tool;

#[launch]
fn start_up() -> _ {
    rocket::build()
}
