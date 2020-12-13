use crate::prelude::*;

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
	https_listener: TcpListener,
	http_listener: TcpListener,
	acceptor: Option<Arc<TlsAcceptor>>,
	peers: Vec<Peer>,

	// the following are always disjoint
	accept_udp_packets: Vec<(Vec<u8>, SocketAddr)>, // packets which will be handled on calling `accept`
	recv_udp_packets: Vec<(Vec<u8>, usize /* peer index */)>, // packets which will be handled on calling `recv_from`
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
			accept_udp_packets: Vec::new(),
			recv_udp_packets: Vec::new(),
		}
	}

	pub fn accept(&mut self) {
		{ // native
			let Self { recv_udp_packets, peers, .. } = self; // this allows me to borrow just what I need in the closure below
			let mut handle_packet = |(bytes, recv_addr)| {
				let new_peer = Peer::Native(recv_addr);

				let pos = (0..peers.len()).find(|&i| match peers[i] {
					Peer::Native(a) => a == recv_addr,
					_ => false,
				});

				match pos {
					Some(x) => recv_udp_packets.push((bytes, x)),
					None => {
						deser::<Init>(&bytes); // This will unwrap() in case its not an init packet!
						peers.push(new_peer)
					},
				}
			};

			// handle old packets
			self.accept_udp_packets.drain(..).for_each(&mut handle_packet);

			// fetch new packets
			while let Some(x) = recv_bytes(&mut self.udp_socket) {
				handle_packet(x);
			}
		}

		// https
		if let Some(acceptor) = self.acceptor.as_mut() {
			loop {
				match self.https_listener.accept().map_err(|e| e.kind()) {
					Ok((stream, _)) => {
						let tls_stream = acceptor.accept(stream).unwrap();
						let mut tung = tungstenite::server::accept(tls_stream).unwrap();
						tung.get_mut().get_mut().set_nonblocking(true).unwrap();
						self.peers.push(Peer::Web(WebPeer::Https(tung)));
					},
					Err(ErrorKind::WouldBlock) => break,
					Err(_) => panic!("listener.accept() failed"),
				}
			}
		}

		// http
		loop {
			match self.http_listener.accept().map_err(|e| e.kind()) {
				Ok((stream, _)) => {
					let mut tung = tungstenite::server::accept(stream).unwrap();
					tung.get_mut().set_nonblocking(true).unwrap();
					self.peers.push(Peer::Web(WebPeer::Http(tung)));
				},
				Err(ErrorKind::WouldBlock) => break,
				Err(_) => panic!("listener.accept() failed"),
			}
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
		// return old packets
		if !self.recv_udp_packets.is_empty() {
			let (bytes, i) = self.recv_udp_packets.swap_remove(0);
			return Some((deser::<P>(&bytes), i));
		}

		// fetch new packets
		let (bytes, addr) = recv_bytes(&mut self.udp_socket)?;
		let idx = (0..self.peers.len())
			.find(|&i| match self.peers[i] {
				Peer::Native(peer_addr) => peer_addr == addr,
				_ => false,
			});

		match idx {
			Some(i) => Some((deser::<P>(&bytes), i)),
			None => {
				self.accept_udp_packets.push((bytes, addr));
				None
			},
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