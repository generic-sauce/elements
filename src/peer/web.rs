use crate::prelude;
use super::*;

impl PeerManager {
	pub(super) fn tick_web<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
		let mut events = Vec::new();

		// https-accept
		if let Some(acceptor) = self.acceptor.as_mut() {
			loop {
				match self.https_listener.accept().map_err(|e| e.kind()) {
					Ok((stream, _)) => {
						let tls_stream = acceptor.accept(stream).unwrap();
						let mut tung = tungstenite::server::accept(tls_stream).unwrap();
						tung.get_mut().get_mut().set_nonblocking(true).unwrap();

						let handle = add_peer(&mut self.peers, PeerKind::Https(tung));
						events.push(PeerEvent::NewPeer(handle));
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

					let handle = add_peer(&mut self.peers, PeerKind::Http(tung));
					events.push(PeerEvent::NewPeer(handle));
				},
				Err(ErrorKind::WouldBlock) => break,
				Err(_) => panic!("listener.accept() failed"),
			}
		}

		// recv + disconnect
		for (i, peer) in self.peers.iter_mut().enumerate() {
			if let Peer::Web(web_peer) = peer {
				events.extend(tung_fetch_events::<R>(i, web_peer));
			}
		}

		events
	}
}

fn tung_fetch_events<P: Packet>(i: usize, socket: &mut WebPeer) -> Vec<PeerEvent<P>> {
	let mut events = Vec::new();

	let is_tung_open = |socket: &mut WebPeer| match socket {
		WebPeer::Http(s) => s.can_write(),
		WebPeer::Https(s) => s.can_write(),
	};

	let read_fn = |socket: &mut WebPeer| match socket {
		WebPeer::Http(s) => s.read_message(),
		WebPeer::Https(s) => s.read_message(),
	};

	if is_tung_open(socket) {
		loop {
			match read_fn(socket) {
				Ok(Message::Binary(bytes)) => {
					let p = deser::<P>(&bytes[..]);
					events.push(PeerEvent::ReceivedPacket(p, i));
				},
				Ok(Message::Text(_)) => panic!("text should not be sent!"),
				Ok(Message::Close(_)) => {
					events.push(PeerEvent::Disconnect(i));
				}
				Ok(_) => continue,
				Err(tungstenite::error::Error::Io(io_err)) => {
					if io_err.kind() == std::io::ErrorKind::WouldBlock {
						break;
					}
					panic!("recv error (1)");
				}
				e @ Err(_) => { e.unwrap(); unreachable!(); },
			}
		}
	}

	events
}
