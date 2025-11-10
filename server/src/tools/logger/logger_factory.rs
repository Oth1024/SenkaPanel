use chrono::Local;
use log::{Level, Record};
use std::io::Write;
use std::thread;

pub struct Logger {
    crate_name: &'static str,
    logger_name: &'static str,
}

impl Logger {
    fn new(logger_name: &'static str) -> Self {
        let crate_name = env!("CARGO_PKG_NAME");
        Self {
            crate_name,
            logger_name,
        }
    }

    fn log(&self, level: Level, message: impl std::fmt::Display) {
        let message = message.to_string();
        log::logger().log(
            &Record::builder()
                .level(level)
                .target(format!("{}-{}", self.crate_name, self.logger_name).as_str())
                .module_path(Some(self.crate_name))
                .args(format_args!("{}", message))
                .build(),
        );
    }

    pub fn trace(&self, message: impl std::fmt::Display) {
        self.log(Level::Trace, message);
    }

    pub fn debug(&self, message: impl std::fmt::Display) {
        self.log(Level::Debug, message);
    }

    pub fn info(&self, message: impl std::fmt::Display) {
        self.log(Level::Info, message);
    }

    pub fn warn(&self, message: impl std::fmt::Display) {
        self.log(Level::Warn, message);
    }

    pub fn error(&self, message: impl std::fmt::Display) {
        self.log(Level::Error, message);
    }
}

/// Call this function before logger is used
pub fn initialize_logger() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let time_stamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let logger_name = record.target();
            let level = record.level().to_string();
            let thread_id = get_thread_id();
            let message = record.args();
            writeln!(
                buf,
                "[{}][{}][{}][{}]{}",
                time_stamp, logger_name, level, thread_id, message
            )
        })
        .init();
}

/// Get an instance of logger
/// # Example
/// let logger = get_logger("YOUR_LOGGER_NAME")
pub fn get_logger(logger_name: &'static str) -> Logger {
    Logger::new(logger_name)
}

fn get_thread_id() -> String {
    let thread_id = thread::current().id();
    format!("{:?}", thread_id)
        .replace("ThreadId(", "")
        .replace(")", "")
}

/// Get an instance of logger with name of current file
#[macro_export]
macro_rules! LOGGER {
    () => {
        get_logger(extract_filename(file!()))
    };
}

fn extract_filename(file_path: &str) -> &str {
    // 1. 拆分路径分隔符（兼容 Windows \ 和 Unix /），取最后一段（如 user.rs）
    let file_name_with_ext = file_path
        .split(|c: char| c == '/' || c == '\\')
        .last()
        .unwrap_or("unknown_file"); // 异常路径默认名

    // 2. 去掉 .rs 后缀（如果有）
    file_name_with_ext
        .rsplit_once('.')
        .map(|(name, _ext)| name) // 拆分 文件名.rs → 取文件名部分
        .unwrap_or(file_name_with_ext) // 无后缀时直接用原名称
}
