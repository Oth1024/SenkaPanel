pub mod utilities;

use std::fmt::Display;

use tklog::{Format, LEVEL, LOG, MODE, sync::Logger};
use utilities::config_tool::ConfigTool;
use serde;

#[macro_use]
extern crate rocket;

// #[launch]
// fn startup() -> _ {
//     initialize();
//     rocket::build()
// }

#[launch]
async fn startup() -> _ {
    initialize();
    let config_path = "./target/test.toml";
    let test_config = TestConfig { test_one: 1, test_two: 2, test_vec: vec![1, 2, 4] };
    let result = ConfigTool::set_config::<TestConfig>(String::from(config_path), &test_config).await;
    let result = ConfigTool::get_config::<TestConfig>(String::from(config_path)).await.unwrap();
    print!("result: test1:{}", result.test_one);
    rocket::build()
}

fn initialize() {
    initialize_logger();
}

fn initialize_logger() {
    LOG.set_console(false)
    .set_level(LEVEL::Trace)
    .set_format(Format::Time | Format::LevelFlag | Format::ShortFileName)
    .set_formatter("[{level}] {time} {file}: {message}")
    .set_cutmode_by_size("./target/TestProject.log", 1024, 10, true);
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestConfig {
    pub test_one :u32,
    pub test_two: u32,
    pub test_vec: Vec<u32>
}