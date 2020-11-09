use crate::prelude::*;

// my js code

#[wasm_bindgen] // TODO put into module!
extern {
	// pub fn draw_render_world(rw: JsValue, tilemap_data: Uint8Array, fluidmap_data: Uint8Array);
	pub fn js_init(texture_filenames: JsValue);
	pub fn js_render(draw: JsValue, tilemap_data: Uint8Array, fluidmap_data: Uint8Array);
	pub fn load_tilemap(name: &str, closure: &Closure<dyn FnMut(JsValue)>);

	#[wasm_bindgen(js_name = "input_state")]
	fn input_state_js(i: usize) -> JsValue;
}

pub fn input_state(i: usize) -> RawGamepadState {
	input_state_js(i).into_serde().unwrap()
}


// generic js

#[wasm_bindgen]
extern {
	pub fn setInterval(closure: &Closure<dyn FnMut()>, time_ms: f64);
	pub fn prompt(txt: &str) -> String;
	pub fn alert(txt: &str);

	#[wasm_bindgen(js_namespace = console)]
	pub fn log(txt: &str);
}
