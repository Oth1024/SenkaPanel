use once_cell::sync::Lazy;

use crate::{helper::language_helper::{self, LANGUAGE_HELPER}, *};

#[test]
fn test_lang_helper_get_static() {
    if let Some(language_helper) = Lazy::get(&LANGUAGE_HELPER) {
        print!("test_lang_helper_get_static success!");
        print!("current_langauge:{}", &language_helper.current_lang)
    }
}