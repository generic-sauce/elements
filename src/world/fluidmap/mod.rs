use crate::prelude::*;

mod draw;
mod physics;
mod grab;

pub mod force;
pub use self::force::*;

pub const FLUID_SPAWN_DIST: u32 = 20; // every 20 frames a new fluid will spawn

#[derive(Clone, PartialEq, Eq)]
pub enum FluidState {
	AtHand,
	Free,
}

#[derive(Clone)]
pub struct Fluid {
	pub state: FluidState,
	pub owner: usize,
	pub velocity: GameVec,
	pub position: GameVec,
	pub id: i32,
}

pub struct FluidMap {
	pub grid: Vec<Vec<Fluid>>,
	pub size: FluidVec,
	pub next_id: i32,
	pub spawn_counter: u32,
}

impl FluidMap {
	pub fn new(tilemap_size: TileVec) -> FluidMap {
		let tilemap_size = TileVec::new(tilemap_size.x as i32, tilemap_size.y as i32); // number of tiles
		let gamemap_size = tilemap_size.to_game(); // number of game-tiles
		let size = gamemap_size.to_fluid(); // number of fluid-tiles // TODO this may round down and not cover the full size, right?

		FluidMap {
			grid: FluidMap::mk_grid(None.into_iter(), size),
			size,
			next_id: 0,
			spawn_counter: 0,
		}
	}

	pub fn tick(&mut self, t: &TileMap, players: &[Player; 2]) {
		let iter = self.iter()
			.cloned()
			.map(|f| self.apply_grab(f, players))
			.map(|f| self.apply_forces(f, t, players))
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

	// this function is not allowed to mutate the position, as this requires grid reordering!
	pub fn iter_mut_notranslate(&mut self) -> impl Iterator<Item=&mut Fluid> + '_ {
		self.grid.iter_mut()
			.map(|x| x.iter_mut())
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
			.filter(move |n| (f.position - n.position).as_short_as(FLUID_AFFECT_DIST))
	}

	pub fn neighbours_with_owner<'s>(&'s self, f: &'s Fluid) -> impl Iterator<Item=&Fluid> + 's {
		self.neighbours(f).filter(move |n| n.owner == f.owner)
	}

	pub fn add_fluid(&mut self, fluid: Fluid) {
		let tile_pos: FluidVec = fluid.position.into();
		let index = (tile_pos.x + tile_pos.y * self.size.x) as usize;
		self.grid[index].push(fluid);
	}
}
