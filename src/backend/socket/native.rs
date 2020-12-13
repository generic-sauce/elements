use crate::prelude::*;

impl SocketBackend for NativeSocketBackend {
	fn new(server_ip: &str, port: u16) -> Self {
		NativeSocketBackend::new(server_ip, port)
	}

	fn is_open(&self) -> bool {
		NativeSocketBackend::is_open(self)
	}

	fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		NativeSocketBackend::send(self, packet)
	}

	fn try_recv<P: Packet>(&mut self) -> Option<P> {
		NativeSocketBackend::try_recv(self)
	}
}
