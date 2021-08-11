use clap::{App as ClapApp, Arg, ArgMatches};

use world::prelude::*;

// update_desire is within 0..=1000
const UPDATE_DESIRE_PER_FRAME: u32 = 350;
const GAME_FPS: u32 = 60;

const JOIN_FPS: u32 = 10;

pub struct Server {
	world: World,
	update_desire: Vec<u32>,
	peer_manager: PeerManager,
	peers: Vec<PeerHandle>,
}

impl Server {
	pub fn new(port: u16, domain_name: Option<&str>, identity_file: Option<&str>) -> Server {
		println!("INFO: Server starting on port {}. Waiting for players.", port);

		let (peer_manager, peers, packet) = waiting_for_players(port, domain_name, identity_file);

		let tilemap_image = load_tilemap_image(&format!("map/{}", AVAILABLE_MAPS[packet.map_id as usize]));

		let mut server = Server {
			world: World::new(0, &tilemap_image, &packet.teams[..]),
			update_desire: vec![0; peers.len()],
			peer_manager,
			peers,
		};

		for (i, p) in server.peers.iter().enumerate() {
			let go = GameSCPacket::Go {
				your_player_id: i,
				tilemap_image: tilemap_image.clone(),
				teams: packet.teams.clone(),
			};
			if let Err(x) = server.peer_manager.send_to(*p, &go) {
				eprintln!("game-server: error: can't send GameSCPacket::Go to some client \"{}\"", x);
			}
		}

		server
	}

	pub fn run(&mut self) {
		println!("INFO: Game started");

		for timed_loop_info in TimedLoop::with_fps(GAME_FPS) {
			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("WARN: Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			// receive packets
			for ev in self.peer_manager.tick::<GameCSPacket>() {
				match ev {
					PeerEvent::ReceivedPacket(GameCSPacket::InputState(input_state), p) => {
						let i = match self.peers.iter().position(|p2| *p2 == p) {
							Some(i) => i,
							None => {
								println!("Got packet from external player");
								continue
							}
						};

						let diff = self.world.players[i].input.diff(&input_state);
						for x in &mut self.update_desire { *x += diff; }
						self.world.players[i].input = input_state;
						self.world.players[i].input.clamp();
					},
					PeerEvent::NewPeer(_) => println!("new player joined while game is already running!"),
					PeerEvent::Disconnect(_) => println!("a player disconnected!"),
				}
			}

			self.world.tick(&mut ());

			// send game update
			for i in 0..self.peers.len() {
				self.update_desire[i] += UPDATE_DESIRE_PER_FRAME;
				if self.update_desire[i] >= 1000 {
					self.update_desire[i] = 0;
					let update = GameSCPacket::WorldUpdate(self.world.update());
					if let Err(x) = self.peer_manager.send_to(self.peers[i], &update) {
						eprintln!("game-server: error: can't send GameSCPacket::WorldUpdate to some client! {}", x);
					}
				}
			}

		}
	}
}

pub fn game_server_cli_args() -> ArgMatches<'static> {
	ClapApp::new("Elements Game Server")
		.about("This is the Game Server of the Elements Game. Lets host some game :D")
		.arg(Arg::with_name("port")
			.short("-p")
			.long("--port")
			.value_name("PORT")
			.help(&format!("The server will bind this port. (default: {})", DEFAULT_MASTER_SERVER_PORT))
			.takes_value(true)
		)
		.arg(Arg::with_name("domain_name")
			.short("-d")
			.long("--domain-name")
			.value_name("DOMAIN_NAME")
			.help(&"The domain name of this server. Only used, if connecting to a master server.")
			.takes_value(true)
		)
		.arg(Arg::with_name("identity_file")
			.short("-i")
			.long("--identity-file")
			.value_name("IDENTITY_FILE")
			.help(&"The identity file for tls. If not given https is not supported")
			.takes_value(true)
		)
		.get_matches()
}

// waits for all players to join, and for some MasterToGameServerGoPacket to be received
fn waiting_for_players(port: u16, domain_name: Option<&str>, identity_file: Option<&str>) -> (PeerManager, Vec<PeerHandle>, MasterToGameServerGoPacket) {
	let mut go_packet: Option<MasterToGameServerGoPacket> = None;
	let mut peer_manager = PeerManager::new(port, port+1, identity_file);

	let mut master_socket = NativeSocketBackend::new(DEFAULT_MASTER_SERVER_HOSTNAME, DEFAULT_MASTER_SERVER_PORT).expect("can't connect to master server");

	master_socket.send(&MasterServerPacket::GameServerReady {
		domain_name: domain_name.expect("domain name not specified").to_string(),
		port,
	}).expect("can't send GameServerReady packet");

	for _ in TimedLoop::with_fps(JOIN_FPS) {
		let evs = peer_manager.tick::<()>();

		for ev in evs {
			match ev {
				PeerEvent::NewPeer(_) => println!("INFO: new player joined!"),
				PeerEvent::ReceivedPacket(..) => println!("received packet before game start!"),
				PeerEvent::Disconnect(_) => println!("INFO: player disconnected!"),
			}
		}

		if let Some(p) = &go_packet {
			if peer_manager.get_peer_handles().len() >= p.teams.len() {
				let peers: Vec<_> = peer_manager.get_peer_handles()
					.iter()
					.cloned()
					.take(p.teams.len())
					.collect();

				return (peer_manager, peers, p.clone());
			}
		}

		// master server networking
		master_socket.tick();
		loop {
			match master_socket.recv::<MasterToGameServerGoPacket>() {
				Ok(None) => break,
				Ok(Some(x)) => { go_packet = Some(x); },
				Err(x) => eprintln!("game-server: waiting_for_players: error: {}", x),
			}
		}
	}
	unreachable!()
}
