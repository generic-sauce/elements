use crate::prelude::*;

pub trait TileMapLoaderBackend {
	fn new(src: &str) -> Self;
	fn poll(&mut self) -> Option<TileMapImage>;
}
