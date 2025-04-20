use std::{
    collections::HashMap, ops::Add, path::{Path, PathBuf}, sync::{Arc, Mutex}
};
use serde::{Deserialize, Serialize};
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

    pub fn regist_config<T>(&mut self, config_name: String, config: &T) -> Result<(), String> 
    where T: Clone
    {
        if self.config_infos.contains_key(&config_name) {
            let message = format!("[Error] Can not regist config with config name:{} because it was registed!", &config_name);
            println!("{}", &message);
            return Err(message);
        }
        else {
            self.config_infos.insert(config_name, config.clone());
            let set_config_file = self.set_config(config);
            if set_config_file == Ok(()) {
                return Ok(());
            }
            else {
                
            }
        }
    }

    pub fn read_config<T>(&self) -> T {}

    pub fn set_config<T>(&self, new_config_object: &T) -> Result<(), String> {}

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
    read_at_first: bool,
}

impl Clone for ConfigInfo {
    fn clone(&self) -> Self {
        ConfigInfo { 
            config_name: self.config_name.clone(),
            config_path: self.config_path.clone(),
            read_at_first: self.read_at_first
         }
    }
}