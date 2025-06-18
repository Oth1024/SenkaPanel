pub mod helper;
mod pages;

use rocket::{self, launch, routes};
use rocket_dyn_templates::Template;
use pages::home_page::home_page_func;

#[launch]
fn start_up() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![home_page_func])
}

#[cfg(test)]
mod tests;