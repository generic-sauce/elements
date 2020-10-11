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
		let x = wasm_bindgen::JsValue::from_serde(&self.world).unwrap();
		let y = wasm_bindgen::JsValue::from_serde(&Constants::new()).unwrap();
		draw_world(&x, y);
	}
}

#[wasm_bindgen]
pub fn work_local(runnable: *mut Local) {
	unsafe {
		(*runnable).tick();
		(*runnable).draw();
	}
}
