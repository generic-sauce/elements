use crate::prelude::*;

impl PeerManager {
	pub fn tick_web<R: Packet>(&mut self) -> (Vec<PeerEvent<R>>, Vec<SocketErr>) {
		let mut events = Vec::new();
		let mut errs: Vec<SocketErr> = Vec::new();

		// https-accept
		if let Some(acceptor) = self.acceptor.as_mut() {
			's_acceptloop: loop {
				match self.https_listener.accept(){
					Ok((stream, _)) => {
						let tls_stream = match acceptor.accept(stream) {
							Ok(x) => x,
							Err(x) => {
								errs.push(Box::new(x));
								continue 's_acceptloop;
							}
						};
						let mut tung = match tungstenite::server::accept(tls_stream) {
							Ok(x) => x,
							Err(x) => {
								errs.push(Box::new(x));
								continue 's_acceptloop;
							}
						};
						if let Err(x) = tung.get_mut().get_mut().set_nonblocking(true) {
							errs.push(Box::new(x));
							continue 's_acceptloop;
						}

						let handle = add_peer(&mut self.peers, PeerKind::Https(tung));
						events.push(PeerEvent::NewPeer(handle));
					},
					Err(x) if matches!(x.kind(), ErrorKind::WouldBlock) => break 's_acceptloop,
					Err(x) => {
						errs.push(Box::new(x));
						eprintln!("breaking accept loop (https)!");
						break 's_acceptloop;
					}
				}
			}
		}

		// http-accept
		'acceptloop: loop {
			match self.http_listener.accept() {
				Ok((stream, _)) => {
					let mut tung = match tungstenite::server::accept(stream) {
						Ok(x) => x,
						Err(x) => {
							errs.push(Box::new(x));
							continue 'acceptloop;
						}
					};
					if let Err(x) = tung.get_mut().set_nonblocking(true) {
						errs.push(Box::new(x));
						continue 'acceptloop;
					}

					let handle = add_peer(&mut self.peers, PeerKind::Http(tung));
					events.push(PeerEvent::NewPeer(handle));
				},
				Err(e) if e.kind() == ErrorKind::WouldBlock => break 'acceptloop,
				Err(x) => {
					errs.push(Box::new(x));
					eprintln!("breaking accept loop (http)!");
					break 'acceptloop;
				}
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
				PeerKind::Http(s) => {
					match tung_fetch_events(handle, s, &mut peer.alive) {
						Ok(x) => events.extend(x),
						Err(x) => errs.push(x),
					}
				},
				PeerKind::Https(s) => {
					match tung_fetch_events(handle, s, &mut peer.alive) {
						Ok(x) => events.extend(x),
						Err(x) => errs.push(x),
					}
				}
				_ => {},
			}
		}

		(events, errs)
	}
}

fn tung_fetch_events<P: Packet, C: Read + Write>(handle: PeerHandle, socket: &mut tungstenite::WebSocket<C>, alive: &mut bool) -> Result<Vec<PeerEvent<P>>, SocketErr> {
	let mut events = Vec::new();

	if socket.can_write() {
		'fetchloop: loop {
			match socket.read_message() {
				Ok(Message::Binary(bytes)) => {
					let p = deser::<P>(&bytes[..])?;
					events.push(PeerEvent::ReceivedPacket(p, handle));
				},
				Ok(Message::Text(_)) => return Err(strerr("tung_fetch_events: text should not be sent")),
				Ok(Message::Close(_)) => {
					events.push(PeerEvent::Disconnect(handle));
					*alive = false;
					break 'fetchloop;
				}
				Ok(_) => continue 'fetchloop,
				Err(tungstenite::error::Error::Io(io_err)) if io_err.kind() == std::io::ErrorKind::WouldBlock => break 'fetchloop,
				Err(e) => return Err(Box::new(e)),
			}
		}
	} else {
		events.push(PeerEvent::Disconnect(handle));
		*alive = false;
	}

	Ok(events)
}
