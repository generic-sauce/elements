use crate::prelude::*;

mod audio;
pub use audio::*;

mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

mod tilemap_loader;
pub use tilemap_loader::*;

mod storage;
pub use storage::*;

pub struct NativeBackend;

impl Backend for NativeBackend {
	type InputBackend = NativeInputBackend;
	type GraphicsBackend = NativeGraphicsBackend;
	type AudioBackend = NativeAudioBackend;
	type SocketBackend = NativeSocketBackend;
	type TileMapLoaderBackend = NativeTileMapLoaderBackend;
	type StorageBackend = NativeStorageBackend;

	fn now() -> f64 {
		std::time::UNIX_EPOCH.elapsed().unwrap().as_micros() as f64 / 1000.
	}

	fn print(s: &str) {
		println!("{}", s);
	}
}
