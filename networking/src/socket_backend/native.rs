use crate::prelude::*;

pub struct NativeSocketBackend {
	socket: UdpSocket,
	last_sent_time: Instant,
}

impl SocketBackend for NativeSocketBackend {
	fn new(server_ip: &str, port: u16) -> Result<Self, SocketErr> {
		let mut socket = UdpSocket::bind("0.0.0.0:0")?;
		socket.set_nonblocking(true)?;
		socket.connect((server_ip, port))?;

		let last_sent_time= Instant::now();
		send_packet(&mut socket, &NativeCSPacket::<()>::Heartbeat)?;

		Ok(Self {
			socket,
			last_sent_time,
		})
	}

	fn send(&mut self, packet: &impl Packet) -> Result<(), SocketErr> {
		self.last_sent_time = Instant::now();
		send_packet(&mut self.socket, &NativeCSPacket::Payload(packet.clone())) // TODO: maybe fix this clone, see https://serde.rs/lifetimes.html
	}

	fn tick(&mut self) {
		if self.last_sent_time.elapsed().as_secs() >= HEARTBEAT_TIME_SECS as u64 {
			if send_packet(&mut self.socket, &NativeCSPacket::<()>::Heartbeat).is_ok() {
				self.last_sent_time = Instant::now();
			}
		}
	}

	fn recv<P: Packet>(&mut self) -> Result<Option<P>, SocketErr> {
		Ok(recv_packet::<P>(&mut self.socket)?
			.map(|(x, _)| x))
	}
}