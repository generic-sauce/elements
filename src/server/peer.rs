use crate::prelude::*;

enum Peer {
	Web(TungSocket),
	Native(SocketAddr),
}

pub struct PeerManager {
	udp_socket: UdpSocket,
	peers: [Peer; 2],
}

impl PeerManager {
	pub fn wait_for_players() -> PeerManager {
		let mut peers = Vec::new();

		// native
		let mut udp_socket = UdpSocket::bind(("0.0.0.0", PORT)).expect("Could not create server udp-socket");
		udp_socket.set_nonblocking(true).unwrap();

		// web
		let listener = TcpListener::bind(("0.0.0.0", PORT)).expect("Could not create server tcp-listener");
		listener .set_nonblocking(true).unwrap();

		for _ in TimedLoop::with_fps(10) {
			// native
			if let Some((Init::Init, recv_addr)) = recv_packet(&mut udp_socket) {
				peers.push(Peer::Native(recv_addr));
				println!("new player joined {}", recv_addr);
				if peers.len() == 2 {
					break;
				}
			}

			// web
			match listener.accept().map_err(|e| e.kind()) {
				Ok((stream, recv_addr)) => {
					let mut tung = tungstenite::server::accept(stream).unwrap();
					tung.get_mut().set_nonblocking(true).unwrap();
					peers.push(Peer::Web(tung));

					println!("new player joined {}", recv_addr);
					if peers.len() == 2 {
						break;
					}
				},
				Err(ErrorKind::WouldBlock) => {},
				Err(_) => panic!("listener.accept() failed"),
			}
		}

		let peers = [peers.remove(0), peers.remove(0)];

		PeerManager {
			udp_socket,
			peers,
		}
	}

	pub fn send_to(&mut self, i: usize, p: &impl Packet) {
		match &mut self.peers[i] {
			Peer::Native(addr) => send_packet_to(&mut self.udp_socket, p, *addr),
			Peer::Web(socket) => {
				socket.write_message(Message::Binary(ser(p))).unwrap();
			}
		}
	}

	pub fn recv_from<P: Packet>(&mut self) -> Option<(P, usize)> {
		self.native_recv_from::<P>().or_else(|| self.web_recv_from::<P>())
	}

	fn native_recv_from<P: Packet>(&mut self) -> Option<(P, usize)> {
		let (p, addr) = recv_packet::<P>(&mut self.udp_socket)?;
		let idx = (0..2)
			.find(|&i| match self.peers[i] {
				Peer::Native(peer_addr) => peer_addr == addr,
				_ => false,
			})
			.unwrap();
		Some((p, idx))
	}

	fn web_recv_from<P: Packet>(&mut self) -> Option<(P, usize)> {
		self.peers.iter_mut().enumerate()
			.filter_map(|(i, p)| match p {
				Peer::Web(socket) => {
					tung_recv_packet::<P>(socket)
						.map(|p| (p, i))
				},
				_ => None,
			})
			.next()
	}
}

fn tung_recv_packet<P: Packet>(socket: &mut TungSocket) -> Option<P> {
	while socket.can_read() {
		let bytes = match socket.read_message() {
			Ok(Message::Binary(b)) => b,
			Ok(Message::Text(_)) => panic!("text should not be sent!"),
			Ok(_) => continue,
			Err(tungstenite::error::Error::Io(io_err)) => {
				if io_err.kind() == std::io::ErrorKind::WouldBlock {
					return None;
				}
				panic!("recv error (1)");
			}
			Err(_) => panic!("recv error (2)"),
		};
		let p = deser::<P>(&bytes[..]);
		return Some(p);
	}
	None
}
