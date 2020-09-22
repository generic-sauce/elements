use crate::prelude::*;

pub const FLUID_SPAWN_INTERVAL: u32 = 20; // every 20 frames a new fluid will spawn

#[derive(Serialize, Deserialize, Clone)]
pub struct FluidCharacter {
	spawn_counter: u32,
}

impl FluidCharacter {
	pub fn new() -> FluidCharacter {
		FluidCharacter {
			spawn_counter: FLUID_SPAWN_INTERVAL,
		}
	}

	pub fn tick(&mut self, p: usize, players: &mut [Player; 2], tilemap: &mut TileMap, fluidmap: &mut FluidMap) {
		self.spawn_fluids(p, &players[p], tilemap, fluidmap);
	}

	fn spawn_fluids(&mut self, p: usize, player: &Player, tilemap: &TileMap, fluidmap: &mut FluidMap) {
		if self.spawn_counter > 0 {
			self.spawn_counter -= 1;
			return;
		} else {
			self.spawn_counter = FLUID_SPAWN_INTERVAL;
		}

		let calc_spawn_pos = |from: GameVec, to: GameVec| {
			let accuracy = |v: GameVec| (v.x.abs() + v.y.abs()) / 40 + 2; // TODO is this a good choice?
			let n = accuracy(from - to);
			for i in 0..n {
				let current = from * (n-1-i) / (n-1) + to * i / (n-1);
				if !tilemap.check_solid(current) { return current; }
			}
			panic!("this implies that the player is glitched actually!");
		};

		let position = calc_spawn_pos(player.cursor_position(), player.center_position());

		fluidmap.add_fluid(Fluid {
			state: FluidState::AtHand,
			owner: p,
			velocity: 0.into(),
			position,
			reference_position: position,
			ignore_counter: 0,
			id: fluidmap.next_id,
		});

		fluidmap.next_id = fluidmap.next_id.checked_add(1).unwrap_or(0);
	}
}
