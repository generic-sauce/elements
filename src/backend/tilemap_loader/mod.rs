use crate::prelude::*;

#[cfg(feature = "web-client")] mod web;
#[cfg(feature = "web-client")] pub use web::*;

#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

pub trait TileMapLoaderBackend {
	fn new(src: &str) -> Self;
	fn poll(&mut self) -> Option<TileMapImage>;
}
