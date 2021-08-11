use crate::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum CameraMode {
	Normal,
	Transformed,
}

pub struct DrawVec {
	pub vec: ViewVec,
	pub camera_mode: CameraMode,
}

pub trait IntoDrawVec : IntoViewVec {
	fn to_draw(self) -> DrawVec;
}

impl IntoDrawVec for ViewVec {
	fn to_draw(self) -> DrawVec {
		DrawVec {
			vec: self,
			camera_mode: CameraMode::Normal,
		}
	}
}

impl IntoDrawVec for CanvasVec {
	fn to_draw(self) -> DrawVec {
		DrawVec {
			vec: self.to_view(),
			camera_mode: CameraMode::Normal,
		}
	}
}

impl IntoDrawVec for GameVec {
	fn to_draw(self) -> DrawVec {
		DrawVec {
			vec: self.to_view(),
			camera_mode: CameraMode::Transformed,
		}
	}
}

impl IntoDrawVec for TileVec {
	fn to_draw(self) -> DrawVec {
		DrawVec {
			vec: self.to_view(),
			camera_mode: CameraMode::Transformed,
		}
	}
}
