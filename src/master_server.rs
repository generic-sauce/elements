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

			while let Some((p, i)) = self.peer_manager.recv_from::<GameServerStatusUpdate>() {
				println!("got packet from {}. Num players: {}", i, p.num_players);
			}
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