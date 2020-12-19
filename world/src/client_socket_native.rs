use networking::prelude::*;

// TODO: move
pub struct ClientSocketNative {
	socket: UdpSocket,
	last_sent_time: Instant,
}

impl ClientSocketNative {
	pub fn new(server_ip: &str, port: u16) -> Self {
		let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, port)).expect("Could not connect to server");

		let mut socket_backend = ClientSocketNative {
			socket,
			last_sent_time: Instant::now(),
		};

		send_packet(&mut socket_backend.socket, &NativeCSPacket::<()>::Heartbeat).unwrap();

		socket_backend
	}

	pub fn is_open(&self) -> bool { true }

	pub fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		self.last_sent_time = Instant::now();
		send_packet(&mut self.socket, &NativeCSPacket::Payload(packet.clone())) // TODO: maybe fix this clone, see https://serde.rs/lifetimes.html
	}

	pub fn tick<P: Packet>(&mut self) -> Option<P> {
		if self.last_sent_time.elapsed().as_secs() >= HEARTBEAT_TIME_SECS as u64 {
			self.last_sent_time = Instant::now();
			send_packet(&mut self.socket, &NativeCSPacket::<()>::Heartbeat).unwrap();
		}

		recv_packet::<P>(&mut self.socket)
			.map(|(x, _)| x)
	}
}
