use crate::prelude::*;

#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

#[cfg(feature = "web-client")] mod web;
#[cfg(feature = "web-client")] pub use web::*;

pub trait SocketBackend {
	fn new(server_ip: &str, port: u16) -> Self;
	fn is_open(&self) -> bool;
	fn send(&mut self, packet: &impl Packet) -> std::io::Result<()>;  // TODO: use better Result-Type
	fn tick<P: Packet>(&mut self) -> Option<P>;
}