pub mod player;
pub mod tilemap;

use crate::prelude::*;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub input: Box<dyn Input>, // TODO move this to App
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vector2f::new(0.0, 0.0)), Player::new(Vector2f::new(20.0, 0.0))],
			tilemap: TileMap::new("res/map/map01.png"),
			input: Box::new(AdaptiveInput::new(0)),
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		self.handle_player_inputs(inputs);
		for p in self.players.iter_mut() {
			p.tick();
		}
	}

	pub fn render(&mut self, w: &mut RenderWindow, texture_state: &TextureState) {
        self.tilemap.render(w, texture_state);
		for p in self.players.iter_mut() {
			p.render(w, texture_state);
		}
	}

	fn handle_player_inputs(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		for (player, input) in self.players.iter_mut().zip(inputs.iter()) {
			player.direction = input.get_direction();
		}
		if !self.input.is_connected() {
			println!("joystick not connected");
		}
		self.players[0].direction = self.input.get_direction();
	}
}
