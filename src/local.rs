use crate::prelude::*;

pub struct Local {
	inputs: [InputDevice; 2],
	client_world: ClientWorld,
}

impl Local {
	pub fn new() -> Local {
		let app = App::new();
		let inputs = [InputDevice::new_adaptive(0, false, &app.gilrs), InputDevice::new_adaptive(1, true, &app.gilrs)];

		Local {
			inputs,
			client_world: ClientWorld::new(),
		}
	}
}

impl Runnable for Local {
	fn tick(&mut self, app: &mut App) {
		for (i, input) in self.inputs.iter_mut().enumerate() {
			self.client_world.world.players[i].input = input.update(&app.gilrs);
		}
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App, elapsed_time: Duration, fps: u32, load: f32) {
		self.client_world.draw(app, elapsed_time, fps, load);
	}
}