use crate::prelude::*;

pub const AWAITING_TIMEOUT: u32 = 50;

pub struct MasterServer {
	pub peer_manager: PeerManager,
	pub game_servers: Vec<GameServerInfo>,
	pub clients: Vec<ClientInfo>,
}

pub struct GameServerInfo {
	pub peer_index: usize,
	pub num_players: u32,
	pub state: GameServerState,
	pub port: u16,
}

#[derive(Clone)]
pub struct ClientInfo {
	pub peer_index: usize,
	pub name: String,
	pub state: ClientState,
}

#[derive(Clone)]
pub enum ClientState {
	Ready,
	InGame,
}

pub enum GameServerState {
	Ready,
	/*
	 * This state is set, if this master server redirected clients to this game server,
	 * but the game server did not acknowledged until now.
	 * The u32 saves the number of frames, since this GameServer is
	 */
	AwaitingGame(u32),
	InGame,
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
					MasterServerPacket::GameServerStatusUpdate { num_players, port } => {
						self.apply_game_server_status_update(peer_index, num_players, port);
					},
					MasterServerPacket::ClientRequest { name } => {
						self.apply_client_request(peer_index, name);
					}
				}
			}

			self.check_awaiting_servers();
		}
	}

	fn check_awaiting_servers(&mut self) {
		for server in self.game_servers.iter_mut() {
			if let GameServerState::AwaitingGame(frames) = &mut server.state {
				*frames += 1;
				if *frames > AWAITING_TIMEOUT {
					// clients did not connect in 5 seconds -> make this server available again
					server.state = GameServerState::Ready;
				}
			}
		}
	}

	fn apply_game_server_status_update(&mut self, peer_index: usize, num_players: u32, port: u16) {
		if let Some(game_server) = self.game_servers.iter_mut().find(|gs| gs.peer_index == peer_index) {
			game_server.num_players = num_players;
			if num_players == 2 {
				game_server.state = GameServerState::InGame;
			} else if matches!(game_server.state, GameServerState::InGame) {
				game_server.state = GameServerState::Ready;
			}
			if game_server.port != port {
				eprintln!("WARNING: game server changing port {} -> {}", game_server.port, port);
			}
			game_server.port = port;
		} else {
			self.game_servers.push(GameServerInfo::new(peer_index, num_players, port));
		}
	}

	fn apply_client_request(&mut self, peer_index: usize, name: String) {
		println!("got client request: {}", name);
		if let Some(client_info) = self.clients.iter_mut().find(|c| c.peer_index == peer_index) {
			client_info.name = name;
		} else {
			self.clients.push(ClientInfo::new(peer_index, &name));
		}
		self.check_game_start();
	}

	fn check_game_start(&mut self) {
		let mut ready_clients: Vec<&mut ClientInfo> = self.clients.iter_mut().filter(|c| matches!(c.state, ClientState::Ready)).collect();
		if ready_clients.len() >= 2 {
			if let Some(game_server) = self.game_servers.iter_mut().find(|gs| matches!(gs.state, GameServerState::Ready)) {
				MasterServer::initiate_game(&mut self.peer_manager, game_server, &mut ready_clients[0..2]);
			} else {
				// TODO: no server could be found
			}
		}
	}

	fn initiate_game(peer_manager: &mut PeerManager, game_server: &mut GameServerInfo, clients: &mut [&mut ClientInfo]) {
		println!("initiating game with players: {}, {}", clients[0].name, clients[1].name);
		for client in clients {
			let game_server_ip = format!("{}", peer_manager.get_udp_ip(game_server.peer_index).unwrap().ip());
			peer_manager.send_to(client.peer_index, &MasterClientPacket::GameRedirection(game_server_ip, game_server.port));
			game_server.state = GameServerState::AwaitingGame(0);
			client.state = ClientState::InGame;
		}
	}
}


impl ClientInfo {
	fn new(peer_index: usize, name: &str) -> ClientInfo {
		ClientInfo {
			peer_index,
			name: String::from(name),
			state: ClientState::Ready,
		}
	}
}

impl GameServerInfo {
	pub fn new(peer_index: usize, num_players: u32, port: u16) -> GameServerInfo {
		GameServerInfo {
			peer_index,
			num_players,
			state: GameServerState::Ready,
			port,
		}
	}
}