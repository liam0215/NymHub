#![allow(dead_code, unused_variables)]

use crate::messages;
use log::info;
use nym_sdk::mixnet;

type SendResult = Result<(), ()>;
type Address = mixnet::Recipient;

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
            .send_str(self.hub_address, &msg.to_string())
            .await;
        info!("successfully sent {} to {}", &msg, &self.hub_address);
        Ok(())
    }
}
