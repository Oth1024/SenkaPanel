use std::{
    collections::HashMap, fmt::format, ops::Add, path::{Path, PathBuf}, sync::{Arc, Mutex}
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::fs::{self, File};

pub struct ConfigTool;

impl ConfigTool {

    pub async fn get_config<T>(config_name: &String, config_path: &String) -> Result<T, String> 
    where T: DeserializeOwned
    {

    }

    pub async  fn set_config<T>(config_name: &String, config_path: &String, new_config_object: &T) -> Result<(), String> {

    }
}