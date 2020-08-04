mod player;

use sfml::system::Vector2f;
use sfml::graphics::RenderWindow;

use crate::input::{Input, KeyboardInput, GamePadInput};
use crate::world::player::Player;
use crate::texture_state::TextureState;

pub struct World {
	pub players: [Player; 2],
	pub input: Box<dyn Input>,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vector2f::new(400.0, 400.0)), Player::new(Vector2f::new(400.0, 400.0))],
			input: Box::new(GamePadInput::new(0)),
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
		if !self.input.is_connected() {
			println!("joystick not connected");
		}
		self.players[0].direction = self.input.get_direction();
	}
}
