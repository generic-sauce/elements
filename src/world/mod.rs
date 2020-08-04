mod player;

use sfml::system::Vector2f;
use sfml::graphics::RenderWindow;

use crate::world::player::Player;
use crate::texture_state::TextureState;

pub struct World {
	pub players: [Player; 2],
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vector2f::new(0.0, 0.0)), Player::new(Vector2f::new(20.0, 0.0))],
		}
	}

	pub fn tick(&mut self) {
		for p in self.players.iter_mut() {
			p.tick();
		}
	}

	pub fn render(&mut self, _w: &mut RenderWindow, _texture_state: &TextureState) {
		for _p in self.players.iter_mut() {
			// p.render(w); // TODO
		}
	}
}
