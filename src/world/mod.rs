mod player;

use sfml::system::Vector2f;
use sfml::graphics::RenderWindow;

use crate::input::{Input, KeyboardInput};
use crate::world::player::Player;

pub struct World {
	pub players: [Player; 2],
    pub input: Box<dyn Input>,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vector2f::new(0.0, 0.0)), Player::new(Vector2f::new(20.0, 0.0))],
			input: Box::new(KeyboardInput::new()),
		}
	}

	pub fn tick(&mut self) {
		self.handle_local_player();
		for p in self.players.iter_mut() {
			p.tick();
		}
		// println!("local player {} {}", self.players[0].position.x, self.players[0].position.y);
	}

	pub fn render(&mut self, w: &mut RenderWindow) {
		for p in self.players.iter_mut() {
			p.render(w);
		}
	}

	fn handle_local_player(&mut self) {
		self.players[0].direction = self.input.get_direction();
	}
}
