use crate::prelude::*;

pub struct NativeSocketBackend(ClientSocketNative);

impl SocketBackend for NativeSocketBackend {
	fn new(server_ip: &str, port: u16) -> Self {
		NativeSocketBackend(ClientSocketNative::new(server_ip, port))
	}

	fn is_open(&self) -> bool {
		self.0.is_open()
	}

	fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		self.0.send(packet)
	}

	fn tick<P: Packet>(&mut self) -> Option<P> {
		self.0.tick()
	}
}
