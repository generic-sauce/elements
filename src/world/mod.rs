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
			players: [Player::new(Vec2i::new(38 * TILESIZE, 45 * TILESIZE)), Player::new(Vec2i::new(64 * TILESIZE, 32 * TILESIZE))],
			tilemap: TileMap::new("res/map/map02.png"),
			input: Box::new(AdaptiveInput::new(0)),
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		for (p, input) in self.players.iter_mut().zip(inputs.iter()) {
			p.tick(&self.tilemap, input.as_ref());
		}
	}

	pub fn draw(&mut self, context: &mut Context) {
        self.tilemap.draw(context);
		for p in self.players.iter_mut() {
			p.draw(context);
		}
        context.draw_fluids();
	}
}
