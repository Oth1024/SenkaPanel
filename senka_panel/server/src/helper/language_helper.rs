use std::{collections::HashMap, hash::Hash};
use once_cell::sync::OnceCell;
use std::path::Path;

#[derive(PartialEq)]
pub enum Language {
    ZhCn,
    EnUs
}

pub struct LanguageHelper {
    pub current_lang: Language,
    pub lang_res: HashMap<String, String>
}

impl LanguageHelper {
    pub fn new() -> Self {
        let lang_string_res = get_local_lang_string_res();
        let instance = LanguageHelper {
            current_lang: Language::EnUs,
            lang_res: lang_string_res
        };
        instance
    }

    pub fn get_current_lang(&self) -> &Language {
        return &self.current_lang;
    }

    pub fn get_string_from_str(&self, string_key: &str) -> String {
        let key = String::from(string_key);
        if self.lang_res.contains_key(&key) {
            return String::from(&self.lang_res[&key]);
        }
        return String::from("Not Found");
    }

    pub fn get_string_from_string(&self, string_key: &str) -> String {
        let key = String::from(string_key);
        if self.lang_res.contains_key(&key) {
            return String::from(&self.lang_res[&key]);
        }
        return String::from("Not Found");
    }

    pub fn switch_current_lang(&mut self, current_language: Language) {
        self.current_lang = current_language;
    }
}

fn get_local_lang_string_res() -> HashMap<String, String> {
    let mut lang_res = HashMap::<String, String>::new();

    lang_res
}

// Static lang management
static LANGUAGE_HELPER: OnceCell<LanguageHelper> = OnceCell::new();