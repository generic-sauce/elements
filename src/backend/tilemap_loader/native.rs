use crate::prelude::*;

pub struct NativeTileMapLoaderBackend(Option<TileMapImage>);

impl TileMapLoaderBackend for NativeTileMapLoaderBackend {
	fn new(src: &str) -> Self {
		NativeTileMapLoaderBackend(Some(TileMapImage::new(src)))
	}

	fn poll(&mut self) -> Option<TileMapImage> {
		self.0.take()
	}
}
