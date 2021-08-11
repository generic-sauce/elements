use crate::prelude::*;

mod physics;
mod grab;

mod force;
mod activity;
mod update;

pub use update::*;
pub use force::*;

pub const FLUID_SPAWN_DIST: u32 = 20; // every 20 frames a new fluid will spawn
pub const MAX_IGNORE_COUNTER: u32 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FluidState {
	AtHand(u8), // the player index you're at
	Free,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Fluid {
	pub state: FluidState,
	pub team: u8,
	pub velocity: GameVec,
	pub position: GameVec,
	pub reference_position: GameVec,
	pub ignore_counter: u32,
	pub id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct FluidMap {
	pub grid: Vec<Vec<Fluid>>,
	pub size: FluidVec,
	pub next_id: u32,
	pub spawn_counter: u32,
}

impl World {
	pub fn tick_fluidmap(&mut self) {
		let iter = self.fluidmap.iter()
			.cloned()
			.map(|f| self.fluidmap.apply_grab(f, &self.teams[..], &self.players))
			.map(|f| self.fluidmap.apply_forces(f, &self.tilemap, &self.players, self.frame_id))
			.map(|f| FluidMap::move_fluid_by_velocity(f, &self.tilemap))
			.map(|mut f| {
				f.update_reference_position();
				f.ignore_counter = f.ignore_counter.checked_sub(1).unwrap_or(0);
				f
			});
		self.fluidmap.grid = FluidMap::mk_grid(iter, self.fluidmap.size);
	}
}

impl FluidMap {
	pub fn new(tilemap_size: TileVec) -> FluidMap {
		let gamemap_size = tilemap_size.to_game(); // number of game-tiles
		let size = gamemap_size.to_fluid(); // number of fluid-tiles // TODO this may round down and not cover the full size, right?

		FluidMap {
			grid: FluidMap::mk_grid(None.into_iter(), size),
			size,
			next_id: 0,
			spawn_counter: 0,
		}
	}

	pub fn mk_grid(fluids: impl Iterator<Item=Fluid>, size: FluidVec) -> Vec<Vec<Fluid>> {
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
			.flat_map(|x| x.iter())
	}

	// this function is not allowed to mutate the position, as this requires grid reordering!
	pub fn iter_mut_notranslate(&mut self) -> impl Iterator<Item=&mut Fluid> + '_ {
		self.grid.iter_mut()
			.flat_map(|x| x.iter_mut())
	}

	pub fn index(&self, t: FluidVec) -> &'_ [Fluid] {
		let i = (t.x + t.y * self.size.x) as usize;
		self.grid.get(i).map(|v| &v[..]).unwrap_or(&[])
	}

	pub fn neighbours<'s>(&'s self, f: &'s Fluid) -> impl Iterator<Item=&Fluid> + 's {
		let fluid_tile = f.position.to_fluid();

		iproduct!(-1..2, -1..2)
			.flat_map(move |t| self.index(fluid_tile + t))
			.filter(move |n| (f.position - n.position).as_short_as(FLUID_AFFECT_DIST))
	}

	pub fn add_fluid(&mut self, fluid: Fluid) {
		let tile_pos: FluidVec = fluid.position.into();
		let index = (tile_pos.x + tile_pos.y * self.size.x) as usize;
		self.grid[index].push(fluid);
	}
}
