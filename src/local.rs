use crate::prelude::*;

pub struct Local<B: Backend> {
	world: World,
	phantom: PhantomData<B>,
}

impl<B: Backend> Local<B> {
	pub fn new(best_of_n: u32) -> Local<B> {
		Local {
			world: World::new(best_of_n, DEFAULT_TILEMAP),
			phantom: PhantomData,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		for (i, player) in self.world.players.iter_mut().enumerate() {
			player.input.update_gamepad(&app.input_backend.gamepad(i as u32));
		}
		self.world.players.last_mut().unwrap().input.update_peripherals(&app.peripherals_state);
		self.world.tick_within_app(app);
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		let mut draw = Draw::new();
		self.world.draw(&mut draw);
		app.graphics_backend.draw(draw, Some(&self.world));
	}
}