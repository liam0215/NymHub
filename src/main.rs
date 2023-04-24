use log::{info, warn, error};
mod messages;
mod sensor;
mod logging;
mod parser;
mod hub;
mod command;

fn main() {
    logging::init_logger();
    println!("Hello, world!");
    info!("test1");
    warn!("test2");
    error!("test3");
}
