use crate::prelude::*;

const HEARTBEAT_TIME_SECS: u32 = 1;

pub struct NativeSocketBackend {
	socket: UdpSocket,
	last_sent_time: Instant,
}

impl NativeSocketBackend {
	pub fn new(server_ip: &str, port: u16) -> Self {
		let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, port)).expect("Could not connect to server");

		let mut socket_backend = NativeSocketBackend {
			socket,
			last_sent_time: Instant::now(),
		};

		send_packet(&mut socket_backend.socket, &NativeCSPacket::<()>::Heartbeat).unwrap();

		socket_backend
	}

	pub fn is_open(&self) -> bool { true }

	pub fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		send_packet(&mut self.socket, &NativeCSPacket::Payload(packet.clone()))
	}

	pub fn tick<P: Packet>(&mut self) -> Option<P> {
		if self.last_sent_time.elapsed().as_secs() >= HEARTBEAT_TIME_SECS as u64 {
			send_packet(&mut self.socket, &NativeCSPacket::<()>::Heartbeat).unwrap();
			self.last_sent_time = Instant::now();
		}

		recv_packet::<P>(&mut self.socket)
			.map(|(x, _)| x)
	}
}
