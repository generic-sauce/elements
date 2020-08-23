pub mod player;
pub mod tilemap;
pub mod fluidmap;
pub mod skill;
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
		self.fluidmap.tick(&self.tilemap, &self.players);
		self.tick_players(inputs);
		self.handle_skills(inputs);
		self.spawn_fluids();
		self.despawn_fluids();
		self.check_damage();
	}

	fn tick_players(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		for (player, input) in self.players.iter_mut().zip(inputs.iter_mut()) {
			input.update(player);
			player.tick(&self.tilemap, input.as_ref());
		}
	}

	fn spawn_fluids(&mut self) {
		for i in 0..2 {
			let p = &self.players[i];

			let calc_spawn_pos = |from: GameVec, to: GameVec| {
				let accuracy = |v: GameVec| (v.x.abs() + v.y.abs()) / 40 + 2; // TODO is this a good choice?
				let n = accuracy(from - to);
				for i in 0..n {
					let current = from * (n-1-i) / (n-1) + to * i / (n-1);
                    dbg!(from, to, current);
					if !self.tilemap.check_solid(current) { return current; }
				}
				panic!("this implies that the player is glitched actually!");
			};

			let position = calc_spawn_pos(p.cursor_position(), p.center_position());

			self.fluidmap.add_fluid(Fluid{
				state: FluidState::AtHand,
				owner: i,
				velocity: 0.into(),
				position,
			});
		}
	}

	fn despawn_fluids(&mut self) {
		for cell in self.fluidmap.grid.iter_mut() {
			cell.drain_filter(|_| {
				let r = rand::random::<u8>();
				r < 2
			});
		}
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
