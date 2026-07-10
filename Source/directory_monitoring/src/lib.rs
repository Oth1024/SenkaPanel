use std::{
    collections::HashMap, fs::metadata, os::windows::fs::MetadataExt, path::Path,
    sync::{atomic::{AtomicUsize, Ordering}, Mutex},
};
use chrono::{Duration, TimeZone, Utc};
use notify::{Event, ReadDirectoryChangesWatcher, Watcher, recommended_watcher};
use crate::def::file_info::{FileInfo, FileType};
use os_info::{Type, get};
use common::senka_error::SenkaError;
use once_cell::sync::Lazy;
use tokio::sync::broadcast;

pub mod def;

/// 订阅状态：持有 broadcast sender、watcher、以及订阅者计数
pub struct SubscriptionState {
    sender: broadcast::Sender<Event>,
    watcher: ReadDirectoryChangesWatcher,
    subscriber_count: AtomicUsize,
}

static FS_WATCHER: Lazy<Mutex<HashMap<String, SubscriptionState>>> =
    Lazy::new(|| Mutex::new(HashMap::<String, SubscriptionState>::new()));

pub fn get_fs_watcher() -> &'static Mutex<HashMap<String, SubscriptionState>> {
    &FS_WATCHER
}

// 对外方法
pub fn show_fs_on_dir(directory: &str) -> Vec<FileInfo> {
    let mut result = Vec::<FileInfo>::new();
    let path = Path::new(directory);
    if let Ok(files) = path.read_dir() {
        for file in files.enumerate() {
            if let Ok(dir_entry) = file.1 {
                let file_path = dir_entry.path();
                // Get metadata and translate into file info
                if let Ok(meta_data) = metadata(&file_path) {
                    let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());
                    let last_modified_time_in_100_nanos = meta_data.last_write_time() as i64;
                    let last_modified_time = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap()
                        + Duration::microseconds(last_modified_time_in_100_nanos / 10);
                    let mut file_type = FileType::Unspecified;
                    let file_size = meta_data.file_size();
                    if meta_data.is_dir() { file_type = FileType::Directory; }
                    else if meta_data.is_file() { file_type = FileType::File; }
                    else if meta_data.is_symlink() { file_type = FileType::Link; }
                    let file_type_clone = file_type.clone();

                    let mut file_info = FileInfo {
                        file_name,
                        file_type,
                        last_modified_time,
                        file_size,
                        file_mod: None,
                        owner: None,
                        group: None
                    };

                    let platform = get().os_type();
                    // For Windows
                    // owner and user mod is not available attribute
                    if platform == Type::Windows {
                        // do nothing
                    }
                    // For Linux
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionExt;
                        if platform == Type::Ubuntu
                        || platform == Type::Arch
                        || platform == Type::CentOS
                        || platform == Type::ALTLinux
                        || platform == Type::Debian {
                            let mode = meta_data.permissions();
                            let file_mode = FileMode::from_u32(file_type_clone, mode.mode());
                            file_info.file_mod = file_mode;
                        }
                    }

                    result.push(file_info);
                }
            }
        }
    }
    result
}

/// 添加对一个路径的订阅，返回 broadcast::Receiver。
/// 如果该路径已被订阅，则递增引用计数并返回新的 Receiver（clone），避免重复创建 watcher。
pub fn subscribe_fs(directory: &str) -> Result<broadcast::Receiver<Event>, SenkaError> {
    let mut map = get_fs_watcher().lock().unwrap();

    // 已有订阅：递增计数，返回 clone receiver
    if let Some(state) = map.get(directory) {
        state.subscriber_count.fetch_add(1, Ordering::SeqCst);
        return Ok(state.sender.subscribe());
    }

    // 新建订阅
    let (tx, _) = broadcast::channel::<Event>(100);
    let tx_clone = tx.clone();
    let mut watcher = recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => { let _ = tx_clone.send(event); },
            // TODO: 记录到Logger
            Err(e) => { eprintln!("[subscribe_fs] watch error: {:?}", e); },
        }
    }).unwrap();

    let path = Path::new(directory);
    let _ = watcher.watch(path, notify::RecursiveMode::NonRecursive);

    let rx = tx.subscribe();

    map.insert(String::from(directory), SubscriptionState {
        sender: tx,
        watcher,
        subscriber_count: AtomicUsize::new(1),
    });

    Ok(rx)
}

/// 取消对一个路径的订阅。递减引用计数，仅当无其他订阅者时才停止 watch 并移除。
pub fn unsubscribe_fs(directory: &str) {
    if let Ok(mut map) = get_fs_watcher().lock() {
        if let Some(state) = map.get_mut(directory) {
            let prev = state.subscriber_count.fetch_sub(1, Ordering::SeqCst);
            if prev == 1 {
                // 最后一个订阅者，取消 watch 并移除
                let path = Path::new(directory);
                state.watcher.unwatch(path).ok();
                map.remove(directory);
            }
        }
    }
}