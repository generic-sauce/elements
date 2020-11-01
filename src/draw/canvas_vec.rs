use crate::prelude::*;

const VIEW_SIZE: TileVec = TileVec::new(128, 72);

pub struct CanvasParam;
pub type CanvasVec = Vec2t<f32, CanvasParam>;

impl CanvasVec {
	pub const fn to_f(self) -> Vec2f { Vec2f::new(self.x as f32, self.y as f32) }
}

pub trait IntoCanvasVec {
	fn to_canvas(self, _: TileVec) -> CanvasVec;
}

impl IntoCanvasVec for GameVec {
	fn to_canvas(self, _: TileVec) -> CanvasVec {
		let factor = (TILESIZE * VIEW_SIZE.y) as f32;
		let x = self.x as f32 / factor;
		let y = self.y as f32 / factor;
		CanvasVec::new(x, y)
	}
}

impl IntoCanvasVec for TileVec {
	fn to_canvas(self, v: TileVec) -> CanvasVec {
		self.to_game().to_canvas(v)
	}
}

impl IntoCanvasVec for CanvasVec {
	fn to_canvas(self, _: TileVec) -> CanvasVec { self }
}
