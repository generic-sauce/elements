use crate::prelude::*;

pub struct NativeTileMapLoaderBackend(TileMapImage);

impl TileMapLoaderBackend for NativeTileMapLoaderBackend {
	fn new(src: &str) -> Self {
		NativeTileMapLoaderBackend(TileMapImage::new(src))
	}

	fn poll(&mut self) -> Option<TileMapImage> {
		Some(self.0.clone())
	}
}
