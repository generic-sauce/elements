use crate::prelude::*;

enum WebPeer {
	Http(TungSocket),
	Https(TungTlsSocket),
}

pub enum PeerEvent<R: Packet> {
	NewPeer(usize),
	ReceivedPacket(R, usize),
}

enum Peer {
	Web(WebPeer),
	Native(SocketAddr),
}

pub struct PeerManager {
	udp_socket: UdpSocket,
	https_listener: TcpListener,
	http_listener: TcpListener,
	acceptor: Option<Arc<TlsAcceptor>>,
	peers: Vec<Peer>,
}

impl PeerManager {
	pub fn new(port: u16, https_port: u16) -> PeerManager {
		// native
		let udp_socket = UdpSocket::bind(("0.0.0.0", port)).expect("Could not create server udp-socket");
		udp_socket.set_nonblocking(true).unwrap();

		// https
		let https_listener = TcpListener::bind(("0.0.0.0", https_port)).expect("Could not create server https tcp-listener");
		https_listener.set_nonblocking(true).unwrap();

		// http
		let http_listener = TcpListener::bind(("0.0.0.0", port)).expect("Could not create server http tcp-listener");
		http_listener.set_nonblocking(true).unwrap();

		let acceptor = tls_acceptor();

		PeerManager {
			udp_socket,
			https_listener,
			http_listener,
			acceptor,
			peers: Vec::new(),
		}
	}

	pub fn tick<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
		let mut events = Vec::new();

		{ // native
			while let Some((bytes, recv_addr)) = recv_bytes(&mut self.udp_socket) {
				let pos = self.peers.iter().position(|p|
					match p {
						&Peer::Native(a) => a == recv_addr,
						_ => false,
					}
				);

				match pos {
					Some(x) => events.push(PeerEvent::ReceivedPacket(deser::<R>(&bytes), x)),
					None => {
						deser::<Init>(&bytes); // This will unwrap() in case its not an init packet!

						events.push(PeerEvent::NewPeer(self.peers.len()));
						self.peers.push(Peer::Native(recv_addr));
					},
				}
			}
		}

		// https-accept
		if let Some(acceptor) = self.acceptor.as_mut() {
			loop {
				match self.https_listener.accept().map_err(|e| e.kind()) {
					Ok((stream, _)) => {
						let tls_stream = acceptor.accept(stream).unwrap();
						let mut tung = tungstenite::server::accept(tls_stream).unwrap();
						tung.get_mut().get_mut().set_nonblocking(true).unwrap();

						events.push(PeerEvent::NewPeer(self.peers.len()));
						self.peers.push(Peer::Web(WebPeer::Https(tung)));
					},
					Err(ErrorKind::WouldBlock) => break,
					Err(_) => panic!("listener.accept() failed"),
				}
			}
		}

		// http-accept
		loop {
			match self.http_listener.accept().map_err(|e| e.kind()) {
				Ok((stream, _)) => {
					let mut tung = tungstenite::server::accept(stream).unwrap();
					tung.get_mut().set_nonblocking(true).unwrap();

					events.push(PeerEvent::NewPeer(self.peers.len()));
					self.peers.push(Peer::Web(WebPeer::Http(tung)));
				},
				Err(ErrorKind::WouldBlock) => break,
				Err(_) => panic!("listener.accept() failed"),
			}
		}

		// http/https-recv
		for (i, peer) in self.peers.iter_mut().enumerate() {
			if let Peer::Web(web_peer) = peer {
				while let Some(p) = tung_recv_packet::<R>(web_peer) {
					events.push(PeerEvent::ReceivedPacket(p, i));
				}
			}
		}

		events
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

	pub fn get_udp_ip(&self, index: usize) -> Option<SocketAddr> {
		if let Peer::Native(sock_addr) = self.peers[index] {
			Some(sock_addr)
		} else {
			None
		}
	}

	pub fn count(&self) -> usize {
		self.peers.len()
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