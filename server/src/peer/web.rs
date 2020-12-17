use crate::prelude::*;

impl PeerManager {
	pub fn tick_web<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
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
		for (index, peer) in self.peers.iter_mut().enumerate() {
			if !peer.alive { continue; }

			let handle = PeerHandle {
				index,
				generation: peer.generation,
			};

			match &mut peer.kind {
				PeerKind::Http(s) => events.extend(tung_fetch_events(handle, s, &mut peer.alive)),
				PeerKind::Https(s) => events.extend(tung_fetch_events(handle, s, &mut peer.alive)),
				_ => {},
			}
		}

		events
	}
}

fn tung_fetch_events<P: Packet, C: Read + Write>(handle: PeerHandle, socket: &mut tungstenite::WebSocket<C>, alive: &mut bool) -> Vec<PeerEvent<P>> {
	let mut events = Vec::new();

	if socket.can_write() {
		loop {
			match socket.read_message() {
				Ok(Message::Binary(bytes)) => {
					let p = deser::<P>(&bytes[..]);
					events.push(PeerEvent::ReceivedPacket(p, handle));
				},
				Ok(Message::Text(_)) => panic!("text should not be sent!"),
				Ok(Message::Close(_)) => {
					events.push(PeerEvent::Disconnect(handle));
					*alive = false;
					break;
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
	} else {
		events.push(PeerEvent::Disconnect(handle));
		*alive = false;
	}

	events
}
