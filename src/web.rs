use crate::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn constants() -> JsValue {

	#[derive(Serialize, Deserialize)]
	#[allow(non_snake_case)]
	struct Constants {
		PLAYER_SIZE: GameVec,
	}

	let constants = Constants { PLAYER_SIZE };
	JsValue::from_serde(&constants).unwrap()
}

#[wasm_bindgen]
pub fn init() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn tick_world_nohandler(w: *mut World) {
	unsafe { &mut *w }.tick(&mut ());
}

#[wasm_bindgen]
pub fn world_to_json(w: *const World) -> JsValue {
	JsValue::from_serde(unsafe { & *w }).unwrap()
}

#[wasm_bindgen]
pub fn new_world() -> *mut World {
	Box::leak(Box::new(World::new())) as *mut World
}
