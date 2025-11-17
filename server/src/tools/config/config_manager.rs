use std::{
    any::{Any, type_name},
    collections::HashMap,
    fs::create_dir_all,
    path::Path,
    sync::Arc,
};
use tokio::fs::{read_to_string, write};

use crate::{
    common_definitions::senka_error::{SenkaError, SenkaErrorCode},
    tools::consts::DEFAULT_CONFIG_DIRECTORY,
};

pub trait Config: Any + Send + Sync {}

pub struct ConfigManager {
    configs: HashMap<&'static str, Arc<dyn Config>>,
}

impl ConfigManager {
    /// 获取配置
    pub fn get_config<T: Config + 'static>(&self) -> Option<&T> {
        let config_name = type_name::<T>();
        match self.configs.get(config_name) {
            Some(config) => (config.as_ref() as &dyn Any).downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn update_config_cache<T: Config + 'static>(&mut self, new_config: Arc<T>) {
        let config_name = type_name::<T>();
        self.configs.insert(config_name, new_config);
    }

    pub async fn update_config_to_disk<T: Config + 'static>(
        new_config_string: String,
    ) -> Result<(), SenkaError> {
        let config_name = type_name::<T>();
        let config_dir = String::from(DEFAULT_CONFIG_DIRECTORY);
        let mut config_path = config_dir.clone();
        config_path.push_str(config_name);
        config_path.push_str(".conf");
        match create_dir_all(&config_dir) {
            Err(_) => Err(SenkaError::new(
                SenkaErrorCode::Unexpected,
                format!("Cannot create config folder:{}", config_dir),
            )),
            Ok(_) => match write(config_path, new_config_string).await {
                Ok(_) => Ok(()),
                Err(error) => Err(SenkaError::new(
                    SenkaErrorCode::Inner,
                    format!(
                        "Cannot update config[{}] to disk, message:{}",
                        config_name,
                        error.to_string()
                    ),
                )),
            },
        }
    }

    pub async fn read_config_string_from_disk<T: Config + 'static>() -> Result<String, SenkaError> {
        let config_name = type_name::<T>();
        let config_dir = String::from(DEFAULT_CONFIG_DIRECTORY);
        let mut config_path = config_dir.clone();
        config_path.push_str(config_name);
        config_path.push_str(".conf");
        let config_path_entity = Path::new(&config_path);
        match !config_path_entity.exists() || config_path_entity.is_dir() {
            true => Err(SenkaError::new(
                SenkaErrorCode::Arguement,
                format!("Config path not exists:{}", config_path),
            )),
            false => match read_to_string(&config_dir).await {
                Ok(content) => Ok(content),
                Err(error) => Err(SenkaError::new(
                    SenkaErrorCode::Inner,
                    format!("Read file failed,message:{}", error.to_string()),
                )),
            },
        }
    }
}
