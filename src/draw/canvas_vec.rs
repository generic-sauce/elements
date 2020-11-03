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

impl ViewVec {
	pub fn to_canvas(self) -> CanvasVec { CanvasVec::new(self.x * VIEW_ASPECT, self.y) }
}

impl CanvasVec {
	pub const fn to_f(self) -> Vec2f { Vec2f::new(self.x as f32, self.y as f32) }
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
