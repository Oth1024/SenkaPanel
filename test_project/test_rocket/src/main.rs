pub mod utilities;

use tklog::{Format, LEVEL, LOG, MODE, sync::Logger};

#[macro_use]
extern crate rocket;

// #[launch]
// fn startup() -> _ {
//     initialize();
//     rocket::build()
// }

fn main() {}

fn initialize() {

}

fn initialize_logger() {
    LOG.set_console(false)
    .set_level(LEVEL::Trace)
    .set_format(Format::Time | Format::LevelFlag | Format::ShortFileName)
    .set_formatter("[{level}] {time} {file}: {message}");
}