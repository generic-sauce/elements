use crate::prelude::*;

pub struct Server {
	world: World,
	socket: UdpSocket,
	peers: [SocketAddr; 2],
	input_states: [InputState; 2],
}

impl Server {
	pub fn new() -> Server {
		let mut socket = UdpSocket::bind("127.0.0.1:7575").expect("Could not create server socket");
		socket.set_nonblocking(true).unwrap();

		let peers = wait_for_players(&mut socket);

		Server {
			world: World::new(),
			socket,
			peers,
			input_states: [InputState::new(), InputState::new()]
		}
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (_elapsed_time, delta_time, _fps, _load) in timed_loop {
			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			// receive packets
			while let Some((input_state, recv_addr)) = recv_packet(&mut self.socket) {
				let mut index: i32 = -1;
				for i in 0i32..2i32 {
					if recv_addr == self.peers[i as usize] {
						index = i;
					}
				}
				if index == -1 {
					eprintln!("got packet from {}, which is not a known peer", recv_addr);
				} else {
					self.input_states[index as usize] = input_state;
				}
			}

			self.tick();

			// send game update
			for (i, peer) in self.peers.iter().enumerate() {
				let update = Update::from_tuple(i, self.input_states[1-i].clone(), &self.world);
				send_packet_to(&mut self.socket, &update, *peer);
			}

			self.check_restart();
		}
	}

	pub fn check_restart(&mut self) {
		// TODO
	}

	fn tick(&mut self) {
		self.world.tick(&self.input_states);
	}
}

fn wait_for_players(socket: &mut UdpSocket) -> [SocketAddr; 2] {
	let mut peers = vec!();

	for _ in TimedLoop::with_fps(10) {
		if let Some((Init::Init, recv_addr)) = recv_packet(socket) {
			peers.push(recv_addr);
			if peers.len() == 2 {
				break;
			}
		}
	}

	return [peers[0], peers[1]];
}
