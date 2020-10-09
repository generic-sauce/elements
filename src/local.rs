use crate::prelude::*;

pub struct Local {
	inputs: [InputDevice; 2],
	world: World,
}

impl Local {
	pub fn new() -> Local {
		let inputs = [InputDevice, InputDevice];

		Local {
			inputs,
			world: World::new(),
		}
	}

	fn tick(&mut self) {
		for (i, input) in self.inputs.iter_mut().enumerate() {
			self.world.players[i].input = input.update();
		}
		self.world.tick(&mut ());
	}

	fn draw(&mut self, _: &TimedLoopInfo) {
		// TODO
	}
}

pub fn run(mut runnable: Local) {
	for timed_loop_info in TimedLoop::with_fps(60) {
		alert("test!");
		if timed_loop_info.delta_time > timed_loop_info.interval {
			println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
		}

		runnable.tick();
		runnable.draw(&timed_loop_info);

		/* TODO
		if !self.window.is_open() {
			std::process::exit(0);
		}
		*/
	};
}

