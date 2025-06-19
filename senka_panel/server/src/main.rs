pub mod helper;
mod pages;

use std::path::Path;

use pages::home_page::home_page_func;
use rocket::{self, fs::FileServer, launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn start_up() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![home_page_func])
        .mount("/", FileServer::from(Path::new("./static/")))
}

#[cfg(test)]
mod tests;
