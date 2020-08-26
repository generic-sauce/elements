use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum Packet {
	Init,
	Update(World),
}

pub fn send_packet(socket: &mut UdpSocket, p: Packet) {
	unimplemented!() // TODO
}

pub fn recv_packet(socket: &mut UdpSocket) -> Option<(Packet, SocketAddr)> {
	unimplemented!() // TODO
}
