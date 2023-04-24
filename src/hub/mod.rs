#![allow(dead_code, unused_variables)]

pub struct Hub{}


impl Hub {
    pub fn init() -> Self {
        Hub{}
    }
    pub fn blocking_receive(&self) -> ! {
        panic!();
    }
}
