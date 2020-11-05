mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

mod audio;
pub use audio::*;

pub trait Backend: 'static {
	type InputBackend: InputBackend;
	type GraphicsBackend: GraphicsBackend;
	type AudioBackend: AudioBackend;
}

#[cfg(feature = "native-client")] mod native {
	use super::*;

	pub struct NativeBackend;

	impl Backend for NativeBackend {
		type InputBackend = NativeInputBackend;
		type GraphicsBackend = NativeGraphicsBackend;
		type AudioBackend = NativeAudioBackend;
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
	}
}
#[cfg(feature = "web-client")] pub use web::*;
