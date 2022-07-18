use networking::prelude::*;
use clap::{App as ClapApp, Arg};

pub const MASTER_SERVER_FPS: u32 = 10;
pub const AWAITING_TIMEOUT: u32 = 5 * MASTER_SERVER_FPS;
pub const CLIENT_REQUEST_TIMEOUT: u32 = 5 * MASTER_SERVER_FPS;

pub struct MasterServer {
	pub peer_manager: PeerManager,
	pub game_servers: Vec<GameServerInfo>, // list of ready game server: game_servers will be popped when the games starts but will rejoin afterwards
	pub clients: Vec<ClientInfo>,
	pub lobbies: Vec<LobbyData>,
}

pub struct LobbyData {
	lobby_id: u32,
	name: String,
	players: Vec<PeerHandle>, // players[0] is the owner
	teams: Vec<u8>,
	max_no_players: u32,
	map_id: u8,
}

pub struct GameServerInfo {
	pub peer: PeerHandle,
	pub domain_name: String,
	pub port: u16,
}

pub struct ClientInfo {
	pub peer: PeerHandle,
	pub name: String,
	pub last_request_counter: u32,
}

impl MasterServer {
	pub fn new(port: u16, identity_file: Option<&str>) -> MasterServer {
		MasterServer {
			peer_manager: PeerManager::new(port, port+1, identity_file),
			game_servers: Vec::new(),
			clients: Vec::new(),
			lobbies: Vec::new(),
		}
	}

	fn alloc_lobby_id(&self) -> u32 {
		let mut v: Vec<_> = self.lobbies.iter().map(|x| x.lobby_id).collect();
		v.sort_unstable();
		v.iter().zip(0u32..)
			.find(|(old_id, new_id)| *old_id != new_id)
			.map(|(_, new)| new)
			.unwrap_or(v.len() as u32)
	}

	pub fn send_lobby_info(&mut self, lobby_id: u32) {
		let idx = self.lobbies.iter().position(|x| x.lobby_id == lobby_id).unwrap();
		let l = &mut self.lobbies[idx];

		let mut player_names = Vec::new();
		for &p in &l.players {
			let client_info = &self.clients.iter().find(|x| x.peer == p).unwrap();
			player_names.push(client_info.name.clone());
		}

		for (&p, your_player_index) in l.players.iter().zip(0u32..) {
			let packet = MasterClientPacket::LobbyInfoUpdate(LongLobbyInfo {
				lobby_id,
				name: l.name.clone(),
				player_names: player_names.clone(),
				your_player_index,
				map_id: l.map_id,
				teams: l.teams.clone(),
			});
			if let Err(x) = self.peer_manager.send_to(p, &packet) {
				eprintln!("master-server: can't send MasterClientPacket::LobbyInfoUpdate to some client \"{}\"", x);
			}
		}
	}

