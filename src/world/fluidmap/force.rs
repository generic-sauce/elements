use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates

impl FluidMap {
	pub(in super) fn apply_forces<'a>(&'a self, t: &'a TileMap, players: &'a [Player; 2]) -> impl Iterator<Item=Fluid> + 'a {
		self.iter().map(move |f| {
			let neighbours = self.neighbours(f);

			let velocity = f.velocity;

			// gravity
			let velocity = velocity - GameVec::new(0, GRAVITY);

			// drag
			let velocity = velocity - velocity / 10;


			// neighbour-affection
			let velocity = velocity + neighbours
				.map(|n| affect(f, n))
				.sum::<GameVec>();

			// tilemap-affection
			let mut velocity = velocity + tilemap_affect(f, t);

			if let FluidState::AtHand = f.state {
				let cursor = players[f.owner].left_bot + PLAYER_SIZE / 2 + players[f.owner].cursor;
				velocity = velocity * 9 / 10 + (cursor - f.position).with_length(30);
			}

			let velocity = velocity.clamped(-200, 200);

			Fluid {
				velocity,
				..f.clone()
			}
		})
	}
}

fn affect(f: &Fluid, n: &Fluid) -> GameVec {
	let v = n.position - f.position;

	if v.magnitude_sqr() <= 30 * 30 {
		v.with_length(-30)
	} else if v.magnitude_sqr() <= 200 * 200 {
		v.with_length(-10)
	} else {
		(v * (-1)).with_length(2)
	}
}

fn tilemap_affect(f: &Fluid, t: &TileMap) -> GameVec {
	let mut affect = GameVec::new(0, 0);

	let p = f.position + TileVec::new(0, -1).to_game();
	if super::physics::is_colliding(p, t) {
		affect += GameVec::new(0, 40);
	}

	let p = f.position + TileVec::new(0, 1).to_game();
	if super::physics::is_colliding(p, t) {
		affect += GameVec::new(0, -10);
	}

	let p = f.position + TileVec::new(-1, 0).to_game();
	if super::physics::is_colliding(p, t) {
		affect += GameVec::new(10, 0);
	}

	let p = f.position + TileVec::new(1, 0).to_game();
	if super::physics::is_colliding(p, t) {
		affect += GameVec::new(-10, 0);
	}

	affect
}
