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
			players: [Player::new(Vec2f::new(32.0, 32.0)), Player::new(Vec2f::new(64.0, 32.0))],
			tilemap: TileMap::new("res/map/map02.png"),
			input: Box::new(AdaptiveInput::new(0)),
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		self.handle_player_inputs(inputs);
		for p in self.players.iter_mut() {
			p.tick();
		}
	}

	pub fn draw(&mut self, context: &Context) {
        self.tilemap.draw(context);
		for p in self.players.iter_mut() {
			p.draw(context);
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
