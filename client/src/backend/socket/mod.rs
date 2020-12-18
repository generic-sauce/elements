use crate::prelude::*;

pub trait SocketBackend {
	fn new(server_ip: &str, port: u16) -> Self;
	fn is_open(&self) -> bool;
	fn send(&mut self, packet: &impl Packet) -> std::io::Result<()>;  // TODO: use better Result-Type
	fn tick<P: Packet>(&mut self) -> Option<P>;
}
