use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum Packet {
	Init,
	Update(World),
	Input(InputState),
}

pub fn send_packet(socket: &mut UdpSocket, p: Packet) {
	unimplemented!() // TODO
}

pub fn send_packet_to(socket: &mut UdpSocket, p: Packet, target: SocketAddr) {
	unimplemented!() // TODO
}

pub fn recv_packet(socket: &mut UdpSocket) -> Option<(Packet, SocketAddr)> {
	unimplemented!() // TODO
}
