mod messages;
mod sensor;
mod logging;
mod parser;
mod hub;
mod command;

#[tokio::main]
async fn main() {
    logging::init_logger();
    let hub = hub::Hub::init();
    hub.blocking_receive().await;
}
