pub fn init_logger() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
}
