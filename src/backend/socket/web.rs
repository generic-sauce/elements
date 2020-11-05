use crate::prelude::*;

pub struct WebSocketBackend;

impl SocketBackend for WebSocketBackend {
	fn new(server_ip: &str) -> Self {
		unimplemented!()
	}

	fn send(&mut self, packet: &impl Packet) {
		unimplemented!()
	}

	fn try_recv<P: Packet>(&mut self) -> Option<P> {
	}
}
