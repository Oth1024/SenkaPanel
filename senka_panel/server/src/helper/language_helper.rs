use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::fmt::Display;
use toml;

use crate::helper::consts::{
    ENUS_LANG_RES, LANGRAGE_RESOURCE_DIR, RESOURCE_FILE_NAME, ZHCN_LANG_RES,
};

// Language definition
#[derive(PartialEq)]
pub enum Language {
    ZhCn,
    EnUs,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Language::ZhCn {
            write!(f, "ZhCn")
        }
        else {
            write!(f, "EnUs")
        }
    }
}

// Language helper
pub struct LanguageHelper {
    pub current_lang: Language,
    pub lang_res_ch: HashMap<String, String>,
    pub lang_res_en: HashMap<String, String>,
}

impl LanguageHelper {
    pub fn new() -> Self {
        let lang_string_res = get_local_lang_string_res();
        let instance = LanguageHelper {
            current_lang: Language::EnUs,
            lang_res_ch: lang_string_res.0,
            lang_res_en: lang_string_res.1,
        };
        instance
    }

    pub fn get_current_lang(&self) -> &Language {
        return &self.current_lang;
    }

    pub fn get_string_from_str(&self, string_key: &str, language: &Language) -> String {
        let key = String::from(string_key);

        if language == &Language::ZhCn {
            if self.lang_res_ch.contains_key(&key) {
                return String::from(&self.lang_res_ch[&key]);
            }
        }

        if language == &Language::EnUs {
            if self.lang_res_en.contains_key(&key) {
                return String::from(&self.lang_res_en[&key]);
            }
        }

        return String::from("Not Found");
    }

    pub fn get_string_from_string(&self, string_key: &str, language: &Language) -> String {
        let key = String::from(string_key);

        if language == &Language::ZhCn {
            if self.lang_res_ch.contains_key(&key) {
                return String::from(&self.lang_res_ch[&key]);
            }
        }

        if language == &Language::EnUs {
            if self.lang_res_en.contains_key(&key) {
                return String::from(&self.lang_res_en[&key]);
            }
        }

        return String::from("Not Found");
    }

    pub fn switch_current_lang(&mut self, current_language: Language) {
        self.current_lang = current_language;
    }
}

// First member of returned tuple is zh;
// Second member of returned tuple is en;
fn get_local_lang_string_res() -> (HashMap<String, String>, HashMap<String, String>) {
    let mut lang_res_zh = HashMap::<String, String>::new();
    let mut lang_res_en = HashMap::<String, String>::new();
    let language_res_dir = Path::new(LANGRAGE_RESOURCE_DIR);
    let mut dir_exist: bool = false;
    if !language_res_dir.exists() {
        if let Ok(_) = fs::create_dir_all(language_res_dir) {
            dir_exist = true;
        }
    } else {
        dir_exist = true;
    }
    if dir_exist {
        if let Ok(mut res_file) = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(language_res_dir.join(Path::new(RESOURCE_FILE_NAME)))
        {
            let mut content = String::new();
            if let Ok(_) = res_file.read_to_string(&mut content) {
                if let Ok(lang_res) =
                    toml::from_str::<HashMap<String, HashMap<String, String>>>(&content)
                {
                    if lang_res.contains_key(ZHCN_LANG_RES) && lang_res.contains_key(ENUS_LANG_RES) {
                        lang_res_zh = lang_res[ZHCN_LANG_RES].clone();
                        lang_res_en = lang_res[ENUS_LANG_RES].clone();
                        return (lang_res_zh, lang_res_en);
                    }
                }
            }
            let mut new_res = HashMap::<String, HashMap<String, String>>::new();
            new_res.insert(String::from(ZHCN_LANG_RES), HashMap::new());
            new_res.insert(String::from(ENUS_LANG_RES), HashMap::new());
            if let Ok(new_content) = toml::to_string(&new_res) {
                if let Ok(_) = res_file.write_all(new_content.as_bytes()) {}
            }
        }
    }
    (lang_res_zh, lang_res_en)
}

// Static lang management
pub static LANGUAGE_HELPER: Lazy<LanguageHelper> = Lazy::new(|| LanguageHelper::new());
