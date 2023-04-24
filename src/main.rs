use log::{info, warn, error};
mod logging;
mod parser;
mod command;

fn main() {
    logging::init_logger();
    println!("Hello, world!");
    info!("test1");
    warn!("test2");
    error!("test3");
}
