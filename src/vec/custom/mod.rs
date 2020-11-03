use crate::prelude::*;

mod serde;

pub struct DefaultParam;
pub type Vec2u = Vec2t<u32, DefaultParam>;
pub type Vec2f = Vec2t<f32, DefaultParam>;

pub struct GameParam;
pub type GameVec = Vec2t<i32, GameParam>;

pub struct TileParam;
pub type TileVec = Vec2t<i32, TileParam>;

pub struct FluidParam;
pub type FluidVec = Vec2t<i32, FluidParam>;

pub struct WindowParam;
pub type WindowVec = Vec2t<u32, WindowParam>;

pub struct SubPixelParam;
pub type SubPixelVec = Vec2t<f32, SubPixelParam>;

pub struct TextureParam;
pub type TextureVec = Vec2t<f32, TextureParam>;

pub const TILESIZE: i32 = 256;

impl GameVec {
	pub const fn to_tile(self) -> TileVec { TileVec::new(self.x / TILESIZE, self.y / TILESIZE) }
	pub const fn to_fluid(self) -> FluidVec { FluidVec::new(self.x / FLUID_AFFECT_DIST, self.y / FLUID_AFFECT_DIST) }
}

impl TileVec {
	pub const fn to_game(self) -> GameVec { GameVec::new(self.x * TILESIZE, self.y * TILESIZE) }
}

impl FluidVec {
	pub const fn to_game(self) -> GameVec { GameVec::new(self.x * FLUID_AFFECT_DIST, self.y * FLUID_AFFECT_DIST) }
}

impl SubPixelVec {
	pub const fn to_window(self) -> WindowVec { WindowVec::new(self.x as u32, self.y as u32) }
}

impl WindowVec {
	pub const fn to_subpixel(self) -> SubPixelVec { SubPixelVec::new(self.x as f32, self.y as f32) }
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

impl From<SubPixelVec> for WindowVec {
	fn from(t: SubPixelVec) -> WindowVec {
		t.to_window()
	}
}
