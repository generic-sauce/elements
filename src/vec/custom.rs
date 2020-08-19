use crate::prelude::*;

pub struct GameParam;
pub type GameVec = Vec2t<i32, GameParam>;

pub struct TileParam;
pub type TileVec = Vec2t<i32, TileParam>;

pub struct FluidParam;
pub type FluidVec = Vec2t<i32, FluidParam>;

pub const TILESIZE: i32 = 256;

impl GameVec {
	pub const fn to_tile(self) -> TileVec { TileVec::new(self.x / TILESIZE, self.y / TILESIZE) }
	pub const fn to_fluid(self) -> FluidVec { FluidVec::new(self.x / FLUID_AFFECT_DIST, self.y / FLUID_AFFECT_DIST) }
	pub fn to_f(self) -> Vec2f { Vec2i::new(self.x, self.y).to_f() } // TODO maybe generalise those!
}

impl TileVec {
	pub const fn to_game(self) -> GameVec { GameVec::new(self.x * TILESIZE, self.y * TILESIZE) }
}

impl FluidVec {
	pub const fn to_game(self) -> GameVec { GameVec::new(self.x * FLUID_AFFECT_DIST, self.y * FLUID_AFFECT_DIST) }
}

impl From<TileVec> for GameVec {
	fn from(t: TileVec) -> GameVec {
		t.to_game()
	}
}

impl From<GameVec> for TileVec {
	fn from(t: GameVec) -> TileVec {
		t.to_tile()
	}
}

impl From<FluidVec> for GameVec {
	fn from(t: FluidVec) -> GameVec {
		t.to_game()
	}
}

impl From<GameVec> for FluidVec {
	fn from(t: GameVec) -> FluidVec {
		t.to_fluid()
	}
}
