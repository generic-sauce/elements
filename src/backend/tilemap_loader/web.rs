use crate::prelude::*;

pub struct WebTileMapLoaderBackend {
	_closure: Closure<dyn FnMut(JsValue)>,
	receiver: Receiver<TileMapImage>,
}

impl TileMapLoaderBackend for WebTileMapLoaderBackend {
	fn new(src: &str) -> Self {
		let (sender, receiver) = channel();

		let closure = Closure::<dyn FnMut(JsValue)>::wrap(Box::new(move |tilemap_image| {
			let tilemap_image = tilemap_image.into_serde().unwrap();
			sender.send(tilemap_image).unwrap();
		}));

		load_tilemap(src, &closure);

		Self {
			_closure: closure,
			receiver,
		}
	}

	fn poll(&mut self) -> Option<TileMapImage> {
		match self.receiver.try_recv() {
			Err(TryRecvError::Empty) => None,
			x => Some(x.unwrap()),
		}
	}
}