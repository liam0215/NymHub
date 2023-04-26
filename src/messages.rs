#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fmt;

type Payload = String;
type Key = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub_key: Key,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.payload)
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            header: Header {
                pub_key: String::from("testpub"),
            },
            payload: String::from("user1 create ac_on bool true\n"),
        }
    }
}
