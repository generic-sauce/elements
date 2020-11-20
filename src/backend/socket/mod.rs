use crate::prelude::*;

#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

#[cfg(feature = "web-client")] mod web;
#[cfg(feature = "web-client")] pub use web::*;

pub trait SocketBackend {
	fn new(server_ip: &str) -> Self;
	fn is_open(&mut self) -> bool;
	fn send(&mut self, packet: &impl Packet);
	fn try_recv<P: Packet>(&mut self) -> Option<P>;
}