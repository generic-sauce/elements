use crate::prelude::*;

mod native;
mod web;

pub type TungSocket = tungstenite::WebSocket<TcpStream>;
pub type TungTlsSocket = tungstenite::WebSocket<native_tls::TlsStream<TcpStream>>;

pub enum PeerEvent<R: Packet> {
	NewPeer(PeerHandle),
	ReceivedPacket(R, PeerHandle),
	Disconnect(PeerHandle),
}

pub enum PeerKind {
	Http(TungSocket),
	Https(TungTlsSocket),
	Native {
		addr: SocketAddr,
		last_recv_time: Instant,
	},
}

pub struct Peer {
	kind: PeerKind,
	generation: u32,
	alive: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerHandle {
	index: usize,
	generation: u32,
}

pub struct PeerManager {
	udp_socket: UdpSocket,
	https_listener: TcpListener,
	http_listener: TcpListener,
	acceptor: Option<Arc<TlsAcceptor>>,
	peers: Vec<Peer>,
}

impl PeerManager {
	pub fn new(port: u16, https_port: u16, identity_file: Option<&str>) -> PeerManager {
		// native
		let udp_socket = UdpSocket::bind(("0.0.0.0", port)).expect("Could not create server udp-socket");
		udp_socket.set_nonblocking(true).unwrap();

		// https
		let https_listener = TcpListener::bind(("0.0.0.0", https_port)).expect("Could not create server https tcp-listener");
		https_listener.set_nonblocking(true).unwrap();

		// http
		let http_listener = TcpListener::bind(("0.0.0.0", port)).expect("Could not create server http tcp-listener");
		http_listener.set_nonblocking(true).unwrap();

		let acceptor = tls_acceptor(identity_file);

		if acceptor.is_none() {
			println!("WARN: no identity file found. HTTPS is disabled!");
		}

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

		let (evs, errs) = self.tick_native();
		events.extend(evs);
		for err in errs {
			eprintln!("PeerManager::tick_native error: {}", err);
		}

		let (evs, errs) = self.tick_web();
		events.extend(evs);
		for err in errs {
			eprintln!("PeerManager::tick_web error: {}", err);
		}

		events
	}

	pub fn send_to(&mut self, handle: PeerHandle, p: &impl Packet) -> Result<(), SocketErr> {
		let opt = self.peers.get_mut(handle.index)
			.filter(|p| p.alive)
			.filter(|p| p.generation == handle.generation)
			.map(|p| &mut p.kind);

		match opt {
			Some(PeerKind::Native { addr, .. }) => send_packet_to(&mut self.udp_socket, p, *addr)?,
			Some(PeerKind::Http(socket)) => {
				if socket.can_write() { socket.write_message(Message::Binary(ser(p)?))?; }
			},
			Some(PeerKind::Https(socket)) => {
				if socket.can_write() { socket.write_message(Message::Binary(ser(p)?))?; }
			},
			None => println!("send_to: Peer does not exist!"),
		}

		Ok(())
	}

	pub fn get_udp_ip(&self, handle: PeerHandle) -> Option<SocketAddr> {
		let opt = self.peers.get(handle.index)
			.filter(|p| p.alive)
			.filter(|p| p.generation == handle.generation)
			.map(|p| &p.kind);

		if let Some(PeerKind::Native { addr, .. }) = opt {
			Some(*addr)
		} else {
			None
		}
	}

	pub fn get_peer_handles(&self) -> Vec<PeerHandle> {
		self.peers.iter()
			.enumerate()
			.filter(|(_, p)| p.alive)
			.map(|(i, p)| PeerHandle {
				index: i,
				generation: p.generation
			})
			.collect()
	}
}

pub fn add_peer(peers: &mut Vec<Peer>, kind: PeerKind) -> PeerHandle {
	if let Some(i) = peers.iter().position(|p| !p.alive) {
		let generation = peers[i].generation + 1;

		peers[i] = Peer {
			generation,
			alive: true,
			kind,
		};

		PeerHandle {
			generation,
			index: i,
		}
	} else {
		let index = peers.len();

		peers.push(Peer {
			generation: 0,
			alive: true,
			kind,
		});

		PeerHandle {
			generation: 0,
			index,
		}
	}
}

fn tls_acceptor(identity_file: Option<&str>) -> Option<Arc<TlsAcceptor>> {
	let mut file = File::open(identity_file?).ok()?;
	let mut identity = vec![];
	file.read_to_end(&mut identity).unwrap();
	let passphrase = match std::env::var("ELEMENTS_IDENTITY_PASSPHRASE") {
		Ok(var) => var,
		Err(e) => match e {
			std::env::VarError::NotPresent => String::from(""),
			std::env::VarError::NotUnicode(_) => panic!("Identity passphrase is not unicode. Use a different passphrase"),
		}
	};
	let identity = Identity::from_pkcs12(&identity, &passphrase).unwrap();  // TODO: replace passphrase

	let acceptor = TlsAcceptor::new(identity).unwrap();
	Some(Arc::new(acceptor))
}
