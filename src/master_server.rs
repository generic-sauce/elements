use crate::prelude::*;

pub struct MasterServer {
	pub peer_manager: PeerManager,
	pub game_servers: Vec<GameServerInfo>,
	pub clients: Vec<ClientInfo>,
}

pub struct GameServerInfo {
	pub peer_index: usize,
	pub num_players: u32,
}

pub struct ClientInfo {
	pub peer_index: usize,
	pub name: String,
}

impl MasterServer {
	pub fn new() -> MasterServer {
		MasterServer {
			peer_manager: PeerManager::new(MASTER_SERVER_PORT, MASTER_SERVER_HTTPS_PORT),
			game_servers: Vec::new(),
			clients: Vec::new(),
		}
	}

	pub fn run(&mut self) {
		for _info in TimedLoop::with_fps(10) {
			let count_before = self.peer_manager.count();
			self.peer_manager.accept();
			if self.peer_manager.count() != count_before {
				println!("new peer connected, count: {}", self.peer_manager.count());
			}

			while let Some((packet, peer_index)) = self.peer_manager.recv_from::<MasterServerPacket>() {
				match packet {
					MasterServerPacket::GameServerStatusUpdate { num_players } => {
						self.apply_game_server_status_update(peer_index, num_players);
					},
					MasterServerPacket::ClientRequest { name } => {
						self.apply_client_request(peer_index, name);
					}
				}
			}
		}
	}

	fn apply_game_server_status_update(&mut self, peer_index: usize, num_players: u32) {
		println!("got packet from peer {}. Num players: {}", peer_index, num_players);
		if let Some(game_server) = self.game_servers.iter_mut().find(|gs| gs.peer_index == peer_index) {
			game_server.num_players = num_players;
		} else {
			self.game_servers.push(GameServerInfo { num_players, peer_index });
		}
	}

	fn apply_client_request(&mut self, peer_index: usize, name: String) {
		if matches!(self.clients.iter().find(|c| c.peer_index == peer_index), None) {
            self.clients.push(ClientInfo { name, peer_index });
		}
	}
}

impl GameServerInfo {
	pub fn new(peer_index: usize) -> GameServerInfo {
		GameServerInfo {
			peer_index,
			num_players: 0,
		}
	}
}