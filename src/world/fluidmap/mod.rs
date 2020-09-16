use crate::prelude::*;

mod physics;
mod grab;

pub mod force;
mod activity;
mod update;

pub use update::*;
pub use force::*;

pub const FLUID_SPAWN_DIST: u32 = 20; // every 20 frames a new fluid will spawn
pub const MAX_IGNORE_COUNTER: u32 = 20;

pub const FLUID_MIN_DIST: i32 = TILESIZE;
pub const FLUID_CELL_SIZE: i32 = calc_fluid_cell_size();

// if two fluids have a distance >= FLUID_MIN_DIST, then they should be in different cells
const fn calc_fluid_cell_size() -> i32 {
	let cs = (FLUID_MIN_DIST as f64 * std::f64::consts::FRAC_1_SQRT_2) as i32 - 1;
	assert!(cs * cs + cs * cs < FLUID_MIN_DIST * FLUID_MIN_DIST);
	cs
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FluidState {
	AtHand,
	Free,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Fluid {
	pub state: FluidState,
	pub owner: usize,
	pub velocity: GameVec,
	pub position: GameVec,
	pub reference_position: GameVec,
	pub ignore_counter: u32,
	pub id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct FluidMap {
	pub grid: Vec<Option<Fluid>>,
	pub size: FluidVec,
	pub next_id: u32,
	pub spawn_counter: u32,
}

impl World {
	pub fn tick_fluidmap(&mut self) {
		self.fluidmap.tick_grab(&self.players);
		self.fluidmap.apply_forces(&self.tilemap, &self.players, self.frame_id);
		self.fluidmap.tick_physics(&self.tilemap);
		self.fluidmap.update_reference_positions();
	}
}

impl FluidMap {
	fn update_reference_positions(&mut self) {
		for f in self.iter_mut_notranslate() {
			f.update_reference_position();
		}
	}

	pub fn new(tilemap_size: TileVec) -> FluidMap {
		let tilemap_size = TileVec::new(tilemap_size.x as i32, tilemap_size.y as i32); // number of tiles
		let gamemap_size = tilemap_size.to_game(); // number of game-tiles
		let size = gamemap_size.to_fluid() + 1; // number of fluid-tiles

		FluidMap {
			grid: FluidMap::mk_grid(None.into_iter(), size),
			size,
			next_id: 0,
			spawn_counter: 0,
		}
	}

	pub fn mk_grid(iter: impl Iterator<Item=Fluid>, size: FluidVec) -> Vec<Option<Fluid>> {
		let mut grid = vec![None; (size.x * size.y) as usize];

		for f in iter {
			let i = FluidMap::index(size, f.position.into());
			assert!(grid[i].is_none());
			grid[i] = Some(f);
		}

		grid
	}

	pub fn get(&self, v: FluidVec) -> Option<&'_ Fluid> {
		self.grid.get(FluidMap::index(self.size, v))
			.and_then(|x| x.as_ref())

	}

	pub fn get_mut(&mut self, v: FluidVec) -> Option<&'_ mut Fluid> {
		self.grid.get_mut(FluidMap::index(self.size, v))
			.and_then(|x| x.as_mut())
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

	fn index(size: FluidVec, t: FluidVec) -> usize {
		(t.x + t.y * size.x) as usize
	}

	// returns fluids with distance <= dist
	pub fn neighbours<'s>(&'s self, f: &'s Fluid, dist: i32) -> impl Iterator<Item=&Fluid> + 's {
		let fluid_tile = f.position.to_fluid();

		let num_cells = dist / FLUID_CELL_SIZE + 1; // TODO round_up(dist / FLUID_CELL_SIZE) may be a smaller iteration
		// it would possibly be faster to iterate from 0, 0 to the farther parts for a few applications!
		iproduct!(-num_cells..=num_cells, -num_cells..=num_cells)
			.map(move |t| fluid_tile + t)
			.map(move |t| FluidMap::index(self.size, t))
			.map(move |idx| self.grid[idx].iter())
			.flatten()
			.filter(move |n| (f.position - n.position).as_short_as(dist))
	}

	pub fn neighbours_with_owner<'s>(&'s self, f: &'s Fluid, dist: i32) -> impl Iterator<Item=&Fluid> + 's {
		self.neighbours(f, dist).filter(move |n| n.owner == f.owner)
	}

	pub fn add_fluid(&mut self, fluid: Fluid) {
		let tile_pos: FluidVec = fluid.position.into();
		let index = FluidMap::index(self.size, tile_pos);
		assert!(self.grid[index].is_none());
		self.grid[index] = Some(fluid);
	}

	pub fn collides_fluid(&self, p: GameVec) -> bool {
		// TODO this is very inefficient, re-implement using neighbours().next().is_some()
		self.iter().any(|f| (f.position - p).as_short_as(MIN_FLUID_DISTANCE))
	}
}
