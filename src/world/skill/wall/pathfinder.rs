use crate::prelude::*;

impl World {
	pub(super) fn generate_wall_path(&self, p: usize, from: GameVec, to: GameVec) -> Vec<TileVec> {
		let mut path = self.direct_path(p, from, to);

		self.assert_path(&path[..]);

		// remove glitched start- and end-segments
		while path.first().map(|x| self.coll(*x)).unwrap_or(false) {
			path.remove(0);
		}

		while path.last().map(|x| self.coll(*x)).unwrap_or(false) {
			path.pop();
		}

		// remove glitched inner segments
		while let Some(i) = path.iter().position(|tile| self.coll(*tile)) {
			let before_gap = i - 1;
			let after_gap = path.iter().enumerate().position(|(j, tile)| j > i && !self.coll(*tile)).unwrap();

			self.assert_path(&path[..]);

			let diff = path[i] - path[before_gap];
			let dir = Direction::from_diff(diff).unwrap();
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

		// remove diagonals
		while let Some(i) = (0..direct_path.len() - 1).find(|&i| (direct_path[i] - direct_path[i + 1]).length_squared() > 1) {
			let new: TileVec = v(direct_path[i].x, direct_path[i + 1].y);

			assert_eq!((new - direct_path[i]).length_squared(), 1);
			assert_eq!((new - direct_path[i + 1]).length_squared(), 1);

			direct_path.insert(i + 1, new);
		}

		direct_path
	}

	fn pathfind(&self, start: TileVec, gap_dir: Direction, goal: TileVec) -> Vec<TileVec> {
		let p1 = self.pathfind_impl(start, gap_dir, goal, true);
		let p2 = self.pathfind_impl(start, gap_dir, goal, false);

		if p1.len() < p2.len() { p1 } else { p2 }
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

	pub(super) fn coll(&self, tile_pos: TileVec) -> bool {
		self.players.iter().any(|pl| pl.collides_tile(tile_pos))
	}

	fn assert_unglitched_path(&self, path: &[TileVec]) {
		assert!(path.iter().all(|t| !self.coll(*t)));
		self.assert_path(path);
	}

	fn assert_path(&self, path: &[TileVec]) {
		for i in 1..path.len() {
			let diff = path[i-1] - path[i];
			assert_eq!(diff.x.abs() + diff.y.abs(), 1);
		}
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

	fn from_diff(diff: TileVec) -> Option<Direction> {
		match diff {
			TileVec { x: 1, y: 0, .. } => Some(Direction::Right),
			TileVec { x: -1, y: 0, .. } => Some(Direction::Left),
			TileVec { x: 0, y: 1, .. } => Some(Direction::Up),
			TileVec { x: 0, y: -1, .. } => Some(Direction::Down),
			_ => None,
		}
	}
}
