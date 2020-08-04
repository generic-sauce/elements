mod player;

use sfml::system::Vector2f;
use sfml::graphics::RenderWindow;

use crate::world::player::Player;

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
		self.handle_local_player();
		for p in self.players.iter_mut() {
			p.tick();
		}
		// println!("local player {} {}", self.players[0].position.x, self.players[0].position.y);
	}

	pub fn render(&mut self, _w: &mut RenderWindow) {
		for _p in self.players.iter_mut() {
			// p.render(w); // TODO
		}
	}

	fn handle_local_player(&mut self) {
		let mut direction = Vector2f::new(0.0, 0.0);
		if sfml::window::Key::W.is_pressed() {
			direction.y += 1.0;
		}
		if sfml::window::Key::S.is_pressed() {
			direction.y -= 1.0;
		}

		if sfml::window::Key::D.is_pressed() {
			direction.x += 1.0;
		}
		if sfml::window::Key::A.is_pressed() {
			direction.x -= 1.0;
		}
        self.players[0].direction = direction;
	}
}
