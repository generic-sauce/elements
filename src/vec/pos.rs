use super::*;

use crate::prelude::*;

pub struct GameParam;
pub type GamePos = Vec2t<i32, GameParam>;

pub struct TileParam;
pub type TilePos = Vec2t<i32, TileParam>;

// Do we want this?
// pub struct FluidParam;
// pub type FluidGridPos = Vec2t<i32, FluidParam>;

impl GamePos {
	fn to_tilepos(self) -> TilePos {
		TilePos::new(self.x / TILESIZE, self.y / TILESIZE)
	}
}

impl TilePos {
	fn to_gamepos(self) -> GamePos {
		GamePos::new(self.x * TILESIZE, self.y * TILESIZE)
	}
}
