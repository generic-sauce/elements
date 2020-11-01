use crate::prelude::*;

mod graphics;
pub use graphics::*;

mod input;
pub use input::*;

pub trait Backend {
	type InputBackend: InputBackend;
	type GraphicsBackend: GraphicsBackend;
}

pub struct NativeBackend;

impl Backend for NativeBackend {
	 type InputBackend = NativeInputBackend;
	 type GraphicsBackend = NativeGraphicsBackend;
}
