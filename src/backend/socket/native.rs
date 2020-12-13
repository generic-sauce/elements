use crate::prelude::*;

pub struct NativeSocketBackend(UdpSocket);

impl SocketBackend for NativeSocketBackend {
	fn new(server_ip: &str, port: u16) -> Self {
		let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, port)).expect("Could not connect to server");

		let mut socket = NativeSocketBackend(socket);

		// this only happens on native!
		socket.send(&Init::Init).unwrap();

		socket
	}

	fn is_open(&self) -> bool { true }

	fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		send_packet(&mut self.0, packet)
	}

	fn try_recv<P: Packet>(&mut self) -> Option<P> {
		recv_packet(&mut self.0)
			.map(|(x, _)| x)
	}
}
