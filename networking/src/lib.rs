#[macro_use]
extern crate serde_derive;

mod constants;

mod packets;
pub mod prelude;

use serde::Serialize;
use serde::de::DeserializeOwned;
use bincode::{serialize, deserialize};

pub trait Packet: Serialize + DeserializeOwned + Clone {}

impl Packet for () {}

pub fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
