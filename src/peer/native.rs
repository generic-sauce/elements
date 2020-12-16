use crate::prelude;
use super::*;

impl PeerManager {
	pub(super) fn tick_native<R: Packet>(&mut self) -> Vec<PeerEvent<R>> {
		let mut events = Vec::new();

		while let Some((bytes, recv_addr)) = recv_bytes(&mut self.udp_socket) {

			let pos = self.peers.iter_mut()
				.map(|p| &mut p.kind)
				.position(|p|
					match p.kind {
						&PeerKind::Native(a) => a == recv_addr,
						_ => false,
					}
				)
				.filter(|p| p.alive);

			match pos {
				Some(x) => events.push(PeerEvent::ReceivedPacket(deser::<R>(&bytes), x)),
				None => {
					deser::<Init>(&bytes); // This will unwrap() in case its not an init packet!

					events.push(PeerEvent::NewPeer(self.peers.len()));
					self.peers.push(Peer::Native(recv_addr));
				},
			}
		}

		events
	}
}
