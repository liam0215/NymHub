#![allow(dead_code, unused_variables)]
use crate::common::Address;
use crate::messages::Message;
use crate::parser;
use log::{error, info};
use nym_sdk::mixnet;

pub struct Hub {
    client: mixnet::MixnetClient,
}

impl Hub {
    pub async fn init() -> Self {
        info!("starting up nym client...");
        let client = mixnet::MixnetClient::connect_new().await.unwrap();
        info!("established connection with client with hub");
        Self { client }
    }

    pub fn address(&self) -> Address {
        *self.client.nym_address()
    }

    pub async fn blocking_receive(&mut self) -> ! {
        info!("Waiting for message (ctrl-c to exit)");
        self.client
            .on_messages(|raw_msg| {
                let raw_msg_string = String::from_utf8_lossy(&raw_msg.message);
                info!("Received raw: {}", &raw_msg_string);

                let msg: Message = serde_json::from_str(&raw_msg_string).unwrap();
                info!("Received parsed: {:#?}", &msg);
                match parser::NymHubDSL::parse_msg(&msg.payload) {
                    Ok(cmds) => {
                        info!("successful parse");
                        cmds.iter().for_each(|cmd| cmd.execute());
                    }
                    Err(err) => {
                        error!("cannot parse, retting no-op error, input_str was: {}", &msg)
                    }
                }
            })
            .await;

        panic!();
    }
}

#[cfg(test)]
mod test {
    use crate::common::testing;
    use log::Level;
    extern crate testing_logger;

    use super::*;

    async fn get_hub() -> Hub {
        Hub::init().await
    }

    fn validate_logs(expected_logs: &[(&str, Level)]) {
        let target = "iot_hub::hub".to_string();
        testing::validate_logs(target, expected_logs);
    }

    #[tokio::test]
    async fn test_init_hub() {
        testing::before_each();
        let hub = get_hub().await;

        assert!(hub.client.identity().to_base58_string().len() != 0);
        let expected_logs = [
            ("starting up nym client...", Level::Info),
            ("established connection with client with hub", Level::Info),
        ];

        validate_logs(&expected_logs);
    }

    #[tokio::test]
    async fn test_get_address() {
        testing::before_each();
        let hub = get_hub().await;

        assert!(hub.address().to_string().len() != 0);
    }
}
