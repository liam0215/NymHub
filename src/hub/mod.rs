#![allow(dead_code, unused_variables)]
use nym_sdk::mixnet;
use log::{info};

pub struct Hub{}


impl Hub {
    pub fn init() -> Self {
        Hub{}
    }
    pub async fn blocking_receive(&self) -> ! {

        info!("starting up nym client...");
        // Passing no config makes the client fire up an ephemeral session and figure shit out on its own
        let mut client = mixnet::MixnetClient::connect_new().await.unwrap();

        // Be able to get our client address
        let our_address = client.nym_address();
        println!("Our client nym address is: {our_address}");

        // Send a message throught the mixnet to ourselves
        client.send_str(*our_address, "hello there").await;

        println!("Waiting for message (ctrl-c to exit)");
        client
            .on_messages(|msg| println!("Received: {}", String::from_utf8_lossy(&msg.message)))
        .await;

        panic!();
    }
}
