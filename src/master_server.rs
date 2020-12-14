use crate::prelude::*;

pub const MASTER_SERVER_FPS: u32 = 10;
pub const AWAITING_TIMEOUT: u32 = 5 * MASTER_SERVER_FPS;
pub const CLIENT_REQUEST_TIMEOUT: u32 = 5 * MASTER_SERVER_FPS;

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

pub struct ClientInfo {
	pub peer_index: usize,
	pub name: String,
	pub state: ClientState,
	pub last_request_counter: u32,
}

pub enum ClientState {
	Ready,
	InGame,
	Disconnected,
}

pub enum GameServerState {
	Ready, // TODO: make game servers disconnect
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
		for _info in TimedLoop::with_fps(MASTER_SERVER_FPS) {
			self.peer_manager.accept();

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

			self.check_clients();
			self.check_awaiting_servers();
		}
	}

	fn check_awaiting_servers(&mut self) {
		for server in self.game_servers.iter_mut() {
			if let GameServerState::AwaitingGame(frames) = &mut server.state {
				*frames += 1;
				if *frames > AWAITING_TIMEOUT {
					// clients did not connect in 5 seconds -> make this server available again
					println!("WARN: players did not connect. making Game Server available again.");
					server.state = GameServerState::Ready;
				}
			}
		}
	}

	fn check_clients(&mut self) {
		for client in self.clients.iter_mut().filter(|c| matches!(c.state, ClientState::Ready)) {
			client.last_request_counter += 1;
			if client.last_request_counter >= CLIENT_REQUEST_TIMEOUT {
				client.state = ClientState::Disconnected;
				println!("INFO: client disconnected:  {}", client.name);
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
				eprintln!("WARN: game server changing port {} -> {}", game_server.port, port);
			}
			game_server.port = port;
		} else {
			println!("INFO: new game server connected");
			self.game_servers.push(GameServerInfo::new(peer_index, num_players, port));
		}
	}

	fn apply_client_request(&mut self, peer_index: usize, name: String) {
		if let Some(client) = self.clients.iter_mut().find(|c| c.peer_index == peer_index) {
			client.name = name;
			client.last_request_counter = 0;
			if matches!(client.state, ClientState::Disconnected) {
				println!("INFO: reactivating client: {}", client.name);
				client.state = ClientState::Ready;
			}
		} else {
			println!("INFO: new client connected: {}", name);
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
		let game_server_ip = format!("{}", peer_manager.get_udp_ip(game_server.peer_index).unwrap().ip());
		println!("INFO: initiating game with players: {}, {}\t server ip: {}", clients[0].name, clients[1].name, game_server_ip);
		for client in clients {
			peer_manager.send_to(client.peer_index, &MasterClientPacket::GameRedirection(game_server_ip.clone(), game_server.port));
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
			last_request_counter: 0,
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