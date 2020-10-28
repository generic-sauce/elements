use crate::prelude::*;

const WALLS_PER_FLUID: u32 = 6;

impl World {
	pub(in super) fn handle_wall(&mut self, p: usize, handler: &mut impl EventHandler) {
		let player = &mut self.players[p];
		let cursor = player.cursor_position();
		let from = match player.wall_mode {
			WallMode::NoFluids => return,
			WallMode::NotWalling => cursor,
			WallMode::InProgress(x) => x,
		};

		player.wall_mode = WallMode::InProgress(cursor);

		self.wall_from_to(p, from, cursor, handler);
	}

	pub(in super) fn stop_wall(&mut self, p: usize) {
		self.players[p].wall_mode = WallMode::NotWalling;
	}

	fn wall_from_to(&mut self, p: usize, from: GameVec, to: GameVec, handler: &mut impl EventHandler) {
		let path = self.generate_wall_path(p, from, to);
		for t in path {
			if self.wall(p, t, handler).is_none() {
				self.players[p].wall_mode = WallMode::NoFluids;
				return;
			}
		}
	}

	fn assert_unglitched_path(&self, path: &[TileVec]) {
		assert!(path.iter().all(|t| !self.coll(*t)));
		self.assert_path(path);
	}

	fn assert_path(&self, path: &[TileVec]) {
		for i in 0..path.len()-1 {
			assert_eq!((path[i] - path[i+1]).length_squared(), 1);
		}
	}

	fn generate_wall_path(&self, p: usize, from: GameVec, to: GameVec) -> Vec<TileVec> {
		let mut path = self.direct_path(p, from, to);

		self.assert_path(&path[..]);

		while let Some(i) = path.iter().position(|tile| self.coll(*tile)) {
			let before_gap = i-1;
			let after_gap = path.iter().enumerate().position(|(j, tile)| j > i && !self.coll(*tile)).unwrap();

			self.assert_path(&path[..]);

			let dir = Direction::from_diff(path[before_gap], path[i]).unwrap();
			let inner_path = self.pathfind(path[before_gap], dir, path[after_gap]);

			self.assert_path(&inner_path[..]);
			let n = inner_path.len();

			path.splice(before_gap..=after_gap, inner_path);

			self.assert_path(&path[..]);
			self.assert_unglitched_path(&path[..(i - 1 + n)]);
		}
		self.assert_unglitched_path(&path[..]);
		path
	}

	fn direct_path(&self, p: usize, from: GameVec, to: GameVec) -> Vec<TileVec> {
		let from = self.unglitch(p, from);
		let to = self.unglitch(p, to);

		let mut direct_path = vec![from.to_tile()];

		let n = (from - to).length() * 8 / TILESIZE; // is this well?
		if n >= 2 {
			for i in 0..n {
				let current = from * (n - i - 1) / (n - 1) + to * i / (n - 1);
				let current_tile = current.to_tile();

				if direct_path.last().unwrap() != &current_tile {
					direct_path.push(current_tile);
				}
			}
		}
		if direct_path.last().unwrap() != &to.to_tile() {
			direct_path.push(to.to_tile());
		}

		direct_path
	}

	fn wall(&mut self, p: usize, pos_tile: TileVec, handler: &mut impl EventHandler) -> Option<()> {
		assert!(!self.coll(pos_tile));

		let tile = self.tilemap.get(pos_tile);

		let refill_amount = match tile {
			Tile::Void => WALL_LIFETIME,
			Tile::Wall { owner, remaining_lifetime } if owner == p => {
				WALL_LIFETIME - remaining_lifetime
			},
			_ => return Some(()),
		};

		self.alloc_wall_lifetime(p, refill_amount)?;

		self.tilemap.set(pos_tile, Tile::Wall { owner: p, remaining_lifetime: WALL_LIFETIME });
		handler.tilemap_changed();

		Some(())
	}

