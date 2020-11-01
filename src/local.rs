use crate::prelude::*;

pub struct Local<B: Backend> {
	client_world: ClientWorld<B>,
}

impl<B: Backend> Local<B> {
	pub fn new(best_of_n: u32) -> Local<B> {
		Local {
			client_world: ClientWorld::new(best_of_n),
		}
	}
}

impl<B: Backend> Runnable<B> for Local<B> {
	fn tick(&mut self, app: &mut App<B>) {
		for (i, player) in self.client_world.world.players.iter_mut().enumerate() {
			player.input.update_gamepad(&app.input_backend.gamepad(i as u32));
		}
		self.client_world.world.players.last_mut().unwrap().input.update_peripherals(&app.peripherals_state);
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App<B>, timed_loop_info: &TimedLoopInfo) {
		let mut draw = Draw::new(timed_loop_info.elapsed_time);
		self.client_world.draw(&mut draw);
		app.graphics_backend.draw(draw);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::from_world(&self.client_world.world)
	}
}
