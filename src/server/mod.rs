use crate::prelude::*;

pub struct Server {
	world: World,
	inputs: [Box<dyn Input>; 2],
	gilrs: gilrs::Gilrs, // TODO: remove for server
}

impl Server {
	pub fn new() -> Server {
		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");
		Server {
			world: World::new(),
			inputs: [Box::new(AdaptiveInput::new(0, &gilrs)), Box::new(AdaptiveInput::new(1, &gilrs))],
			gilrs,
		}
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (_elapsed_time, delta_time, _fps, _load) in timed_loop {
			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			// TODO: receive from clients
			self.tick();
			// TODO: send game update

			self.check_restart();
		}
	}

	pub fn check_restart(&mut self) {
		unimplemented!(); // TODO
	}

	fn tick(&mut self) {
		self.world.tick(&mut self.inputs, &self.gilrs);
	}
}
