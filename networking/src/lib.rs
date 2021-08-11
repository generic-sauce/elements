extern crate serde_derive;

mod constants;
mod packets;
mod socket_backend;

#[cfg(not(target_arch = "wasm32"))] mod timed_loop;
#[cfg(not(target_arch = "wasm32"))] mod udp;
#[cfg(not(target_arch = "wasm32"))] mod peer;
#[cfg(not(target_arch = "wasm32"))] mod fps_timer;

pub mod prelude;
pub use prelude::*;

pub trait Packet: Serialize + DeserializeOwned + Clone {}

impl Packet for () {}

pub fn ser<P: Serialize>(p: &P) -> Result<Vec<u8>, SocketErr> {
	Ok(serialize(p)?)
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> Result<P, SocketErr> {
	Ok(deserialize(bytes)?) // TODO: this crashes if server packet is malformed
}