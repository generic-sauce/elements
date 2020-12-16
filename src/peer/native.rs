use crate::prelude;
use super::*;

impl PeerManager {
	pub(super) fn tick_native<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
		let mut events = Vec::new();

		while let Some((bytes, recv_addr)) = recv_bytes(&mut self.udp_socket) {
			let pos = self.peers.iter()
				.position(|p|
					match p.kind {
						PeerKind::Native(a) => a == recv_addr,
						_ => false,
					}
				);

			match pos {
				Some(i) => {
					let handle = PeerHandle { index: i, generation: self.peers[i].generation };
					events.push(PeerEvent::ReceivedPacket(deser::<R>(&bytes), handle));
				},
				None => {
					deser::<Init>(&bytes); // This will unwrap() in case its not an init packet!

					let handle = add_peer(&mut self.peers, PeerKind::Native(recv_addr));
					events.push(PeerEvent::NewPeer(handle));
				},
			}
		}

		events
	}
}
