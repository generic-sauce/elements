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

use crate::prelude::*;

pub struct WebBackend;

impl Backend for WebBackend {
    type InputBackend = WebInputBackend;
    type GraphicsBackend = WebGraphicsBackend;
    type AudioBackend = WebAudioBackend;
    type SocketBackend = WebSocketBackend;
    type TileMapLoaderBackend = WebTileMapLoaderBackend;

	fn now() -> f64 {
		date_now()
	}
}