	fn alloc_wall_lifetime(&mut self, p: usize, amount: u32) -> Option<()> {
		let mut pl = &mut self.players[p];

		// allocate free_wall
		if pl.free_wall_lifetime < amount {
			for inner_v in self.fluidmap.grid.iter_mut() {
				while let Some(i) = inner_v.iter().position(|x| x.owner == p) {
					inner_v.swap_remove(i);
					pl.free_wall_lifetime += WALLS_PER_FLUID * WALL_LIFETIME;
					if pl.free_wall_lifetime >= amount { break; }
				}
			}
		}

		if pl.free_wall_lifetime < amount {
			return None;
		}

		pl.free_wall_lifetime -= amount;
		Some(())
	}

	// return a position close to pos, not colliding with the player p
	fn unglitch(&self, p: usize, start: GameVec) -> GameVec {
		let pl = &self.players[p];
		let center = pl.center_position();

		let mut diff = start - center;

		// in order to prevent division by zero
		if diff == v(0, 0) {
			diff = v(1, 0);
		}

		const STEP: i32 = 4;

		// binary search would be much faster - or one could even do it correctly ^^'
		(0..).map(|i| start + diff.with_length(i * STEP))
			.find(|pos| !pl.collides_tile(pos.to_tile()))
			.unwrap()
	}

	fn pathfind(&self, start: TileVec, gap_dir: Direction, goal: TileVec) -> Vec<TileVec> {
		let p1 = self.pathfind_impl(start, gap_dir, goal, true);
		let p2 = self.pathfind_impl(start, gap_dir, goal, false);

		if p1.len() < p2.len() { p1 } else { p2 }
	}

	fn coll(&self, tile_pos: TileVec) -> bool {
		self.players.iter().any(|pl| pl.collides_tile(tile_pos))
	}

	// TODO may infinite-loop if you have >= 4 players
	fn pathfind_impl(&self, start: TileVec, mut gap_dir: Direction, goal: TileVec, clockwise: bool) -> Vec<TileVec> {
		let mut current = start;
		let mut path = vec![start];

		assert!(!self.coll(start));
		assert!(self.coll(start + gap_dir));

		let mut push = |tile_pos| {
			if path.len() >= 2 && path[0] == tile_pos { panic!("infinite loop!"); }

			assert!(!self.coll(tile_pos));

			path.push(tile_pos);
		};

		while current != goal {
			let forward = gap_dir.turn(!clockwise);
			if self.coll(current + forward) {
				gap_dir = forward;
			} else {
				current += forward;
				push(current);

				if !self.coll(current + gap_dir) {
					current += gap_dir;
					push(current);

					gap_dir = gap_dir.turn(clockwise);
				}
			}
		}

		path
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl From<Direction> for TileVec {
	fn from(d: Direction) -> TileVec {
		match d {
			Direction::Up => v(0, 1),
			Direction::Right => v(1, 0),
			Direction::Down => v(0, -1),
			Direction::Left => v(-1, 0),
		}
	}
}

impl Direction {
	fn turn_clockwise(self) -> Direction {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}

	fn turn_counter_clockwise(self) -> Direction {
		match self {
			Direction::Up => Direction::Left,
			Direction::Right => Direction::Up,
			Direction::Down => Direction::Right,
			Direction::Left => Direction::Down,
		}
	}

	fn turn(self, clockwise: bool) -> Direction {
		if clockwise {
			self.turn_clockwise()
		} else {
			self.turn_counter_clockwise()
		}
	}

	fn from_diff(from: TileVec, to: TileVec) -> Option<Direction> {
		match to - from {
			TileVec { x: 1, y: 0, .. } => Some(Direction::Right),
			TileVec { x: -1, y: 0, .. } => Some(Direction::Left),
			TileVec { x: 0, y: 1, .. } => Some(Direction::Up),
			TileVec { x: 0, y: -1, .. } => Some(Direction::Down),
			_ => None,
		}
	}
}