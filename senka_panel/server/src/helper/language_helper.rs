use std::collections::HashMap;
use tokio;

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
}

// Static lang management
// TODO
// static 

// pub fn get_string_from_str(string_key: &str) -> String {

// }

// pub fn get_string_from_string(string_key: String) -> String {

// }