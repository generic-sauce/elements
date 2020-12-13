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
}

#[cfg(feature = "native-client")] mod native {
	use super::*;
	use crate::prelude::*;

	pub struct NativeBackend;

	impl Backend for NativeBackend {
		type InputBackend = NativeInputBackend;
		type GraphicsBackend = NativeGraphicsBackend;
		type AudioBackend = NativeAudioBackend;
		type SocketBackend = NativeSocketBackend;
		type TileMapLoaderBackend = NativeTileMapLoaderBackend;
	}
}
#[cfg(feature = "native-client")] pub use native::*;

#[cfg(feature = "web-client")] mod web {
	use super::*;

	pub struct WebBackend;

	impl Backend for WebBackend {
		type InputBackend = WebInputBackend;
		type GraphicsBackend = WebGraphicsBackend;
		type AudioBackend = WebAudioBackend;
		type SocketBackend = WebSocketBackend;
		type TileMapLoaderBackend = WebTileMapLoaderBackend;
	}
}
#[cfg(feature = "web-client")] pub use web::*;
