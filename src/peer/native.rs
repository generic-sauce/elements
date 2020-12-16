use crate::prelude;
use super::*;

impl PeerManager {
	pub(super) fn tick_native<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
		let mut events = Vec::new();

		// recv-packets
		while let Some((bytes, recv_addr)) = recv_bytes(&mut self.udp_socket) {
			let handle = self.peers.iter()
				.enumerate()
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

			let packet = deser::<NativeCSPacket<R>>(&bytes);

			// guarantee that peer exists
			let handle = handle.unwrap_or_else(|| {
				let kind = PeerKind::Native {
					addr: recv_addr,
					last_recv_time: Instant::now(),
				};
				let new_handle = add_peer(&mut self.peers, kind);
				events.push(PeerEvent::NewPeer(new_handle));

				new_handle
			});

			if let NativeCSPacket::Payload(p) = packet {
				events.push(PeerEvent::ReceivedPacket(p, handle));
			}
		}

		// drop old peers
		for p in self.peers.iter_mut() {
			if let PeerKind::Native { last_recv_time, .. } = p.kind {
				if last_recv_time.elapsed().as_secs() > PEER_DROP_TIMEOUT_SECS as u64 {
					p.alive = false; // TODO actually drop them
				}
			}
		}

		events
	}
}
