pub mod player;
pub mod tilemap;
pub mod fluidmap;
mod draw;

use crate::prelude::*;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub fluidmap: FluidMap,
}

impl World {
	pub fn new() -> World {
		let tilemap = TileMap::new("res/map/map02.png");
		World {
			players: [Player::new(TileVec::new(38, 45).into()), Player::new(TileVec::new(64, 40).into())],
			fluidmap: FluidMap::new(tilemap.size),
			tilemap,
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		// sub-tick
		self.fluidmap.tick(&self.tilemap);
		for (p, input) in self.players.iter_mut().zip(inputs.iter_mut()) {
			input.update();
			p.tick(&self.tilemap, input.as_ref());
		}

		// damage
		self.check_damage();
	}

	fn check_damage(&mut self) {
		for i in 0..2 {
			let player = &mut self.players[i];
			let collides_player = |p: GameVec| player.left_bot.x <= p.x && p.x <= player.left_bot.x + PLAYER_SIZE.x - 1
											&& player.left_bot.y <= p.y && p.y <= player.left_bot.y + PLAYER_SIZE.y - 1;
			let mut dmg = 0;
			for v in self.fluidmap.grid.iter_mut() {
				v.drain_filter(|x| x.owner != i && collides_player(x.position))
				 .for_each(|_| dmg += 1 )
			}
			if dmg > 0 { player.damage(dmg); }
		}
	}
}
