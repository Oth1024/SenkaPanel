use bytes::{Buf, Bytes};
use serde::{Serialize, de::DeserializeOwned};
use tklog::info;
use std::path::Path;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

pub struct ConfigTool;

impl ConfigTool {
    pub async fn get_config<T>(config_path: String) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let get_file = fs::read_to_string(config_path).await;
        if let Ok(open_file_success) = get_file {
            let serialize_file = toml::from_str::<T>(&open_file_success);
            if let Ok(config) = serialize_file {
                return Ok(config);
            }
            else if let Err(error_info) = serialize_file {
                error!("{}", error_info);
                return Err(String::from(error_info.to_string()));
            }
        }
        else if let Err(error_info) = get_file {
            error!("{}", error_info);
            return Err(String::from(error_info.to_string()));
        }
        let error_message = "Uncoverred exception!";
        error!("{}", error_message);
        return Err(String::from(error_message));
    }

    pub async fn set_config<T>(config_path: String, new_config_object: &T) -> Result<(), String>
    where
        T: Serialize,
    {
        let path = Path::new(&config_path);
        let dir = path.parent();
        if let Some(file_dir) = dir {
            if !file_dir.is_dir() {
                let try_create = fs::create_dir(file_dir).await;
                if let Err(create_failed) = try_create {
                    error!("{}", create_failed);
                    return Err(String::from(create_failed.to_string()));
                }
            }
            let config_object_to_string = toml::to_string(new_config_object);
            if let Ok(config_string) = config_object_to_string {
                let try_create_file = File::create(&config_path).await;
                if let Ok(mut file) = try_create_file {
                    let mut buf = Bytes::from(config_string.into_bytes());
                    while buf.has_remaining() {
                        let result = file.write_buf(&mut buf).await;
                        if let Err(write_err) = result {
                            error!("{}", write_err);
                            return Err(String::from(write_err.to_string()));
                        }
                    }
                    info!(format!("Save config to path:{} success!", config_path));
                    return Ok(());
                } 
                else if let Err(create_failed) = try_create_file {
                    error!("{}", create_failed);
                    return Err(String::from(create_failed.to_string()));
                }
            } 
            else if let Err(parse_failed) = config_object_to_string {
                error!("{}", parse_failed);
                return Err(String::from(parse_failed.to_string()));
            }
        } 
        else {
            let error_message = format!("Can not parse directory by path:{}!", config_path);
            error!("{}", error_message);
            return Err(String::from(error_message));
        }
        return Err(String::from("Uncoverred exception!"));
    }
}
