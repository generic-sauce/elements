use crate::prelude;
use super::*;

impl PeerManager {
	pub(super) fn tick_native<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
		let mut events = Vec::new();

		while let Some((bytes, recv_addr)) = recv_bytes(&mut self.udp_socket) {
			let handle = self.peers.iter()
				.enumerate()
				.find(|(_, p)|
					match p.kind {
						PeerKind::Native(a) => a == recv_addr,
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
				let new_handle = add_peer(&mut self.peers, PeerKind::Native(recv_addr));
				events.push(PeerEvent::NewPeer(new_handle));

				new_handle
			});

			if let NativeCSPacket::Payload(p) = packet {
				events.push(PeerEvent::ReceivedPacket(p, handle));
			}
		}

		events
	}
}
