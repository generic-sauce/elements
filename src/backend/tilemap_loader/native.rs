use crate::prelude::*;

pub struct NativeTileMapLoaderBackend(Option<TileMapImage>);

impl TileMapLoaderBackend for NativeTileMapLoaderBackend {
	fn new(src: &str) -> Self {
		NativeTileMapLoaderBackend(Some(load_tilemap_image(src)))
	}

	fn poll(&mut self) -> Option<TileMapImage> {
		self.0.take()
	}
}
