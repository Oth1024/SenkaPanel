use once_cell::sync::Lazy;

use crate::{helper::language_helper::{self, LANGUAGE_HELPER}, *};

#[test]
fn test_lang_helper_get_static() {
    let language_helper = &LANGUAGE_HELPER;
    print!("test_lang_helper_get_static success!");
    print!("current_langauge:{}", &language_helper.get_current_lang());
}