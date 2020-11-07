use crate::prelude::*;

mod peer;
pub use peer::*;

// update_desire is within 0..=1000
const UPDATE_DESIRE_PER_FRAME: u32 = 350;

pub struct Server {
	world: World,
	update_desire: [u32; 2],
	peer_manager: PeerManager,
}

impl Server {
	pub fn new() -> Server {
		let mut tilemap_image = TileMapImage::new(DEFAULT_TILEMAP);

		let mut server = Server {
			world: World::new(0, &tilemap_image),
			update_desire: [0, 0],
			peer_manager: PeerManager::wait_for_players(),
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
		println!("server has started!");

		for timed_loop_info in TimedLoop::with_fps(60) {
			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			// receive packets
			while let Some((input_state, i)) = self.peer_manager.recv_from() {
				let diff = self.world.players[i].input.diff(&input_state);
				self.update_desire[0] += diff;
				self.update_desire[1] += diff;
				self.world.players[i].input = input_state;
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
		}
	}
}
