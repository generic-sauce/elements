mod render;
pub use render::*;

use crate::prelude::*;

#[wasm_bindgen]
pub fn init() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn tick_world(w: *mut World, input_states: JsValue) {
	let mut w = unsafe { &mut *w };

	let input_states: [InputState; 2] = input_states.into_serde().unwrap();

	for p in 0..2 {
		w.players[p].input = input_states[p].clone();
	}

	w.tick(&mut ());
}

#[wasm_bindgen]
pub fn new_world(map: JsValue) -> *mut World {
	let x: TileMapImage = map.into_serde().unwrap();
	Box::leak(Box::new(World::new(0, x))) as *mut World
}
