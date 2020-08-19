use crate::prelude::*;

mod draw;
mod physics;

pub mod force;
pub use self::force::*;

#[derive(Clone)]
pub enum FluidState {
	Owned,
	AtHand,
	Free,
}

#[derive(Clone)]
pub struct Fluid {
	pub state: FluidState,
	pub owner: usize,
	pub velocity: GameVec,
	pub position: GameVec,
}

pub struct FluidMap {
	pub grid: Vec<Vec<Fluid>>,
	pub size: FluidVec,
}

impl FluidMap {
	pub fn new(tilemap_size: TileVec) -> FluidMap {
		let iter =
			(0..1000).map(|offset| {
				let position = GameVec::new(13000 + offset * 10, 13000 + (offset % 200));
				Fluid {
					state: FluidState::Free,
					owner: 1,
					velocity: 0.into(),
					position,
				}
			});

		let tilemap_size = TileVec::new(tilemap_size.x as i32, tilemap_size.y as i32); // number of tiles
		let gamemap_size = tilemap_size.to_game(); // number of game-tiles
		let size = gamemap_size.to_fluid(); // number of fluid-tiles // TODO this may round down and not cover the full size, right?

		FluidMap {
			grid: FluidMap::mk_grid(iter, size),
			size,
		}
	}

	pub fn tick(&mut self, t: &TileMap) {
		let iter = self.apply_forces(t)
			.map(|f| FluidMap::move_fluid_by_velocity(f, t));
		self.grid = FluidMap::mk_grid(iter, self.size);
	}

	fn mk_grid(fluids: impl Iterator<Item=Fluid>, size: FluidVec) -> Vec<Vec<Fluid>> {
		let mut grid: Vec<Vec<Fluid>> = (0..(size.x * size.y)).map(|_| Vec::new()).collect();

		for f in fluids {
			let fluid_pos = f.position.to_fluid();
			let i = (fluid_pos.x + fluid_pos.y * size.x) as usize;
			grid[i].push(f);
		}

		grid
	}

	pub fn iter(&self) -> impl Iterator<Item=&Fluid> + '_ {
		self.grid.iter()
			.map(|x| x.iter())
			.flatten()
	}

	fn index(&self, t: FluidVec) -> &'_ [Fluid] {
		let i = (t.x + t.y * self.size.x) as usize;
		self.grid.get(i).map(|v| &v[..]).unwrap_or(&[])
	}

	pub fn neighbours<'s>(&'s self, f: &'s Fluid) -> impl Iterator<Item=&Fluid> + 's {
		use itertools::iproduct;

		let fluid_tile = f.position.to_fluid();

		iproduct!(-1..2, -1..2)
			.map(move |t| fluid_tile + t)
			.map(move |t| self.index(t))
			.flatten()
			.filter(move |n| (f.position - n.position).magnitude_sqr() <= FLUID_AFFECT_DIST * FLUID_AFFECT_DIST)
	}
}
