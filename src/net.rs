use crate::prelude::*;

pub trait Packet: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize)]
pub struct Init;

impl Packet for Init {}
impl Packet for World {}
impl Packet for InputState {}

pub fn send_packet(socket: &mut UdpSocket, p: &impl Packet) {
	unimplemented!() // TODO
}

pub fn send_packet_to(socket: &mut UdpSocket, p: &impl Packet, target: SocketAddr) {
	unimplemented!() // TODO
}

pub fn recv_packet<P: Packet>(socket: &mut UdpSocket) -> Option<(P, SocketAddr)> {
	unimplemented!() // TODO
}
