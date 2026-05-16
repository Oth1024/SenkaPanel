use std::{collections::HashMap, fs::metadata, os::windows::fs::MetadataExt, path::Path, sync::Mutex};
use chrono::{Duration, TimeZone, Utc};
use notify::{Event, ReadDirectoryChangesWatcher, Watcher, recommended_watcher};
use crate::def::file_info::{FileInfo, FileType};
use os_info::{Type, get};
use once_cell::sync::Lazy;
use tokio::sync::mpsc;

pub mod def;

pub static FS_WATCHER: Lazy<Mutex<HashMap<String, ReadDirectoryChangesWatcher>>> =
    Lazy::new(||Mutex::new(HashMap::<String, ReadDirectoryChangesWatcher>::new()));

static FS_WATCHER_EVENT_HANDLER: Lazy<Mutex<HashMap<String, HashMap<String, Box<dyn Fn(&Event) + Send + Sync>>>>> = 
    Lazy::new(||Mutex::new(HashMap::new()));

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

/// 添加对一个路径的订阅
pub async fn monitor_fs(directory: &str) {
    let (tx, mut rx) = mpsc::channel::<Result<Event, _>>(100);
    let mut watcher = recommended_watcher(move |res: Result<Event, _>| {
        let _ = tx.try_send(res);
    }).unwrap();

    let path = Path::new(directory);
    let _ = watcher.watch(path, notify::RecursiveMode::NonRecursive);

    // 添加到订阅表方便取消订阅
    FS_WATCHER.lock().unwrap().insert(String::from(directory), watcher);

    while let Some(res) = rx.recv().await {
        if let Ok(event) = res {
            on_fs_changed_event(directory, event);
        }
    }
}

/// 取消并移除一个已经订阅的路径
pub fn cancel_monitor_fs(directory: &str) {
    if let Ok(mut watchers) = FS_WATCHER.lock() {
        if watchers.contains_key(directory) {
            // 取消订阅
            let path = Path::new(directory);
            watchers.get_mut(directory).unwrap().unwatch(path);
            // 删除
            watchers.remove(directory);
        }
    }
}

pub fn subscribe_fs_changed_event<F>(directory: &str, func_id: &str, func: F) -> Result<(), ()>
    where F: Fn(&Event) + Send + Sync + 'static {
    let mut hashmap = FS_WATCHER_EVENT_HANDLER.lock().unwrap();
    if !hashmap.contains_key(directory) {
        let mut inner_hashmap = HashMap::<String, Box<dyn Fn(&Event) + Send + Sync>>::new();
        inner_hashmap.insert(String::from(func_id), Box::new(func));
        hashmap.insert(String::from(directory), inner_hashmap);
        return Ok(());
    }
    else {
        let inner_hashmap = hashmap.get_mut(directory).unwrap();
        if inner_hashmap.contains_key(func_id) {
            return Err(());
        }
        else {
            inner_hashmap.insert(String::from(func_id), Box::new(func));
            return Ok(());
        }
    }
}

pub fn unsubscribe_fs_changed_event(directory: &str, func_id: &str) {
    let mut hashmap = FS_WATCHER_EVENT_HANDLER.lock().unwrap();
    if hashmap.contains_key(directory) {
        if hashmap.contains_key(func_id) {
            hashmap.remove(func_id);
        }
    }
}

// 处理事件
fn on_fs_changed_event(directory: &str, event: Event) {
    let hashmap = FS_WATCHER_EVENT_HANDLER.lock().unwrap();
    if hashmap.contains_key(directory) {
        let inner_hashmap = hashmap.get(directory).unwrap();
        let funcs = inner_hashmap.values().enumerate();
        for func in funcs {
            let arg = &event;
            func.1(arg);
        }
    }
}