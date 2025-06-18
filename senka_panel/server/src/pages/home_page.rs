use std::collections::HashMap;

use rocket::{self, get, response::content};
use rocket_dyn_templates::{Template, context};
use tera::{Context, Tera};

use crate::helper::language_helper::{self, LANGUAGE_HELPER};

#[get("/")]
pub fn home_page_func() -> Template {
    if let Ok(mut home_page_tera) = Tera::new("\\templates\\*.html") {
        let mut context = tera::Context::new();

        let language_helper = &LANGUAGE_HELPER;
        let senka_panel_title = language_helper.get_string_from_str("senka_panel_title");
        let login_tab = language_helper.get_string_from_str("login_tab");
        let more_tab = language_helper.get_string_from_str("more_tab");
        let switch_language = language_helper.get_string_from_str("switch_language");
        let source_code_tab = language_helper.get_string_from_str("source_code_tab");
        let about_senka_panel_tab = language_helper.get_string_from_str("about_senka_panel_tab");
        let quick_start_button = language_helper.get_string_from_str("quick_start_button");
        let author_announcement = language_helper.get_string_from_str("author_announcement");

        context.insert("senka_panel_title", &senka_panel_title);
        context.insert("login_tab", &login_tab);
        context.insert("more_tab", &more_tab);
        context.insert("switch_language", &switch_language);
        context.insert("source_code_tab", &source_code_tab);
        context.insert("about_senka_panel_tab", &about_senka_panel_tab);
        context.insert("quick_start_button", &quick_start_button);
        context.insert("author_announcement", &author_announcement);

        if let Ok(rendered) = home_page_tera.render("home.html.tera", &context) {
            return Template::render("home.html.tera", rendered);
        }
    }
    Template::render("not_found.html.tera", "")
}
