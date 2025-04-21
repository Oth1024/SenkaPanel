use std::{
    collections::HashMap, fmt::format, ops::Add, path::{Path, PathBuf}, sync::{Arc, Mutex}
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::fs::{self, File};

const DEFAULT_CONFIG_PATH: &str = "./configs";
const DEFAULT_CONFIG_NAME: &str = "DefaultConfig.toml";

pub struct ConfigTool;

impl ConfigTool {

    pub async fn get_config<T>(config_name: &String, config_path: &String) -> Result<T, String> 
    where T: DeserializeOwned
    {
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

    pub async  fn set_config<T>(config_name: &String, config_path: &String, new_config_object: &T) -> Result<(), String> {
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
            let create_path = fs::create_dir(config_path).await;
            if let Err(create_path_failed) = create_path {
                println!("[Error] Can not create directory:{}!",config_path);
                return Err(String::from(create_path_failed.to_string()));
            }
            else {
                
            }
        }
        if let Some(config_path) = get_config_path {

        }
        else {
            File::create(config_path).await;
        }
        return Err(String::from("Can not set config."));
    }
}