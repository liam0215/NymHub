#![allow(dead_code)]

pub struct Message {
    header: Header,
    payload: Payload,
}

type Payload = String;
type Key = String;
struct Header {
    pub_key: Key
}
