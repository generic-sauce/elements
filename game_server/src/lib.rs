use clap::{App as ClapApp, Arg, ArgMatches};

use world::prelude::*;

// update_desire is within 0..=1000
const UPDATE_DESIRE_PER_FRAME: u32 = 350;
const GAME_FPS: u32 = 60;
const MAX_SILENT_GAME_SECONDS: u32 = 3;

const JOIN_FPS: u32 = 10;
const MAX_SILENT_JOIN_SECONDS: u32 = 2*60;

const MASTER_SERVER_FRAME_INTERVAL: u32 = 50;

pub struct Server {
	world: World,
	update_desire: [u32; 2],
	peer_manager: PeerManager,
	peers: [PeerHandle; 2],
	silent_frames: u32,
}

impl Server {
	pub fn new(port: u16, domain_name: Option<&str>, identity_file: Option<&str>) -> Server {

		println!("INFO: Server starting on port {}. Waiting for players.", port);

		let (peer_manager, peers, packet) = waiting_for_players(port, domain_name, identity_file);

		let tilemap_image = load_tilemap_image(&format!("map/{}", AVAILABLE_MAPS[packet.map_id as usize]));

		let mut server = Server {
			world: World::new(0, &tilemap_image),
			update_desire: [0, 0],
			peer_manager,
			peers,
			silent_frames: 0,
		};

		for (i, p) in server.peers.iter().enumerate() {
			let go = GameSCPacket::Go {
				your_player_id: i,
				tilemap_image: tilemap_image.clone(),
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
						self.update_desire[0] += diff;
						self.update_desire[1] += diff;
						self.world.players[i].input = input_state;
						self.world.players[i].input.clamp();
						self.silent_frames = 0;
					},
					PeerEvent::NewPeer(_) => println!("new player joined while game is already running!"),
					PeerEvent::Disconnect(_) => println!("a player disconnected!"),
				}
			}

			self.world.tick(&mut ());

			// send game update
			for i in 0..2 {
				self.update_desire[i] += UPDATE_DESIRE_PER_FRAME;
				if self.update_desire[i] >= 1000 {
					self.update_desire[i] = 0;
					let update = GameSCPacket::WorldUpdate(self.world.update());
					if let Err(x) = self.peer_manager.send_to(self.peers[i], &update) {
						eprintln!("game-server: error: can't send GameSCPacket::WorldUpdate to some client! {}", x);
					}
				}
			}

			self.silent_frames += 1;

			if self.silent_frames > MAX_SILENT_GAME_SECONDS*GAME_FPS {
				panic!("No game packets received for {} seconds! Shutting down...", MAX_SILENT_GAME_SECONDS);
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
fn waiting_for_players(port: u16, domain_name: Option<&str>, identity_file: Option<&str>) -> (PeerManager, [PeerHandle; 2], MasterToGameServerGoPacket) {
	let mut go_packet: Option<MasterToGameServerGoPacket> = None;
	let mut peer_manager = PeerManager::new(port, port+1, identity_file);

	let mut silent_frames = 0;
	let mut packet_send_counter = 0;

	let mut master_socket: Option<(&str, NativeSocketBackend)> = domain_name.and_then(|d| {
		let master_socket = match NativeSocketBackend::new(DEFAULT_MASTER_SERVER_HOSTNAME, DEFAULT_MASTER_SERVER_PORT) {
			Ok(x) => x,
			Err(x) => {
				eprintln!("game server can't connect to master server: {}", x);
				return None;
			}
		};
		Some((d, master_socket))
	});

	match master_socket {
		Some(_) => println!("INFO: connecting to master server"),
		None => println!("INFO: NOT connecting to master server as domain name is not specified"),
	}

	for _ in TimedLoop::with_fps(JOIN_FPS) {
		let old_cnt = peer_manager.get_peer_handles().len();
		let evs = peer_manager.tick::<()>();
		let cnt = peer_manager.get_peer_handles().len();

		for ev in evs {
			match ev {
				PeerEvent::NewPeer(_) => {
					println!("INFO: new player joined!");
					silent_frames = 0;
				},
				PeerEvent::ReceivedPacket(..) => println!("received packet before game start!"),
				PeerEvent::Disconnect(_) => println!("INFO: player disconnected!"),
			}
		}

		if cnt != old_cnt {
			packet_send_counter = 0;
		}

		match cnt {
			0 => {},
			1 => {
				silent_frames += 1;
				if silent_frames > MAX_SILENT_JOIN_SECONDS*JOIN_FPS {
					panic!("WARN: Missing second player. Timeout! Shutting down...");
				}
			},
			_ => { // enough players joined!
				if go_packet.is_some() { break; }
				else { println!("WARN: waiting for MasterToGameServerGoPacket"); }
			},
		}

		// master server networking
		if let Some((domain_name, socket)) = &mut master_socket {
			socket.tick();
			loop {
				match socket.recv::<MasterToGameServerGoPacket>() {
					Ok(None) => break,
					Ok(Some(x)) => {
						go_packet = Some(x);
					}
					Err(x) => eprintln!("game-server: waiting_for_players: error: {}", x),
				}
			}

			if packet_send_counter == 0 {
				update_master_server(socket, cnt as u32, port, domain_name);
				packet_send_counter = 0;
			}
			packet_send_counter = (packet_send_counter + 1) % MASTER_SERVER_FRAME_INTERVAL;
		}
	}

	let peers = peer_manager.get_peer_handles();
	let peers = [peers[0], peers[1]];

	(peer_manager, peers, go_packet.expect("this is a bug. waiting_for_players finished without receiving a MasterToGameServerGoPacket"))
}

fn update_master_server(socket: &mut NativeSocketBackend, num_players: u32, port: u16, domain_name: &str) {
	let master_server_packet = MasterServerPacket::GameServerStatusUpdate {
		domain_name: String::from(domain_name),
		num_players,
		port,
	};
	if socket.send(&master_server_packet).is_err() {
		println!("WARN: failed to inform master server!");
	}
}
