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
		self.client_world.fetch_keyboard_updates(&app.input_receiver);
		for (i, input_device) in self.input_devices.iter_mut().enumerate() {
			self.client_world.world.players[i].input.update_gamepad(&input_device.get_state(&app.gilrs));
		}
		self.client_world.world.players.last_mut().unwrap().input.update_keyboard(&self.client_world.keyboard_state);
		self.client_world.world.players.last_mut().unwrap().input.update_cursor(&self.client_world.keyboard_state.cursor_move);
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		// self.client_world.draw(app, timed_loop_info);

		let world = &self.client_world.world;
		let graphics_world = GraphicsWorld::new(
			&world.tilemap,
			&world.fluidmap,
			world.players.clone(),
			timed_loop_info.elapsed_time,
		);
		app.graphics_sender.send(graphics_world).unwrap();
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::from_world(&self.client_world.world)
	}
}
