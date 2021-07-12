use crate::prelude::*;

mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

mod audio;
pub use audio::*;

mod tilemap_loader;
pub use tilemap_loader::*;

mod storage;
pub use storage::*;

// The SocketBackend is defined in the networking crate!

pub trait Backend: 'static {
	type InputBackend: InputBackend;
	type GraphicsBackend: GraphicsBackend;
	type AudioBackend: AudioBackend;
	type SocketBackend: SocketBackend;
	type TileMapLoaderBackend: TileMapLoaderBackend;
	type StorageBackend: StorageBackend;
	fn now() -> f64;
}
