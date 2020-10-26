use crate::prelude::*;

// my js code

#[wasm_bindgen] // TODO put into module!
extern {
	pub fn init_js();
	pub fn draw_render_world(rw: JsValue, tilemap_data: Uint8Array, fluidmap_data: Uint8Array);
	pub fn load_tilemap(name: &str, callback: &Closure<dyn Fn(JsValue)>);

	#[wasm_bindgen(js_name = "input_state")]
	fn input_state_js(i: usize) -> JsValue;
}

pub fn input_state(i: usize) -> RawInputState {
	input_state_js(i).into_serde().unwrap()
}


// generic js

#[wasm_bindgen]
extern {
	pub fn setInterval(closure: &Closure<dyn FnMut()>, time_ms: f64);
	pub fn prompt(txt: &str) -> String;

	#[wasm_bindgen(js_namespace = console)]
	pub fn log(txt: &str);
}

pub fn now() -> f64 {
	web_sys::window().unwrap()
		.performance().unwrap()
		.now()
}

