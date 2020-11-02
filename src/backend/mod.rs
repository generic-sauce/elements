mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

pub trait Backend {
	type InputBackend: InputBackend;
	type GraphicsBackend: GraphicsBackend;
}

#[cfg(feature = "native-client")] mod native {
	use super::*;

	pub struct NativeBackend;

	impl Backend for NativeBackend {
		 type InputBackend = NativeInputBackend;
		 type GraphicsBackend = NativeGraphicsBackend;
	}
}
#[cfg(feature = "native-client")] pub use native::*;
