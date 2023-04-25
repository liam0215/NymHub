mod command;
mod hub;
mod logging;
mod messages;
mod parser;
mod sensor;

use log::{error, info};
use std::env;

#[tokio::main]
async fn main() {
    logging::init_logger();

    match match_arg_or_err(1, "Not enough args provided").as_str() {
        "hub" => {
            info!("starting up hub...");
            let hub = hub::Hub::init();
            hub.blocking_receive().await;
        }
        "sensor" => {
            info!("starting up sensor...");
            let address_str = match_arg_or_err(2, "no hub address provided");
            let msg = messages::Message::default();
            let mut sensor = sensor::Sensor::init_from_str(&address_str).await;
            sensor.send_and_listen(msg).await.unwrap();
        }
        _ => {
            println!("Invalid argument. Please use 'hub' or 'sensor'");
            error!("Invalid argument");
        }
    }
}

fn match_arg_or_err(index: usize, err_msg: &str) -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < index + 1 {
        error!("{}", err_msg);
        panic!("{}", err_msg);
    }

    args[index].to_string()
}
