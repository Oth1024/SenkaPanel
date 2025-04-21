use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    collections::HashMap,
    fmt::format,
    ops::Add,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tokio::{fs::{self, File}, sync::watch::error};

pub struct ConfigTool;

impl ConfigTool {
    pub async fn get_config<T>(config_name: &String, config_path: &String) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let get_file = File::open(config_path).await;
        if let Ok(open_file_success) = get_file {
            
        }
        else if let Err(error_info) = get_file {
            error!("{}", error_info);
            return Err(String::from(error_info));
        }
        let error_message = "Uncoverred exception!";
        error!("{}", error_message);
        return Err(String::from(error_message));
    }

    pub async fn set_config<T>(
        config_name: &String,
        config_path: &String,
        new_config_object: &T,
    ) -> Result<(), String> {
        return Err(String::from("Uncoverred exception!"));
    }
}
