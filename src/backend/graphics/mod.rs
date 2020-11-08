use crate::prelude::*;

#[cfg(feature = "web-client")] mod web;
#[cfg(feature = "web-client")] pub use web::*;

#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

pub trait GraphicsBackend {
	fn submit(&mut self, draw: Draw);
	fn get_text_size(&self, text: &str, scale: f32) -> CanvasVec;
}
