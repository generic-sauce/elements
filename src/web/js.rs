use crate::prelude::*;

// my js code

#[wasm_bindgen] // TODO put into module!
extern {
	// pub fn draw_render_world(rw: JsValue, tilemap_data: Uint8Array, fluidmap_data: Uint8Array);
	pub fn js_init(texture_filenames: JsValue);
	pub fn js_render(draw: JsValue, tilemap_data: Uint8Array, fluidmap_data: Uint8Array, vertex_data: Uint8Array);
	// pub fn js_get_text_size(text: JsValue, scale: JsValue) -> JsValue;
	pub fn load_tilemap(name: &str, closure: &Closure<dyn FnMut(JsValue)>);

	#[wasm_bindgen(js_name = "gamepad_state")]
	fn gamepad_state_js(i: usize) -> JsValue;

	fn peripherals_events_js() -> JsValue;

	pub fn date_now() -> f64;
}

pub fn gamepad_state(i: usize) -> RawGamepadState {
	gamepad_state_js(i).into_serde().unwrap()
}

pub fn peripherals_events() -> Vec<PeripheralsUpdate> {
	#[derive(Serialize, Deserialize)]
	struct Ev {
		key: String,
		peri_type: String,
	}

	peripherals_events_js().into_serde::<Vec<Ev>>()
		.unwrap()
		.into_iter()
		.map(|x| {
			match &*x.peri_type {
				"keydown" => PeripheralsUpdate::KeyPress(Key::A),
				"keyup" => PeripheralsUpdate::KeyRelease(Key::A),
				_ => panic!("unexpected peri_type!")
			}
	}).collect()
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
