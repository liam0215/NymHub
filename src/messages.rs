#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
pub struct Message {
    header: Header,
    payload: Payload,
}

type Payload = String;
type Key = String;

#[derive(Debug)]
struct Header {
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
            payload: String::from("testpayload"),
        }
    }
}
