use crate::prelude::*;

pub enum NativeSocketBackend {
	Connected {
		socket: UdpSocket,
		last_sent_time: Instant,
	},
	NotYetConnected(String, u16),
}

impl SocketBackend for NativeSocketBackend {
	fn new(server_ip: &str, port: u16) -> Self {
		let mut socket = match UdpSocket::bind("0.0.0.0:0") {
			Ok(x) => x,
			Err(x) => {
				eprintln!("UdpSocket::bind failed: {:?}", x);
				return NativeSocketBackend::NotYetConnected(server_ip.to_string(), port);
			}
		};

		if let Err(x) = socket.set_nonblocking(true) {
			eprintln!("socket.set_nonblocking failed: {:?}", x);
			return NativeSocketBackend::NotYetConnected(server_ip.to_string(), port);
		}

		if let Err(x) = socket.connect((server_ip, port)) {
			eprintln!("socket.connect failed: {:?}", x);
			return NativeSocketBackend::NotYetConnected(server_ip.to_string(), port);
		}

		let last_sent_time= Instant::now();
		send_packet(&mut socket, &NativeCSPacket::<()>::Heartbeat).unwrap();

		Self::Connected {
			socket,
			last_sent_time,
		}
	}

	fn is_open(&self) -> bool {
		matches!(self, NativeSocketBackend::Connected { .. })
	}

	fn send(&mut self, packet: &impl Packet) -> std::io::Result<()> {
		match self {
			NativeSocketBackend::Connected { socket, last_sent_time } => {
				*last_sent_time = Instant::now();
				send_packet(socket, &NativeCSPacket::Payload(packet.clone())) // TODO: maybe fix this clone, see https://serde.rs/lifetimes.html
			},
			NativeSocketBackend::NotYetConnected(_, _) => {
				panic!("trying to send while in NativeSocketBackend::NotYetConnected (i.e. is_ready() == false)");
			},
		}
	}

	fn tick(&mut self) {
		match self {
			NativeSocketBackend::Connected { socket, last_sent_time } => {
				if last_sent_time.elapsed().as_secs() >= HEARTBEAT_TIME_SECS as u64 {
					*last_sent_time = Instant::now();
					send_packet(socket, &NativeCSPacket::<()>::Heartbeat).unwrap();
				}
			},
			NativeSocketBackend::NotYetConnected(domain, port) => {
				*self = NativeSocketBackend::new(&domain, *port); // connection retry
			}
		}
	}

	fn recv<P: Packet>(&mut self) -> Option<P> {
		match self {
			NativeSocketBackend::Connected { socket, .. } => {
				recv_packet::<P>(socket)
					.map(|(x, _)| x)
			},
			NativeSocketBackend::NotYetConnected(_, _) => None,
		}
	}
}
