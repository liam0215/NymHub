#![allow(dead_code, unused_variables)]

use crate::messages;

pub struct Sensor {}

type SendResult = Result<(), ()>;

impl Sensor {
    pub fn send(&self, msg: messages::Message) -> SendResult {
        Ok(())
    }
}
