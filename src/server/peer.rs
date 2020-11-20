use crate::prelude::*;

const JOIN_FPS: u32 = 10;
const MAX_SILENT_JOIN_SECONDS: u32 = 2*60;

enum WebPeer {
	Http(TungSocket),
	Https(TungTlsSocket),
}

enum Peer {
	Web(WebPeer),
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

		// https
		let https_listener = TcpListener::bind(("0.0.0.0", HTTPS_PORT)).expect("Could not create server https tcp-listener");
		https_listener.set_nonblocking(true).unwrap();

		// http
		let http_listener = TcpListener::bind(("0.0.0.0", PORT)).expect("Could not create server http tcp-listener");
		http_listener.set_nonblocking(true).unwrap();

		let mut acceptor = tls_acceptor();

		let mut silent_frames = 0;

		for _ in TimedLoop::with_fps(JOIN_FPS) {
			// native
			if let Some((Init::Init, recv_addr)) = recv_packet(&mut udp_socket) {
				peers.push(Peer::Native(recv_addr));
				println!("new player joined {}", recv_addr);
				silent_frames = 0;
				if peers.len() == 2 {
					break;
				}
			}

			// https
			if let Some(acceptor) = acceptor.as_mut() {
				match https_listener.accept().map_err(|e| e.kind()) {
					Ok((stream, recv_addr)) => {
						let tls_stream = acceptor.accept(stream).unwrap();
						let mut tung = tungstenite::server::accept(tls_stream).unwrap();
						tung.get_mut().get_mut().set_nonblocking(true).unwrap();
						peers.push(Peer::Web(WebPeer::Https(tung)));

						println!("new player joined {}", recv_addr);
						silent_frames = 0;
						if peers.len() == 2 {
							break;
						}
					},
					Err(ErrorKind::WouldBlock) => {},
					Err(_) => panic!("listener.accept() failed"),
				}
			}

			// http
			match http_listener.accept().map_err(|e| e.kind()) {
				Ok((stream, recv_addr)) => {
					let mut tung = tungstenite::server::accept(stream).unwrap();
					tung.get_mut().set_nonblocking(true).unwrap();
					peers.push(Peer::Web(WebPeer::Http(tung)));

					println!("new player joined {}", recv_addr);
					silent_frames = 0;
					if peers.len() == 2 {
						break;
					}
				},
				Err(ErrorKind::WouldBlock) => {},
				Err(_) => panic!("listener.accept() failed"),
			}

			if !peers.is_empty() {
				silent_frames += 1;
			}

			if silent_frames > MAX_SILENT_JOIN_SECONDS*JOIN_FPS {
				panic!("No more players joined! Shutting down...");
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
			Peer::Web(WebPeer::Http(socket)) => {
				socket.write_message(Message::Binary(ser(p))).unwrap();
			},
			Peer::Web(WebPeer::Https(socket)) => {
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
				Peer::Web(web_peer) => {
					tung_recv_packet::<P>(web_peer)
						.map(|p| (p, i))
				},
				_ => None,
			})
			.next()
	}
}

fn tung_recv_packet<P: Packet>(socket: &mut WebPeer) -> Option<P> {
	let can_read_fn = |socket: &mut WebPeer| match socket {
		WebPeer::Http(s) => s.can_read(),
		WebPeer::Https(s) => s.can_read(),
	};

	let read_fn = |socket: &mut WebPeer| match socket {
		WebPeer::Http(s) => s.read_message(),
		WebPeer::Https(s) => s.read_message(),
	};

	while can_read_fn(socket) {
		let bytes = match read_fn(socket) {
			Ok(Message::Binary(b)) => b,
			Ok(Message::Text(_)) => panic!("text should not be sent!"),
			Ok(_) => continue,
			Err(tungstenite::error::Error::Io(io_err)) => {
				if io_err.kind() == std::io::ErrorKind::WouldBlock {
					return None;
				}
				panic!("recv error (1)");
			}
			e @ Err(_) => { e.unwrap(); unreachable!(); },
		};
		let p = deser::<P>(&bytes[..]);
		return Some(p);
	}
	None
}

fn tls_acceptor() -> Option<Arc<TlsAcceptor>> {
	let mut file = File::open("/root/identity.pfx").ok()?;
	let mut identity = vec![];
	file.read_to_end(&mut identity).unwrap();
	let identity = Identity::from_pkcs12(&identity, "test123").unwrap();

	let acceptor = TlsAcceptor::new(identity).unwrap();
	Some(Arc::new(acceptor))
}