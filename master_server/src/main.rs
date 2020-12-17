#![feature(drain_filter)]

use server::peer::{PeerManager, PeerHandle, PeerEvent};
use networking::prelude::{MASTER_SERVER_PORT, MASTER_SERVER_HTTPS_PORT, MasterServerPacket, MasterClientPacket};
use native_utils::prelude::TimedLoop;

pub const MASTER_SERVER_FPS: u32 = 10;
pub const AWAITING_TIMEOUT: u32 = 5 * MASTER_SERVER_FPS;
pub const CLIENT_REQUEST_TIMEOUT: u32 = 5 * MASTER_SERVER_FPS;

pub struct MasterServer {
    pub peer_manager: PeerManager,
    pub game_servers: Vec<GameServerInfo>,
    pub clients: Vec<ClientInfo>,
}

pub struct GameServerInfo {
    pub peer: PeerHandle,
    pub domain_name: String,
    pub num_players: u32,
    pub state: GameServerState,
    pub port: u16,
}

pub struct ClientInfo {
    pub peer: PeerHandle,
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
            for ev in self.peer_manager.tick::<MasterServerPacket>() {
                match ev {
                    PeerEvent::ReceivedPacket(MasterServerPacket::GameServerStatusUpdate { domain_name, num_players, port }, peer) => {
                        self.apply_game_server_status_update(peer, domain_name, num_players, port);
                    }
                    PeerEvent::ReceivedPacket(MasterServerPacket::ClientRequest { name }, peer) => {
                        self.apply_client_request(peer, name);
                    }
                    PeerEvent::NewPeer(_) => {},
                    PeerEvent::Disconnect(peer) => {
                        for removed_peer in self.game_servers.drain_filter(|gs| gs.peer == peer) {
                            println!("INFO: game server disconnected: {}:{}", removed_peer.domain_name, removed_peer.port);
                        }
                        for removed_client in self.clients.drain_filter(|gs| gs.peer == peer) {
                            println!("INFO: client disconnected: {}", removed_client.name);
                        }
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

    fn apply_game_server_status_update(&mut self, peer: PeerHandle, domain_name: String, num_players: u32, port: u16) {
        if let Some(game_server) = self.game_servers.iter_mut().find(|gs| gs.peer == peer) {
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
            println!("INFO: new game server connected: {}:{}", domain_name, port);
            self.game_servers.push(GameServerInfo::new(peer, domain_name, num_players, port));
            self.check_game_start();
        }
    }

    fn apply_client_request(&mut self, peer: PeerHandle, name: String) {
        if let Some(client) = self.clients.iter_mut().find(|c| c.peer == peer) {
            client.name = name;
            client.last_request_counter = 0;
            if matches!(client.state, ClientState::Disconnected) {
                println!("INFO: reactivating client: {}", client.name);
                client.state = ClientState::Ready;
            }
        } else {
            println!("INFO: new client connected: {}", name);
            self.clients.push(ClientInfo::new(peer, &name));
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
        println!("INFO: initiating game with players: {}, {}\t server addr: {}:{}", clients[0].name, clients[1].name, &game_server.domain_name, game_server.port);
        for client in clients {
            peer_manager.send_to(client.peer, &MasterClientPacket::GameRedirection(game_server.domain_name.clone(), game_server.port));
            game_server.state = GameServerState::AwaitingGame(0);
            client.state = ClientState::InGame;
        }
    }
}

impl ClientInfo {
    fn new(peer: PeerHandle, name: &str) -> ClientInfo {
        ClientInfo {
            peer,
            name: String::from(name),
            state: ClientState::Ready,
            last_request_counter: 0,
        }
    }
}

impl GameServerInfo {
    pub fn new(peer: PeerHandle, domain_name: String, num_players: u32, port: u16) -> GameServerInfo {
        GameServerInfo {
            peer,
            domain_name,
            num_players,
            state: GameServerState::Ready,
            port,
        }
    }
}

fn main() {
    MasterServer::new().run();
}
