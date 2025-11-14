use std::{any::{Any, type_name}, collections::HashMap, sync::Arc};

use dyn_serde::Serializer;
use tokio::fs::File;

use crate::common_definitions::senka_error::{SenkaError, SenkaErrorCode};

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
            None => None
        }
    }

    pub fn update_config<T: Config + 'static>(&mut self, new_config: Arc<T>) {
        let result = export_config_to_disk(new_config.as_ref());
        if let Ok(_) = result {
            let config_name = type_name::<T>();
            self.configs.insert(config_name, new_config);   
        }
    }
}

/// 异步更新配置
fn export_config_to_disk<T: Config + 'static>(config: &T) -> Result<(), SenkaError> {
    let mut buf = String::new();
    let mut toml_serializer = toml::Serializer::new(&mut buf);
    let mut inplace_serializer = <dyn Serializer>::new(&mut toml_serializer);
}