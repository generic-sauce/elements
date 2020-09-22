use crate::prelude::*;

pub struct Local {
	app: App,
	inputs: [InputDevice; 2],
}

impl Local {
	pub fn new() -> Local {
		let app = App::new();
		let inputs = [InputDevice::new_adaptive(0, false, &app.gilrs), InputDevice::new_adaptive(1, true, &app.gilrs)];

		Local {
			app,
			inputs,
		}
	}

	pub fn run(&mut self) {
		unimplemented!();
	}
}
