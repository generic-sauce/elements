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
		alert(&self.world.fluidmap.iter().count().to_string());
		let x = wasm_bindgen::JsValue::from_serde(&self.world).unwrap();
		draw_world(&x);
	}
}

pub fn run(mut runnable: Local) {
	alert("pre!");
	loop {
		// TODO force correct fps
		runnable.tick();
		runnable.draw();

		/* TODO
		if !self.window.is_open() {
			std::process::exit(0);
		}
		*/
	};
}

