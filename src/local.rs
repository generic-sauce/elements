use crate::prelude::*;

pub enum LocalMode<B: Backend> {
	LoadingTileMap {
		loader: B::TileMapLoaderBackend,
		best_of_n: u32,
	},
	InGame(World),
}

pub struct Local<B: Backend> {
	pub mode: LocalMode<B>,
}

impl<B: Backend> Local<B> {
	pub fn new(best_of_n: u32) -> Local<B> {
		Local {
			mode: LocalMode::LoadingTileMap {
				loader: B::TileMapLoaderBackend::new(DEFAULT_TILEMAP),
				best_of_n
			}
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match &mut self.mode {
			LocalMode::LoadingTileMap { loader, best_of_n } => {
				if let Some(image) = loader.poll() {
					self.mode = LocalMode::InGame(World::new(*best_of_n, &image));
				}
			},
			LocalMode::InGame(world) => {
				for (i, player) in world.players.iter_mut().enumerate() {
					player.input.update_gamepad(&app.input_backend.gamepad(i as u32));
				}
				world.players.last_mut().unwrap().input.update_peripherals(&app.peripherals_state);
				world.tick_within_app(app);
			}
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		match &self.mode {
			LocalMode::LoadingTileMap { .. } => {} // TODO
			LocalMode::InGame(world) => {
				let mut draw = Draw::new();
				world.draw(&mut draw);
				app.graphics_backend.draw(draw, Some(world));
			}
		}
	}
}
