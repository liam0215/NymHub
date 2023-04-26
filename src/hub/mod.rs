#![allow(dead_code, unused_variables)]
use crate::common::Address;
use log::info;
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
            .on_messages(|msg| info!("Received: {}", String::from_utf8_lossy(&msg.message)))
            .await;

        panic!();
    }
}

#[cfg(test)]
mod test {
    use log::Level;
    extern crate testing_logger;

    use super::*;

    fn before_each() {
        testing_logger::setup();
    }

    async fn get_hub() -> Hub {
        Hub::init().await
    }

    fn validate_logs(expected_logs: &[(&str, Level)]) {
        testing_logger::validate(|all_logs| {
            let target = "iot_hub::hub".to_string();

            let captured_logs: Vec<&testing_logger::CapturedLog> =
                all_logs.iter().filter(|x| x.target == target).collect();

            assert_eq!(captured_logs.len(), expected_logs.len());

            captured_logs
                .iter()
                .enumerate()
                .map(|(i, log)| (log.body.as_str(), log.level, i))
                .zip(
                    expected_logs
                        .iter()
                        .enumerate()
                        .map(|(i, ex)| (ex.0, ex.1, i)),
                )
                .for_each(|(log_tup, expected_tup)| assert_eq!(log_tup, expected_tup.clone()));
        });
    }

    #[tokio::test]
    async fn test_init_hub() {
        before_each();
        let hub = get_hub().await;
        dbg!(hub.client.identity().to_base58_string());

        assert!(hub.client.identity().to_base58_string().len() != 0);

        let expected_logs = [
            ("starting up nym client...", Level::Info),
            ("established connection with client with hub", Level::Info),
        ];

        validate_logs(&expected_logs);
    }
}
