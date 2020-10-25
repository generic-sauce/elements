use crate::prelude::*;

pub struct Local {
	input_devices: [InputDevice; 2],
	client_world: ClientWorld,
}

impl Local {
	pub fn new(gilrs: &Gilrs, best_of_n: u32) -> Local {
		let input_devices = [InputDevice::new(0, gilrs), InputDevice::new(1, gilrs)];

		Local {
			input_devices,
			client_world: ClientWorld::new(best_of_n),
		}
	}
}

impl Runnable for Local {
	fn tick(&mut self, app: &mut App) {
		for (i, input) in self.input_devices.iter_mut().enumerate() {
			self.client_world.world.players[i].input.update(&input.get_state(&app.gilrs));
		}
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		self.client_world.draw(app, timed_loop_info);

		let world = &self.client_world.world;
		let graphics_world = GraphicsWorld::new(
			&world.tilemap,
			&world.fluidmap,
			world.players.clone(),
			timed_loop_info.elapsed_time,
		);
		app.sender.send(graphics_world).unwrap();
	}

	fn apply_key(&mut self, _ev: &KeyPressedEvent) {}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::from_world(&self.client_world.world)
	}
}
