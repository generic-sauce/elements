use crate::prelude::*;

pub struct Local<B: Backend> {
	world: World,
	phantom: PhantomData<B>,
}

impl<B: Backend> Local<B> {
	pub fn new(best_of_n: u32) -> Local<B> {
		Local {
			world: World::new(best_of_n),
			phantom: PhantomData,
		}
	}
}

impl<B: Backend> Runnable<B> for Local<B> {
	fn tick(&mut self, app: &mut App<B>) {
		for (i, player) in self.world.players.iter_mut().enumerate() {
			player.input.update_gamepad(&app.input_backend.gamepad(i as u32));
		}
		self.world.players.last_mut().unwrap().input.update_peripherals(&app.peripherals_state);
		self.world.tick_within_app(app);
	}

	fn draw(&mut self, app: &mut App<B>, timed_loop_info: &TimedLoopInfo) {
		let mut draw = Draw::new(timed_loop_info.elapsed_time);
		self.world.draw(&mut draw);
		app.graphics_backend.draw(draw);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::from_world(&self.world)
	}
}