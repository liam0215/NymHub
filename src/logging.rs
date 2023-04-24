use log4rs;
use std::env;
use env_logger;

pub fn init_logger() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
}
