pub mod helper;
mod pages;

use std::path::Path;

use pages::home_page::{
    home_page_default,
    home_page_en,
    home_page_zh
};
use rocket::{self, fs::FileServer, launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn start_up() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![home_page_default,home_page_en,home_page_zh])
        .mount("/", FileServer::from(Path::new("./static/")))
}

#[cfg(test)]
mod tests;
