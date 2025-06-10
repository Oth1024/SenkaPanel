use std::collections::HashMap;
use once_cell::sync::OnceCell;

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

// Static lang management
static LANGUAGE_HELPER: OnceCell<LanguageHelper> = OnceCell::new();