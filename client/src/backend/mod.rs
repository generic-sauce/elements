use crate::prelude::*;

mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

mod audio;
pub use audio::*;

mod socket;
pub use socket::*;

mod tilemap_loader;
pub use tilemap_loader::*;

pub trait Backend: 'static {
	type InputBackend: InputBackend;
	type GraphicsBackend: GraphicsBackend;
	type AudioBackend: AudioBackend;
	type SocketBackend: SocketBackend;
	type TileMapLoaderBackend: TileMapLoaderBackend;
	fn now() -> f64;
}
