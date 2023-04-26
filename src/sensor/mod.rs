#![allow(dead_code, unused_variables)]

use crate::common::Address;
use crate::messages;
use log::info;
use nym_sdk::mixnet;

type SendResult = Result<(), ()>;

pub struct Sensor {
    client: mixnet::MixnetClient,
    hub_address: Address,
}

impl Sensor {
    pub async fn init_from_str(address_str: &str) -> Self {
        let address = mixnet::Recipient::try_from_base58_string(String::from(address_str)).unwrap();
        Self::init(address).await
    }
    pub async fn init(address: Address) -> Self {
        info!(
            "creating sensor and init mixnet client, pointing at: {}",
            &address
        );
        Self {
            client: mixnet::MixnetClient::connect_new().await.unwrap(),
            hub_address: address,
        }
    }

    pub async fn send_and_listen(&mut self, msg: messages::Message) -> SendResult {
        self.send(msg).await?;
        info!("waiting for message");
        if let Some(received) = self.client.wait_for_messages().await {
            for r in received {
                info!("Received: {}", String::from_utf8_lossy(&r.message));
            }
        }

        self.client.disconnect().await;

        Ok(())
    }

    async fn send(&self, msg: messages::Message) -> SendResult {
        info!("attempting to send {} to {}", &msg, &self.hub_address);
        self.client
            .send_str(self.hub_address, &serde_json::to_string(&msg).unwrap())
            .await;
        info!("successfully sent {} to {}", &msg, &self.hub_address);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::common::testing;
    use log::Level;
    extern crate testing_logger;

    use super::*;

    async fn get_sensor() -> Sensor {
        let address = get_test_address();
        Sensor::init_from_str(&address).await
    }

    fn validate_logs(expected_logs: &[(&str, Level)]) {
        let target = "iot_hub::sensor".to_string();
        testing::validate_logs(target, expected_logs);
    }

    fn get_test_address() -> String {
        "5Gu1gnu9uWLfKyHmSQFZU1vgkpeTxWk6VtLKH4eW9h3q.6o48hSHUEsY7qeKwQmD3ZQQ84esUD27QRQwCkYMes1w1@3ojQD6V7skM1bSXJX7fVQvscjmcgptzdixQEaAha2ixh".to_string()
    }

    #[tokio::test]
    async fn test_init_sensor() {
        testing::before_each();

        let sensor = get_sensor().await;

        assert!(sensor.client.identity().to_base58_string().len() != 0);
        let ex_log_msg = format!(
            "creating sensor and init mixnet client, pointing at: {}",
            &get_test_address()
        );
        let expected_logs = [(ex_log_msg.as_str(), Level::Info)];

        validate_logs(&expected_logs);
    }
}
