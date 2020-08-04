mod player;
mod tilemap;

use sfml::system::Vector2f;
use sfml::graphics::RenderWindow;

use crate::input::{Input, KeyboardInput};
use crate::world::player::Player;
use crate::world::tilemap::TileMap;
use crate::texture_state::TextureState;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub input: Box<dyn Input>, // TODO move this to App
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vector2f::new(0.0, 0.0)), Player::new(Vector2f::new(20.0, 0.0))],
			tilemap: TileMap::new("res/todo-add-path"),
			input: Box::new(KeyboardInput::new()),
		}
	}

	pub fn tick(&mut self) {
		self.handle_local_player();
		for p in self.players.iter_mut() {
			p.tick();
		}
	}

	pub fn render(&mut self, w: &mut RenderWindow, texture_state: &TextureState) {
		for p in self.players.iter_mut() {
			p.render(w, texture_state);
		}
	}

	fn handle_local_player(&mut self) {
		self.players[0].direction = self.input.get_direction();
	}
}
