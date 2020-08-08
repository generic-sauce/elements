use crate::prelude::*;

pub struct CanvasParam;
pub type CanvasVec = Vec2t<f32, CanvasParam>;

pub trait IntoCanvasVec {
	fn to_canvas(self, tilemap_size: TileVec) -> CanvasVec;
}

impl IntoCanvasVec for GameVec {
	fn to_canvas(self, tilemap_size: TileVec) -> CanvasVec {
		let factor = (TILESIZE * (tilemap_size.y + 1)) as f32;
		let x = self.x as f32 / factor;
		let y = self.y as f32 / factor;
		CanvasVec::new(x, y)
	}
}

impl IntoCanvasVec for TileVec {
	fn to_canvas(self, tilemap_size: TileVec) -> CanvasVec {
		self.to_game().to_canvas(tilemap_size)
	}
}
