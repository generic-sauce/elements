use crate::prelude::*;

const VIEW_SIZE: TileVec = TileVec::new(128, 72);
const VIEW_ASPECT: f32 = VIEW_SIZE.x as f32 / VIEW_SIZE.y as f32;

/* from (-1, -1) to (1, 1)
 * corresponds to the window surface area
 */
pub struct SurfaceParam;
pub type SurfaceVec = Vec2t<f32, SurfaceParam>;

pub trait IntoSurfaceVec {
	fn to_surface(self, window_size: WindowVec) -> SurfaceVec;
}

impl IntoSurfaceVec for CanvasVec {
	fn to_surface(self, _window_size: WindowVec) -> SurfaceVec {
		let mut v = self.to_f();
		v.x /= VIEW_ASPECT;
		v = v * 2.0 - 1.0;

		SurfaceVec::new(v.x, v.y)
	}
}
