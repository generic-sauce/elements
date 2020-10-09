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

	fn draw(&mut self) {
		// TODO
	}
}

pub fn run(mut runnable: Local) {
	alert("pre!");
	loop {
		// TODO force correct fps
		alert("test!");

		runnable.tick();
		runnable.draw();

		/* TODO
		if !self.window.is_open() {
			std::process::exit(0);
		}
		*/
	};
}

