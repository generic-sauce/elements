use crate::prelude::*;

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
	silent_frames: u32,
	port: u16,
}

impl Server {
	pub fn new(port: u16) -> Server {
		let mut tilemap_image = TileMapImage::new(DEFAULT_TILEMAP);

		println!("INFO: Server starting on port {}. Waiting for players.", port);

		let mut server = Server {
			world: World::new(0, &tilemap_image),
			update_desire: [0, 0],
			peer_manager: waiting_for_players(port),
			silent_frames: 0,
			port,
		};

		for i in 0..2 {
			let go = Go {
				your_player_id: i,
				tilemap_image,
			};
			server.peer_manager.send_to(i, &go);

			tilemap_image = go.tilemap_image;
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
			for ev in self.peer_manager.tick::<InputState>() {
				match ev {
					PeerEvent::ReceivedPacket(input_state, i) => {
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
					let update = self.world.update();
					self.peer_manager.send_to(i, &update);
				}
			}

			self.silent_frames += 1;

			if self.silent_frames > MAX_SILENT_GAME_SECONDS*GAME_FPS {
				panic!("No game packets received for {} seconds! Shutting down...", MAX_SILENT_GAME_SECONDS);
			}
		}
	}
}

fn waiting_for_players(port: u16) -> PeerManager {
	let mut peer_manager = PeerManager::new(port, port+1);

	let mut silent_frames = 0;
	let mut packet_send_counter = 0;

	println!("INFO: connecting to master server");
	let mut socket = NativeSocketBackend::new("generic-sauce.de", MASTER_SERVER_PORT);

	for _ in TimedLoop::with_fps(JOIN_FPS) {
		let events = peer_manager.tick::<InputState>(); // TODO this generic parameter is unused!
		let cnt = peer_manager.count();
		for ev in events {
			match ev {
				PeerEvent::NewPeer(_) => {
					update_master_server(&mut socket, cnt as u32, port);
					println!("INFO: new player joined!");
					silent_frames = 0;
				},
				PeerEvent::ReceivedPacket(..) => println!("received packet before game start!"),
				PeerEvent::Disconnect(_) => unimplemented!("handle disconnect"), // TODO
			}
		}

		if cnt == 2 {
			break;
		}

		if cnt > 0 {
			silent_frames += 1;
			if silent_frames > MAX_SILENT_JOIN_SECONDS*JOIN_FPS {
				panic!("WARN: Missing second player. Timeout! Shutting down...");
			}
		}

		// master server networking
		if packet_send_counter == 0 {
			update_master_server(&mut socket, cnt as u32, port);
			packet_send_counter = 0;
		}
		packet_send_counter = (packet_send_counter + 1) % MASTER_SERVER_FRAME_INTERVAL;
	}

	peer_manager
}

fn update_master_server(socket: &mut NativeSocketBackend, num_players: u32, port: u16) {
	let master_server_packet = MasterServerPacket::GameServerStatusUpdate {
		domain_name: String::from("generic-sauce.de"), // TODO: make configurable
		num_players,
		port,
	};
	if socket.send(&master_server_packet).is_err() {
		println!("WARN: failed to inform master server!");
	}
}