use crate::prelude::*;

use std::net::TcpListener;

// update_desire is within 0..=1000
const UPDATE_DESIRE_PER_FRAME: u32 = 350;

pub struct Server {
	world: World,
	peers: [TungSocket; 2],
	update_desire: [u32; 2],
}

impl Server {
	pub fn new() -> Server {
		let mut peers = wait_for_players();

		for (i, peer) in peers.iter_mut().enumerate() {
			send_packet_to(peer, &Go { your_player_id: i });
		}

		Server {
			world: World::new_defaultmap(0),
			peers,
			update_desire: [0, 0],
		}
	}

	pub fn run(&mut self) {
		for timed_loop_info in TimedLoop::with_fps(60) {
			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			// receive packets
			for peer in 0..2 {
				while let Some(input_state) = recv_packet(&mut self.peers[peer]) {
					let diff = self.world.players[peer].input.diff(&input_state);
					self.update_desire[0] += diff;
					self.update_desire[1] += diff;
					self.world.players[peer].input = input_state;
				}
			}
			self.world.tick(&mut ());

			// send game update
			for i in 0..2 {
				self.update_desire[i] += UPDATE_DESIRE_PER_FRAME;
				if self.update_desire[i] >= 1000 {
					self.update_desire[i] = 0;
					let update = self.world.update();
					send_packet_to(&mut self.peers[i], &update);
				}
			}
		}
	}
}

fn wait_for_players() -> [TungSocket; 2] {
    let server = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();
    let mut peers: Vec<_> = server.incoming()
		.take(2)
		.map(|stream_res| {
			let stream = stream_res.unwrap();
			tungstenite::server::accept(stream).unwrap()
		}).collect();
	println!("starting!");
	[peers.remove(0), peers.remove(0)]
}
