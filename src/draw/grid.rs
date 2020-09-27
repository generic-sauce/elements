use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
	alive: bool,
	// Used for the trail effect. Always 255 if `self.alive` is true (We could
	// use an enum for Cell, but it makes several functions slightly more
	// complex, and doesn't actually make anything any simpler here, or save any
	// memory, so we don't)
	heat: u8,
}

impl Cell {
	pub fn new(alive: bool) -> Self {
		Self { alive, heat: 0 }
	}

	#[must_use]
	pub fn update(self) -> Self {
		self.next_state(self.alive)
	}

	#[must_use]
	pub fn next_state(mut self, alive: bool) -> Self {
		self.alive = alive;
		if self.alive {
			self.heat = 255;
		} else {
			self.heat = self.heat.saturating_sub(1);
		}
		self
	}

	pub fn set_alive(&mut self, alive: bool) {
		*self = self.next_state(alive);
	}

	pub fn cool_off(&mut self, decay: f32) {
		if !self.alive {
			let heat = (self.heat as f32 * decay).min(255.0).max(0.0);
			assert!(heat.is_finite());
			self.heat = heat as u8;
		}
	}
}

#[derive(Clone, Debug)]
pub struct Grid {
	cells: Vec<Cell>,
	width: usize,
	height: usize,
	// Should always be the same size as `cells`. When updating, we read from
	// `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
	// use, and `cells` should be updated directly.
	scratch_cells: Vec<Cell>,
}

impl Grid {
	pub fn new_empty(width: usize, height: usize) -> Self {
		assert!(width != 0 && height != 0);
		let size = width.checked_mul(height).expect("too big");
		Self {
			cells: vec![Cell::default(); size],
			scratch_cells: vec![Cell::default(); size],
			width,
			height,
		}
	}

	pub fn new_random(width: usize, height: usize) -> Self {
		let mut result = Self::new_empty(width, height);
		result.randomize();
		result
	}

	pub fn randomize(&mut self) {
		for c in self.cells.iter_mut() {
			let alive = rand::random::<f32>() < 0.3;
			*c = Cell::new(alive);
		}
		// run a few simulation iterations for aesthetics (If we don't, the
		// noise is ugly)
		for _ in 0..3 {
			self.update();
		}
		// Smooth out noise in the heatmap that would remain for a while
		for c in self.cells.iter_mut() {
			c.cool_off(0.4);
		}
	}

	pub fn update(&mut self) {
		for y in 0..self.height {
			for x in 0..self.width {
				let idx = x + y * self.width;
				let next = self.cells[idx].update();
				// Write into scratch_cells, since we're still reading from `self.cells`
				self.scratch_cells[idx] = next;
			}
		}
		std::mem::swap(&mut self.scratch_cells, &mut self.cells);
	}

	pub fn toggle(&mut self, x: isize, y: isize) -> bool {
		if let Some(i) = self.grid_idx(x, y) {
			let was_alive = self.cells[i].alive;
			self.cells[i].set_alive(!was_alive);
			!was_alive
		} else {
			false
		}
	}

	pub fn draw(&self, screen: &mut [u8]) {
		debug_assert_eq!(screen.len(), 4 * self.cells.len());
		for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
			let color = if c.alive {
				[0, 0xff, 0xff, 0xff]
			} else {
				[0, 0, c.heat, 0xff]
			};
			pix.copy_from_slice(&color);
		}
	}

	pub fn set_rect(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, alive: bool) {
		for (x, y) in iproduct!(x0.min(x1)..x0.max(x1), y0.min(y1)..y0.max(y1)) {
			if let Some(i) = self.grid_idx(x, y) {
				self.cells[i].set_alive(alive);
			} else {
				break;
			}
		}
	}

	pub fn grid_idx<I: std::convert::TryInto<usize>>(&self, x: I, y: I) -> Option<usize> {
		if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
			if x < self.width && y < self.height {
				Some(x + y * self.width)
			} else {
				None
			}
		} else {
			None
		}
	}
}
