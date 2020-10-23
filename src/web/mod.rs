use crate::prelude::*;

mod render;
pub use render::*;

mod webclient;
pub use webclient::*;

mod js;
pub use js::*;

#[wasm_bindgen(start)]
pub fn client_main() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	init_js();

	let cb = Closure::<dyn Fn(JsValue)>::wrap(Box::new(|map_src| {
			let map_src: TileMapImage = map_src.into_serde().unwrap();
			let client = WebClient::new("localhost", map_src); // TODO non-hardcode localhost
			client.schedule();
	}));
	let leaked_cb = Box::leak(Box::new(cb)); // TODO
	load_tilemap("map/map02.png", leaked_cb);
}