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
    use super::*;

    #[test]
    fn test_parse_says() {
        const MSG: &str = "user1 says ( or ( < temp 70 ) ac_on )\n";
        let _pairs = NymHubDSL::parse(Rule::main, MSG).unwrap_or_else(|e| panic!("{}", e));
    }
}
