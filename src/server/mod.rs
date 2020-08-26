use crate::prelude::*;

pub struct Server {
	world: World,
}

impl Server {
	pub fn new() -> Server {
		Server {
			world: World::new(),
		}
	}

	fn get_input_states(&self) -> [InputState; 2] {
		unimplemented!()
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (_elapsed_time, delta_time, _fps, _load) in timed_loop {
			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			// TODO: receive from clients

			// TODO: update inputs
			self.tick();
			// TODO: send game update

			self.check_restart();
		}
	}

	pub fn check_restart(&mut self) {
		unimplemented!(); // TODO
	}

	fn tick(&mut self) {
		self.world.tick(&self.get_input_states());
	}
}
