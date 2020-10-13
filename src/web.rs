use crate::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tick_world_nohandler(w: *mut World) {
	unsafe { &mut *w }.tick(&mut ());
}

#[wasm_bindgen]
pub fn new_world() -> *mut World {
	Box::leak(Box::new(World::new())) as *mut World
}
