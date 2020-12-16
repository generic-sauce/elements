use crate::prelude::*;

mod web;
mod native;

pub enum PeerEvent<R: Packet> {
	NewPeer(PeerHandle),
	ReceivedPacket(R, PeerHandle),
	Disconnect(PeerHandle),
}

enum PeerKind {
	Http(TungSocket),
	Https(TungTlsSocket),
	Native(SocketAddr),
}

struct Peer {
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

		events.extend(self.tick_udp());
		events.extend(self.tick_web());

		events
	}

	pub fn send_to(&mut self, handle: PeerHandle, p: &impl Packet) {
		match self.get_mut(handle) {
			Some(PeerKind::Native(addr)) => send_packet_to(&mut self.udp_socket, p, *addr),
			Some(PeerKind::Http(socket)) => {
				if socket.can_write() { socket.write_message(Message::Binary(ser(p))).unwrap(); }
			},
			Some(PeerKind::Https(socket)) => {
				if socket.can_write() { socket.write_message(Message::Binary(ser(p))).unwrap(); }
			},
			None => println!("send_to: Peer does not exist!"),
		}
	}

	pub fn get_udp_ip(&self, handle: PeerHandle) -> Option<SocketAddr> {
		let k = self.get(handle).map(|p| p.kind);
		if let Some(PeerKind::Native(addr)) = k {
			Some(addr)
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

fn add_peer(peers: &mut Vec<Peer>, kind: PeerKind) -> PeerHandle { // TODO re-use long-dead indices
	let index = peers.len();
	let peer = Peer {
		generation: 0,
		alive: true,
		kind,
	};
	peers.push(peer);

	PeerHandle {
		generation: 0,
		index,
	}
}

fn tls_acceptor() -> Option<Arc<TlsAcceptor>> {
	let mut file = File::open("/root/identity.pfx").ok()?;
	let mut identity = vec![];
	file.read_to_end(&mut identity).unwrap();
	let identity = Identity::from_pkcs12(&identity, "test123").unwrap();

	let acceptor = TlsAcceptor::new(identity).unwrap();
	Some(Arc::new(acceptor))
}

