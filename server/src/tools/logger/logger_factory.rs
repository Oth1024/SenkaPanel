use std::{fmt::Display, fs::create_dir_all, path::Path};

use log::LevelFilter;
use log4rs::{Config, append::{console::ConsoleAppender, rolling_file::{RollingFileAppender, policy::{Policy, compound::{CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger}}}}, config::{Appender, Root}, encode::pattern::{self, PatternEncoder}};
use rocket::config::LogLevel;

// logger 定义
#[derive(Debug)]
pub struct Logger {
    pub crate_name: &'static str,
    pub logger_name: &'static str,
}

impl Logger {
    fn new(crate_name: &'static str, logger_name: &'static str) -> Self {
        Logger { crate_name, logger_name }
    }

    fn target(&self) -> String {
        format!("{}-{}", self.crate_name, self.logger_name)
    }

    pub fn trace(&self, message: impl Display) {
        log::trace!(target: self.target().as_str(), "{}", message)
    }

    pub fn debug(&self, message: impl Display) {
        log::debug!(target: self.target().as_str(), "{}", message)
    }

    pub fn info(&self, message: impl Display) {
        log::info!(target: self.target().as_str(), "{}", message)
    }

    pub fn warn(&self, message: impl Display) {
        log::warn!(target: self.target().as_str(), "{}", message)
    }

    pub fn error(&self, message: impl Display) {
        log::error!(target: self.target().as_str(), "{}", message)
    }
}

// logger factory
pub fn get_logger(crate_name: &'static str, logger_name: &'static str) -> Logger {
    Logger::new(crate_name, logger_name)
}

/// 在使用Logger前需要调用此方法
/// # 参数
/// log_level:最低日志等级,
/// console_output:开启终端显示,
/// file_output:开启日志文件,
/// file_output_dir:日志文件路径,
/// file_size:单个日志文件最大大小,
/// max_file_count:最大日志数量
pub fn initialize_logger(log_level: LogLevel, console_output: bool, file_output: bool, file_output_dir: &'static str, file_size: u64, max_file_count: u32) {
    let mut appenders = Vec::new(); 

    let encoder = PatternEncoder::new("[d(%Y-%m-%d %H:%M:%S%.3f)][%t][%l][%i]%m");

    if console_output {
        let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .build();
        appenders.push(Appender::builder().build("console", Box::new(console_appender)));
    }
    if file_output {
        let log_dir = Path::new(file_output_dir);
        // 创建目录
        if !log_dir.exists() {
            create_dir_all(log_dir).unwrap();
        }

        let file_output_path = log_dir.join("SenkaPanelServer.log");
        let mut file_rolling_pattern = String::from(file_output_dir);
        file_rolling_pattern.push_str("/SenkaPanelServer_{}.log");
        // 设置文件大小触发器
        let size_trigger = SizeTrigger::new(file_size);
        // 设置文件窗口数量
        let fixed_window_roller = FixedWindowRoller::builder().build(file_rolling_pattern.as_str(), max_file_count).unwrap();
        let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
        let rolling_file_appender = RollingFileAppender::builder()
        .encoder(Box::new(encoder))
        .build(file_output_path, Box::new(policy))
        .unwrap();
        appenders.push(Appender::builder()
            .build("file", Box::new(rolling_file_appender)));
    } 
    let config = Config::builder()
    .appenders(appenders)
    .build(Root::builder()
    .appenders(
        if console_output && file_output { vec!["console", "file"] }
        else if console_output { vec!["console"] }
        else if file_output { vec!["file"] }
        else { Vec::new() })
    .build(LevelFilter::from(log_level)))
    .unwrap();

    log4rs::init_config(config);
}