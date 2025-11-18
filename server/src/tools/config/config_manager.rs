use serde::{Serialize, de::DeserializeOwned};
use std::{
    any::{Any, type_name},
    collections::HashMap,
    fs::create_dir_all,
    path::Path,
    sync::Arc,
};
use tokio::{
    fs::{read_to_string, write},
    sync::RwLock,
};

use crate::{
    common_definitions::senka_error::{SenkaError, SenkaErrorCode},
    tools::consts::DEFAULT_CONFIG_DIRECTORY,
};

pub trait SenkaConfig: Any + Default {}

pub struct ConfigManager {
    configs: RwLock<HashMap<&'static str, Arc<dyn Any>>>,
}

impl ConfigManager {
    pub async fn get_config<T: SenkaConfig + 'static + Serialize + DeserializeOwned>(
        &self,
    ) -> Result<Arc<T>, SenkaError> {
        match self.get_config_from_cache::<T>().await {
            Ok(config) => Ok(config),
            Err(_) => match ConfigManager::read_config_string_from_disk::<T>().await {
                Ok(config_string) => match toml::from_str::<T>(config_string.as_str()) {
                    Ok(config_from_disk) => {
                        let config_arc = Arc::new(config_from_disk);
                        let config_name = type_name::<T>();
                        self.configs.write().await.insert(config_name, config_arc);
                        self.get_config_from_cache::<T>().await
                    }
                    Err(_) => Err(SenkaError::new(
                        SenkaErrorCode::Format,
                        String::from("Error toml format"),
                    )),
                },
                Err(_) => {
                    let default_config = Arc::new(T::default());
                    let config_name = type_name::<T>();

                    // restore config to disk
                    match toml::to_string::<T>(default_config.as_ref()) {
                        Ok(config_string) => {
                            match ConfigManager::update_config_to_disk::<T>(config_string).await {
                                Ok(_) => {
                                    self.configs
                                        .write()
                                        .await
                                        .insert(config_name, default_config);
                                    self.get_config_from_cache::<T>().await
                                }
                                Err(_) => Err(SenkaError::new(
                                    SenkaErrorCode::Format,
                                    String::from("Write default config to disk failed"),
                                )),
                            }
                        }
                        Err(_) => Err(SenkaError::new(
                            SenkaErrorCode::Format,
                            String::from("Parse config to string failed"),
                        )),
                    }
                }
            },
        }
    }

    /// 从内存中读取之前存储的配置
    pub async fn get_config_from_cache<T: Any + 'static>(&self) -> Result<Arc<T>, SenkaError> {
        let config_name = type_name::<T>();
        match self.configs.read().await.get(config_name) {
            Some(config) => match config.downcast_ref::<Arc<T>>() {
                Some(config_arc) => Ok(Arc::clone(config_arc)),
                None => Err(SenkaError::new(
                    SenkaErrorCode::Arguement,
                    format!("Cannot cast to type[{}]", config_name),
                )),
            },
            None => Err(SenkaError::new(
                SenkaErrorCode::Arguement,
                format!("Cannot get config by type[{}]", config_name),
            )),
        }
    }

    /// 从配置文件中读取配置
    pub async fn read_config_string_from_disk<T: Any + 'static>() -> Result<String, SenkaError> {
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

    /// 更新存储在内存中的配置
    pub async fn update_config_cache<T: Any + 'static>(&mut self, new_config: Arc<T>) {
        let config_name = type_name::<T>();
        self.configs.write().await.insert(config_name, new_config);
    }

    /// 更新配置到文件中
    pub async fn update_config_to_disk<T: Any + 'static>(
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
}
