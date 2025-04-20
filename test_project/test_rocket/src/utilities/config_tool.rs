use std::{
    collections::HashMap, fmt::format, ops::Add, path::{Path, PathBuf}, sync::{Arc, Mutex}
};
use rocket::{error, http::tls::rustls::internal::msgs::message, response::content};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::fs::{self, File};

const DEFAULT_CONFIG_PATH: &str = "./configs";
const DEFAULT_CONFIG_NAME: &str = "DefaultConfig.toml";

pub struct ConfigTool {
    config_infos: HashMap<String, ConfigInfo>,
}

impl ConfigTool {
    fn new() -> Self {
        let config_infos = HashMap::new();
        ConfigTool { config_infos }
    }

    pub fn get_instance() -> Arc<Mutex<Self>> {
        static mut INSTANCE: Option<Arc<Mutex<ConfigTool>>> = None;
        unsafe {
            INSTANCE
                .get_or_insert_with(|| Arc::new(Mutex::new(ConfigTool::new())))
                .clone()
        }
    }

    pub fn initialize(&mut self) {
        self.read_default_config();
    }

    pub async  fn regist_config<T>(&mut self, config_name: &String, config_path: &String, config: &T) -> Result<(), String> {
        if self.config_infos.contains_key(config_name) {
            let message = format!("[Error] Can not regist config with config name:{} because it was registed!", &config_name);
            println!("{}", &message);
            return Err(message);
        }
        else {
            let config_info = ConfigInfo::new(&config_name, &config_name);
            self.config_infos.insert(config_name.clone(), config_info);
            let set_config_file = self.set_config(config_path, config).await;
            if Ok(()) == set_config_file {
                return Ok(());
            }
            else {
                let message = format!("[Error] Can not regist config at path:{}!", &config_path);
                println!("{}", &message);
                return Err(message);
            }
        }
    }

    pub async fn get_config<T>(&self, config_name: &String) -> Result<T, String> 
    where T: DeserializeOwned
    {
        if !self.config_infos.contains_key(config_name) {
            let message = format!("[Error] Can not get config by name:{}, maybe it was not registed.", config_name);
            return Err(message);
        }
        let config_path = &self.config_infos[config_name].config_path;
        let do_read_file_or_dirs = fs::read_dir(config_path).await;
        if let Ok(mut file_or_dirs) = do_read_file_or_dirs {
            while let Ok(Some(entry)) = file_or_dirs.next_entry().await {
                let path = entry.path();
                if !path.is_dir() {
                    if path.as_path().ends_with(&config_name) {
                        if let Some(path_string) = path.as_path().to_str(){
                            let content = fs::read_to_string(path_string).await;
                            if let Ok(content_string) = content {
                                let get_toml = toml::from_str::<T>(&content_string);
                                if let Ok(toml_content) = get_toml {
                                    return Ok(toml_content);
                                }
                                else if let Err(error) = get_toml {
                                    let error_message = error.to_string();
                                    return Err(error_message);
                                }
                            }
                            else if let Err(error) = content {
                                let error_message = error.to_string();
                                return Err(error_message);
                            }
                        }
                    }
                }
            }
            return Err(format!("[Error] Can not get config file of:{} at path {}", config_name, &config_path));
        }
        else if let Err(error) = do_read_file_or_dirs {
            let error_message = error.to_string();
            return Err(error_message);
        }
        return Err(String::from("Uncovered error!"));
    }

    pub async  fn set_config<T>(&self, config_name: &String, new_config_object: &T) -> Result<(), String> {
        if !self.config_infos.contains_key(config_name) {
            return Err(String::from(format!("Can not set config {} because it was not registed.", config_name)));
        }
        let config_path = &self.config_infos[config_name].config_path;
        let mut retry_time = 0;
        while retry_time < 6 {
           let do_read_file_or_dirs = fs::read_dir(config_path).await;
            let mut get_config_path: Option<String> = None;
            if let Ok(mut file_or_dirs) = do_read_file_or_dirs {
                while let Ok(Some(entry)) = file_or_dirs.next_entry().await {
                    let path = entry.path();
                    if !path.is_dir() {
                        if path.as_path().ends_with(config_name) {
                            if let Some(path_string) = path.as_path().to_str(){
                                get_config_path = Some(String::from(path_string))
                            }
                        }
                    }
                }
        
            }
            else {
                println!("[Error] Can not find directory:{}, try make directory...",config_path);
                fs::create_dir(config_path);
            }
            if let Some(config_path) = get_config_path {
                let content = fs::read_to_string(config_path).await;
                if let Ok(content_str) = content {
                    let get_toml = toml::from_str::<HashMap<String, ConfigInfo>>(&content_str);
                    if let Ok(toml_content) = get_toml {
                    
                    }
                    else {
                        println!("Can not read toml from default config because it is in unexpected format!");
                        panic!()
                    }
                }
                else {
                    println!("Can not read config in default config, maybe file is broken!");
                }
                break;
            }
            else {
                println!("[Error] Can not find default config at directory:{}, try create default config...",DEFAULT_CONFIG_PATH);
                File::create(String::add(String::from(DEFAULT_CONFIG_PATH), DEFAULT_CONFIG_NAME)).await;
            }
            retry_time += 1;
        }
        return Err(String::from("Can not set config."));
    }

    async fn read_default_config(&mut self) {
        let mut retry_time = 0;
        loop {
            let do_read_file_or_dirs = fs::read_dir(DEFAULT_CONFIG_PATH).await;
            let mut default_config: Option<String> = None;
            if let Ok(mut file_or_dirs) = do_read_file_or_dirs {
                while let Ok(Some(entry)) = file_or_dirs.next_entry().await {
                    let path = entry.path();
                    if !path.is_dir() {
                        if path.as_path().ends_with(DEFAULT_CONFIG_NAME) {
                            if let Some(path_string) = path.as_path().to_str(){
                                default_config = Some(String::from(path_string))
                            }
                        }
                    }
                }
        
            }
            else {
                println!("[Error] Can not find directory:{}, try make directory...",DEFAULT_CONFIG_PATH);
                fs::create_dir(DEFAULT_CONFIG_PATH);
            }
            if let Some(config_path) = default_config {
                let content = fs::read_to_string(config_path).await;
                if let Ok(content_str) = content {
                    let get_toml = toml::from_str::<HashMap<String, ConfigInfo>>(&content_str);
                    if let Ok(toml_content) = get_toml {
                        for config_name_and_config in toml_content.iter(){
                            self.config_infos.insert(String::from(config_name_and_config.0), config_name_and_config.1.clone());
                        }
                    }
                    else {
                        println!("Can not read toml from default config because it is in unexpected format!");
                        panic!()
                    }
                }
                else {
                    println!("Can not read config in default config, maybe file is broken!");
                }
                break;
            }
            else {
                println!("[Error] Can not find default config at directory:{}, try create default config...",DEFAULT_CONFIG_PATH);
                File::create(String::add(String::from(DEFAULT_CONFIG_PATH), DEFAULT_CONFIG_NAME)).await;
            }
            retry_time += 1;
            if retry_time > 5 {
                println!("Read default file failed!");
                panic!()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigInfo {
    config_name: String,
    config_path: String,
}

impl ConfigInfo {
    pub fn new(config_name: &String, config_path: &String) -> Self {
        ConfigInfo { 
            config_name: config_name.clone(),
            config_path: config_path.clone()
        }
    }
}

impl Clone for ConfigInfo {
    fn clone(&self) -> Self {
        ConfigInfo { 
            config_name: self.config_name.clone(),
            config_path: self.config_path.clone(),
         }
    }
}