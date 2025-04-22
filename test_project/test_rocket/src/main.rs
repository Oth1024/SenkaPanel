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

#[tokio::main]
async fn main() {
    initialize();
    // let test_vec = vec![3];
    // let test_config = TestConfig
    // {
    //     test_one: 2,
    //     test_two: 3,
    //     test_vec
    // };
    let config_path = "./target/test.toml";
    // let result = ConfigTool::set_config::<TestConfig>(String::from(config_path), &test_config).await;
    let result = ConfigTool::get_config::<TestConfig>(String::from(config_path)).await.unwrap();
    print!("result: test1:{}", result.test_one);
}

fn initialize() {
    initialize_logger();
}

fn initialize_logger() {
    LOG.set_console(false)
    .set_level(LEVEL::Trace)
    .set_format(Format::Time | Format::LevelFlag | Format::ShortFileName)
    .set_formatter("[{level}] {time} {file}: {message}");
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TestConfig {
    pub test_one :u32,
    pub test_two: u32,
    pub test_vec: Vec<u32>
}