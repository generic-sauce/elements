mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

mod audio;
pub use audio::*;

pub trait Backend {
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
