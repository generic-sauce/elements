use crate::prelude::*;

const VIEW_SIZE: TileVec = TileVec::new(128, 72);
const VIEW_ASPECT: f32 = VIEW_SIZE.x as f32 / VIEW_SIZE.y as f32;

/* from (0, 0) to (VIEW_APSECT, 1)
 * corresponds to the draw area
 */
pub struct CanvasParam;
pub type CanvasVec = Vec2t<f32, CanvasParam>;

/* from (0, 0) to (1, 1)
 * corresponds to the draw area
 */
pub struct ViewParam;
pub type ViewVec = Vec2t<f32, ViewParam>;

/* from (-1, -1) to (1, 1)
 * corresponds to the window surface area
 */
pub struct SurfaceParam;
pub type SurfaceVec = Vec2t<f32, SurfaceParam>;

#[allow(unused)]
impl ViewVec {
	pub fn left_top(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x, y + 1.0) }
	pub fn right_bot(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x + VIEW_ASPECT, y) }
	pub fn right_top(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x + VIEW_ASPECT, y + 1.0) }
	pub fn center(x: f32, y: f32) -> CanvasVec { CanvasVec::new(x + VIEW_ASPECT * 0.5, y + 0.5) }
	pub fn to_canvas(self) -> CanvasVec { CanvasVec::new(self.x * VIEW_ASPECT, self.y) }
}

#[allow(unused)]
impl CanvasVec {
	pub fn left_top(x: f32, y: f32) -> ViewVec { ViewVec::new(x, y + 1.0) }
	pub fn right_bot(x: f32, y: f32) -> ViewVec { ViewVec::new(x + 1.0, y) }
	pub fn right_top(x: f32, y: f32) -> ViewVec { ViewVec::new(x + 1.0, y + 1.0) }
	pub fn center(x: f32, y: f32) -> ViewVec { ViewVec::new(x + 0.5, y + 0.5) }
	pub fn to_subpixel(self, window_size: SubPixelVec) -> SubPixelVec { SubPixelVec::new(self.x, self.y) * window_size.y }
	pub fn to_view(self) -> ViewVec { ViewVec::new(self.x / VIEW_ASPECT, self.y) }
}

pub trait IntoCanvasVec {
	fn to_canvas(self) -> CanvasVec;
}

impl IntoCanvasVec for GameVec {
	fn to_canvas(self) -> CanvasVec {
		let factor = (TILESIZE * VIEW_SIZE.y) as f32;
		let x = self.x as f32 / factor;
		let y = self.y as f32 / factor;
		CanvasVec::new(x, y)
	}
}

impl IntoCanvasVec for TileVec {
	fn to_canvas(self) -> CanvasVec {
		self.to_game().to_canvas()
	}
}

impl IntoCanvasVec for ViewVec {
	fn to_canvas(self) -> CanvasVec { self.to_canvas() }
}

impl IntoCanvasVec for CanvasVec {
	fn to_canvas(self) -> CanvasVec { self }
}

pub trait IntoSurfaceVec {
	fn to_surface(self, window_size: WindowVec) -> SurfaceVec;
}

impl IntoSurfaceVec for ViewVec {
	fn to_surface(self, _window_size: WindowVec) -> SurfaceVec {
		let v = self * 2.0 - 1.0;
		SurfaceVec::new(v.x, v.y)
	}
}

impl IntoSurfaceVec for CanvasVec {
	fn to_surface(self, window_size: WindowVec) -> SurfaceVec {
		self.to_view().to_surface(window_size)
	}
}