	pub fn run(&mut self) {
		println!("INFO: master server started. Listening on port {}", DEFAULT_MASTER_SERVER_PORT);
		for _info in TimedLoop::with_fps(MASTER_SERVER_FPS) {
			'packet_loop: for ev in self.peer_manager.tick::<MasterServerPacket>() {
				match ev {
					// received packets from game servers
					PeerEvent::ReceivedPacket(MasterServerPacket::GameServerReady{ domain_name, port }, peer) => {
						self.apply_game_server_ready(peer, domain_name, port);
					}

					// received packets from clients
					PeerEvent::ReceivedPacket(MasterServerPacket::Login(name), peer) => {
						self.apply_login_request(peer, name);
					}
					PeerEvent::ReceivedPacket(MasterServerPacket::CreateLobby(lobby_name), peer) => {
						if self.lobbies.iter().any(|d| d.players.contains(&peer)) { continue 'packet_loop; } // ignore packet if player is already in lobby

						let lobby_id = self.alloc_lobby_id();
						self.lobbies.push(LobbyData {
							lobby_id,
							name: lobby_name,
							players: vec![peer],
							teams: vec![0],
							max_no_players: 2,
							map_id: 0,
						});

						self.send_lobby_info(lobby_id);
					}
					PeerEvent::ReceivedPacket(MasterServerPacket::JoinLobby(lobby_id), peer) => {
						if self.lobbies.iter().any(|d| d.players.contains(&peer)) { continue 'packet_loop; } // ignore packet if player is already in lobby

						if let Some(d) = self.lobbies.iter_mut().find(|d| d.lobby_id == lobby_id) {
							d.players.push(peer);
							let id = d.lobby_id;
							self.send_lobby_info(id);
						}
					}
					PeerEvent::ReceivedPacket(MasterServerPacket::LeaveLobby, peer) => {
						if let Some(lobby_idx) = self.lobbies.iter().position(|l| l.players.contains(&peer)) {
							let l = &mut self.lobbies[lobby_idx];
							let peer_idx = l.players.iter().position(|&x| x == peer).unwrap();
							l.players.remove(peer_idx);

							if l.players.is_empty() { // empty lobby will be deleted
								self.lobbies.remove(lobby_idx);
							} else { // the remaining players will be informed
								let id = l.lobby_id;
								self.send_lobby_info(id);
							}
						}
					}
					PeerEvent::ReceivedPacket(MasterServerPacket::LobbyListRequest, peer) => {
						let v = self.lobbies.iter().map(| x | ShortLobbyInfo {
							lobby_id: x.lobby_id,
							name: x.name.clone(),
							no_players: x.players.len() as u32,
							max_no_players: x.max_no_players,
							map_id: x.map_id,
						}).collect();
						if let Err(x) = self.peer_manager.send_to(peer, &MasterClientPacket::LobbyListResponse(v)) {
							eprintln!("can't send LobbyListResponse to some client! error: {}", x);
						}
					}
					PeerEvent::ReceivedPacket(MasterServerPacket::StartGame, peer) => {
						if let Some(idx) = self.lobbies.iter().position(|l| l.players[0] == peer) {
							if let Some(game_server) = self.game_servers.pop() {
								let lobby = self.lobbies.remove(idx);
								for &p in &lobby.players {
									if let Err(x) = self.peer_manager.send_to(p, &MasterClientPacket::GoToGameServer(game_server.domain_name.clone(), game_server.port)) {
										eprintln!("can't send GoToGameServer to some client! error: {}", x);
									}
								}
								println!("INFO: initiating game on game server {}:{}", &game_server.domain_name, game_server.port);

								let packet = MasterToGameServerGoPacket {
									map_id: lobby.map_id,
									teams: (0..lobby.players.len()).map(|x| (x % 2) as u8).collect::<Vec<_>>() // TODO make configurable in the lobby
								};
								if let Err(x) = self.peer_manager.send_to(game_server.peer, &packet) {
									eprintln!("WARN: can't send MasterToGameServerGoPacket due to \"{}\"", x);
								}
							} else {
								eprintln!("no game server is ready!");
							}
						} else {
							eprintln!("some non-host (or someone not even in a lobby) wants to StartGame.. foolish!");
						}
					}
					PeerEvent::ReceivedPacket(MasterServerPacket::ChangeLobbySettings(settings), peer) => {
						if let Some(d) = self.lobbies.iter_mut().find(|d| d.players[0] == peer) {
							d.map_id = settings.map_id;

							let id = d.lobby_id;
							self.send_lobby_info(id);
						}
					}

					// other events
					PeerEvent::NewPeer(_) => {},
					PeerEvent::Disconnect(peer) => {
						for removed_peer in self.game_servers.my_drain_filter(|gs| gs.peer == peer) {
							println!("INFO: game server disconnected: {}:{}", removed_peer.domain_name, removed_peer.port);
						}
						for removed_client in self.clients.my_drain_filter(|gs| gs.peer == peer) {
							println!("INFO: client disconnected: {}", removed_client.name);
						}

						if let Some(lobby_idx) = self.lobbies.iter().position(|l| l.players.contains(&peer)) {
							let l = &mut self.lobbies[lobby_idx];
							let peer_idx = l.players.iter().position(|&x| x == peer).unwrap();
							l.players.remove(peer_idx);

							if l.players.is_empty() { // empty lobby will be deleted
								self.lobbies.remove(lobby_idx);
							}
						}
					}
				}
			}
		}
	}

	fn apply_game_server_ready(&mut self, peer: PeerHandle, domain_name: String, port: u16) {
		if self.game_servers.iter().find(|x| x.peer == peer).is_some() { return; }
		self.game_servers.push(GameServerInfo::new(peer, domain_name, port));
	}

	fn apply_login_request(&mut self, peer: PeerHandle, name: String) {
		if let Some(client) = self.clients.iter_mut().find(|c| c.peer == peer) { // change name, needs no LoginResponsePacket!
			client.name = name;
			client.last_request_counter = 0;
		} else { // add new client
			println!("INFO: new client connected: {}", name);
			self.clients.push(ClientInfo::new(peer, &name));
		}
	}
}

impl ClientInfo {
	fn new(peer: PeerHandle, name: &str) -> ClientInfo {
		ClientInfo {
			peer,
			name: String::from(name),
			last_request_counter: 0,
		}
	}
}

impl GameServerInfo {
	pub fn new(peer: PeerHandle, domain_name: String, port: u16) -> GameServerInfo {
		GameServerInfo {
			peer,
			domain_name,
			port,
		}
	}
}

fn main() {
	let matches = ClapApp::new("Elements Master Server")
		.about("This is the Master Server of the Elements Game. Lets connect some clients with games :D")
		.arg(Arg::with_name("port")
			.short("-p")
			.long("--port")
			.value_name("PORT")
			.help(&format!("The server will bind this port. (default: {})", DEFAULT_MASTER_SERVER_PORT))
			.takes_value(true)
		)
		.arg(Arg::with_name("identity_file")
			.short("-i")
			.long("--identity-file")
			.value_name("IDENTITY_FILE")
			.help(&"The identity file for tls. If not given https is not supported")
			.takes_value(true)
		)
		.get_matches();

	let port = matches.value_of("port")
		.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
		.unwrap_or(DEFAULT_MASTER_SERVER_PORT);

	let identity_file = matches.value_of("identity_file");

	MasterServer::new(port, identity_file).run();
}
