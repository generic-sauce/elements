use crate::prelude::*;

// update_desire is within 0..=1000
const UPDATE_DESIRE_PER_FRAME: u32 = 350;
const GAME_FPS: u32 = 60;
const MAX_SILENT_GAME_SECONDS: u32 = 3;

const JOIN_FPS: u32 = 10;
const MAX_SILENT_JOIN_SECONDS: u32 = 2*60;

pub struct Server {
	world: World,
	update_desire: [u32; 2],
	peer_manager: PeerManager,
	silent_frames: u32,
}

impl Server {
	pub fn new() -> Server {
		let mut tilemap_image = TileMapImage::new(DEFAULT_TILEMAP);

		println!("Server started. Waiting for players.");

		let mut server = Server {
			world: World::new(0, &tilemap_image),
			update_desire: [0, 0],
			peer_manager: waiting_for_players(),
			silent_frames: 0,
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
		println!("Game has started!");

		for timed_loop_info in TimedLoop::with_fps(GAME_FPS) {
			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			// receive packets
			while let Some((input_state, i)) = self.peer_manager.recv_from() {
				let diff = self.world.players[i].input.diff(&input_state);
				self.update_desire[0] += diff;
				self.update_desire[1] += diff;
				self.world.players[i].input = input_state;
				self.world.players[i].input.clamp();
				self.silent_frames = 0;
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

fn waiting_for_players() -> PeerManager {
	let mut peer_manager = PeerManager::new(PORT, HTTPS_PORT);

	let mut silent_frames = 0;
	let mut packet_send_counter = 0;

	println!("creating master server socket");
	let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket for master server connection");
	socket.set_nonblocking(true).unwrap();
	socket.connect(("127.0.0.1", MASTER_SERVER_PORT)).expect("Could not connect to master server");

	send_packet(&mut socket, &Init::Init);

	for _ in TimedLoop::with_fps(JOIN_FPS) {
		let prev_cnt = peer_manager.count();
		peer_manager.accept();
		let cnt = peer_manager.count();

		if cnt > prev_cnt { // a new peer!
			println!("a new player joined!");
			if cnt == 2 {
				break;
			}
			silent_frames = 0;
		} else if cnt > 0 { // if already a player is waiting..
			silent_frames += 1;
			if silent_frames > MAX_SILENT_JOIN_SECONDS*JOIN_FPS {
				panic!("No more players joined! Shutting down...");
			}
		}

		// master server networking
		packet_send_counter += 1;
		if packet_send_counter > 1 {
			println!("sending master server packet");
			send_packet(&mut socket, &GameServerStatusUpdate { num_players: 42 as u32 });
			packet_send_counter = 0;
		}
	}

	while let Some((p, _)) = peer_manager.recv_from::<Init>() {
		assert!(matches!(p, Init::Init));
	}

	peer_manager
}