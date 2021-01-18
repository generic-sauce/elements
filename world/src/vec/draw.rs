use crate::prelude::*;

const VIEW_SIZE_TILE: TileVec = TileVec::new(128, 72);
const VIEW_SIZE_GAME: GameVec = VIEW_SIZE_TILE.to_game();
const VIEW_ASPECT: f32 = VIEW_SIZE_TILE.x as f32 / VIEW_SIZE_TILE.y as f32;
pub const CURSOR_RADIUS: f32 = 0.6 / VIEW_SIZE_TILE.y as f32;

/* from (0, 0) to (1, 1)
 * corresponds to the draw area
 */
pub struct ViewParam;
pub type ViewVec = Vec2t<f32, ViewParam>;

/* from (0, 0) to (VIEW_APSECT, 1)
 * corresponds to the draw area
 */
pub struct CanvasParam;
pub type CanvasVec = Vec2t<f32, CanvasParam>;

/* from (-1, -1) to (1, 1)
 * corresponds to the window surface area
 */
pub struct SurfaceParam;
pub type SurfaceVec = Vec2t<f32, SurfaceParam>;

#[allow(unused)]
impl ViewVec {
	// pub fn left_top(x: f32, y: f32) -> ViewVec { ViewVec::new(x, y + 1.0) }
	// pub fn right_bot(x: f32, y: f32) -> ViewVec { ViewVec::new(x + 1.0, y) }
	// pub fn right_top(x: f32, y: f32) -> ViewVec { ViewVec::new(x + 1.0, y + 1.0) }
	// pub fn center(x: f32, y: f32) -> ViewVec { ViewVec::new(x + 0.5, y + 0.5) }

	pub fn to_canvas(self) -> CanvasVec { CanvasVec::new(self.x * VIEW_ASPECT, self.y) }

	pub fn to_subpixel(self, window_size: SubPixelVec) -> SubPixelVec {
		let mut v = self.to_surface(window_size);
		v = v * 0.5 + 0.5;
		v.x *= window_size.x;
		v.y *= window_size.y;
		SubPixelVec::new(v.x, v.y)
	}

	pub fn to_surface(self, window_size: SubPixelVec) -> SurfaceVec {
		let mut v = self * 2.0 - 1.0;
		let ratio = window_view_ratio(window_size);
		if ratio > 1.0 {
			v.x /= ratio;
		} else {
			v.y *= ratio;
		}
		SurfaceVec::new(v.x, v.y)
	}

	pub fn to_surface_correct_aspect(self) -> SurfaceVec {
		let mut v = self * 2.0 - 1.0;
		SurfaceVec::new(v.x, v.y)
	}
}

#[allow(unused)]
impl CanvasVec {
	pub fn left_top(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x, y + 1.0) }
	pub fn right_bot(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x + VIEW_ASPECT, y) }
	pub fn right_top(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x + VIEW_ASPECT, y + 1.0) }
	pub fn center(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x + VIEW_ASPECT * 0.5, y + 0.5) }

	pub fn to_view(self) -> ViewVec { ViewVec::new(self.x / VIEW_ASPECT, self.y) }
	pub fn to_subpixel(self, window_size: SubPixelVec) -> SubPixelVec { self.to_view().to_subpixel(window_size) }
	pub fn to_surface(self, window_size: SubPixelVec) -> SurfaceVec { self.to_view().to_surface(window_size) }

	pub fn aspect() -> CanvasVec { CanvasVec::new(VIEW_ASPECT, 1.0) }
}

pub trait IntoViewVec {
	fn to_view(self) -> ViewVec;
}

impl IntoViewVec for GameVec {
	fn to_view(self) -> ViewVec {
		let x = self.x as f32 / VIEW_SIZE_GAME.x as f32;
		let y = self.y as f32 / VIEW_SIZE_GAME.y as f32;
		ViewVec::new(x, y)
	}
}

impl IntoViewVec for TileVec {
	fn to_view(self) -> ViewVec {
		self.to_game().to_view()
	}
}

impl IntoViewVec for CanvasVec {
	fn to_view(self) -> ViewVec { self.to_view() }
}

impl IntoViewVec for ViewVec {
	fn to_view(self) -> ViewVec { self }
}

pub fn window_view_ratio(window_size: SubPixelVec) -> f32 {
	let aspect = window_size.x / window_size.y;
	let ratio = aspect / VIEW_ASPECT;
	ratio
}
