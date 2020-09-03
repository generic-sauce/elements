use crate::prelude::*;

pub struct Client {
	app: App,
	input: InputDevice,
	socket: UdpSocket,
	player_id: usize,
}

impl Client {
	pub fn new(server_ip: &str) -> Client {
		let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, PORT)).expect("Could not connect to server");

		send_packet(&mut socket, &Init::Init);

		let player_id = loop {
			// TODO maybe add a sleep here?
			if let Some((Go { your_player_id }, _)) = recv_packet::<Go>(&mut socket) {
				break your_player_id;
			}
		};

		let app = App::new();
		let input = InputDevice::new_adaptive(0, true, &app.gilrs);

		Client {
			app,
			input,
			socket,
			player_id,
		}
	}

	pub fn run(&mut self) {
		self.app.sound_manager.play_music(SoundId::APart);
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (elapsed_time, delta_time, fps, load) in timed_loop {
			while let Some(event) = self.app.window.poll_event() {
				match event {
					Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
						self.app.window.close();
						std::process::exit(0);
					}
					_ => {},
				}
			}

			if let Some((update, _)) = recv_packet::<WorldUpdate>(&mut self.socket) {
				let mut handler = AppEventHandler::new();
				self.app.world.apply_update(update, &mut handler);
				self.app.handle(&handler);
			}

			// process gilrs events
			while let Some(_) = self.app.gilrs.next_event() {}

			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			// inputs
			self.app.world.players[self.player_id].input = self.input.update(&self.app.gilrs);
			send_packet(&mut self.socket, &self.app.world.players[self.player_id].input);

			self.app.tick();
			self.app.draw(elapsed_time, fps, load);

			if !self.app.window.is_open() {
				std::process::exit(0);
			}
		}
	}
}
