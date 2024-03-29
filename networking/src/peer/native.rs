use crate::prelude::*;

impl PeerManager {
	pub fn tick_native<R: Packet>(&mut self) -> (Vec<PeerEvent<R>>, Vec<SocketErr>) {
		let mut events = Vec::new();
		let mut errs = Vec::new();

		// recv-packets
		'recvloop: loop {
			match recv_bytes(&mut self.udp_socket) {
				Ok(Some((bytes, recv_addr))) => {
					let handle = self.peers.iter()
						.enumerate()
						.filter(|(_, p)| p.alive)
						.find(|(_, p)|
							match p.kind {
								PeerKind::Native { addr, .. } => addr == recv_addr,
								_ => false,
							}
						).map(|(i, p)|
						PeerHandle {
							index: i,
							generation: p.generation,
						}
					);

					let packet = match deser::<NativeCSPacket<R>>(&bytes) {
						Ok(x) => x,
						Err(x) => {
							errs.push(x);
							continue 'recvloop;
						}
					};

					let handle = match handle {
						Some(h) => { // if peer existed before: update last_recv_time
							if let PeerKind::Native { last_recv_time, .. } = &mut self.peers[h.index].kind {
								*last_recv_time = Instant::now();
							}

							h
						},
						None => { // if peer didn't exist before: add new peer
							let kind = PeerKind::Native {
								addr: recv_addr,
								last_recv_time: Instant::now(),
							};
							let new_handle = add_peer(&mut self.peers, kind);
							events.push(PeerEvent::NewPeer(new_handle));

							new_handle
						}
					};

					if let NativeCSPacket::Payload(p) = packet {
						events.push(PeerEvent::ReceivedPacket(p, handle));
					}
				},
				Ok(None) => break 'recvloop,
				Err(x) => {
					errs.push(x);
					break 'recvloop;
				}
			};
		}

		// drop old peers
		for (i, p) in self.peers.iter_mut().enumerate() {
			if !p.alive { continue; }

			if let PeerKind::Native { last_recv_time, .. } = p.kind {
				if last_recv_time.elapsed().as_secs() > PEER_DROP_TIMEOUT_SECS as u64 {
					p.alive = false;

					let handle = PeerHandle {
						index: i,
						generation: p.generation,
					};

					events.push(PeerEvent::Disconnect(handle));
				}
			}
		}

		(events, errs)
	}
}
