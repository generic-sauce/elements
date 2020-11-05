use crate::prelude::*;

#[cfg(feature = "web-client")] mod web;
#[cfg(feature = "web-client")] pub use web::*;

#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

pub trait GraphicsBackend {
	fn draw(&mut self, draw: Draw, tmp_world: Option<&World> /* this is only temporary */);
}
