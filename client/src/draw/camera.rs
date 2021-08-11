use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Camera {
	pub left_bot: ViewVec,
	pub zoom: f32,
}
