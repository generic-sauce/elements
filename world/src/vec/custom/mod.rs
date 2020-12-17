use crate::prelude::*;

mod serde;

pub struct DefaultParam;
pub type Vec2f = Vec2t<f32, DefaultParam>;

/* from (0, 0) to tilemap size
 * corresponds to world coordinates
 * 256 units per tile
 */
pub struct GameParam;
pub type GameVec = Vec2t<i32, GameParam>;

/* from (0, 0) to tilemap size
 * corresponds to world coordinates
 * 1 unit per tile
 */
pub struct TileParam;
pub type TileVec = Vec2t<i32, TileParam>;

/* from (0, 0) to fluidmap size
 * corresponds to world coordinates
 * 1 unit per fluid grid cell
 */
pub struct FluidParam;
pub type FluidVec = Vec2t<i32, FluidParam>;

/* from (0, 0) to window size
 * corresponds to window coordinates
 */
pub struct PixelParam;
pub type PixelVec = Vec2t<u32, PixelParam>;

/* from (0, 0) to window size
 * corresponds to window coordinates
 */
pub struct SubPixelParam;
pub type SubPixelVec = Vec2t<f32, SubPixelParam>;

/* from (0, 0) to (1, 1)
 * corresponds to texture coordinates
 */
pub struct TextureParam;
pub type TextureVec = Vec2t<f32, TextureParam>;

pub const TILESIZE: i32 = 256;

impl GameVec {
	pub const fn to_tile(self) -> TileVec { TileVec::new(self.x / TILESIZE, self.y / TILESIZE) }
	pub const fn to_fluid(self) -> FluidVec { FluidVec::new(self.x / FLUID_AFFECT_DIST, self.y / FLUID_AFFECT_DIST) }
}

impl TileVec {
	pub const fn to_game(self) -> GameVec { GameVec::new(self.x * TILESIZE, self.y * TILESIZE) }
	pub const fn to_game_center(self) -> GameVec { GameVec::new(self.x * TILESIZE + TILESIZE/2, self.y * TILESIZE + TILESIZE/2) }
}

impl FluidVec {
	pub const fn to_game(self) -> GameVec { GameVec::new(self.x * FLUID_AFFECT_DIST, self.y * FLUID_AFFECT_DIST) }
}

impl SubPixelVec {
	pub const fn to_pixel(self) -> PixelVec { PixelVec::new(self.x as u32, self.y as u32) }
}

impl PixelVec {
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

impl From<SubPixelVec> for PixelVec {
	fn from(t: SubPixelVec) -> PixelVec {
		t.to_pixel()
	}
}
