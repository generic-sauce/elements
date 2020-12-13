use crate::prelude::*;

pub struct NativeSocketBackend(UdpSocket);

impl NativeSocketBackend {
	pub fn new(server_ip: &str, port: u16) -> Self {
		let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, port)).expect("Could not connect to server");

		let mut socket = NativeSocketBackend(socket);

		// this only happens on native!
		socket.send(&Init::Init).unwrap();

		socket
	}

	pub fn is_open(&self) -> bool { true }

	pub fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		send_packet(&mut self.0, packet)
	}

	pub fn try_recv<P: Packet>(&mut self) -> Option<P> {
		recv_packet(&mut self.0)
			.map(|(x, _)| x)
	}
}
