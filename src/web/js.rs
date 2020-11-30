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
		peri_type: String,
		key: Option<String>,
		movement: Option<SubPixelVec>,
		button: Option<u8>,
	}

	peripherals_events_js().into_serde::<Vec<Ev>>()
		.unwrap()
		.into_iter()
		.filter_map(|x|
			match &*x.peri_type {
				"keydown" => js_to_rust_key(&*x.key.unwrap()).map(PeripheralsUpdate::KeyPress),
				"keyup" => js_to_rust_key(&*x.key.unwrap()).map(PeripheralsUpdate::KeyRelease),
				"mousedown" => Some(PeripheralsUpdate::KeyPress(js_to_rust_button(x.button.unwrap()))),
				"mouseup" => Some(PeripheralsUpdate::KeyRelease(js_to_rust_button(x.button.unwrap()))),
				"mousemove" => Some(PeripheralsUpdate::MouseMove(x.movement.unwrap())),
				_ => panic!("unexpected peri_type!"),
			}
		).collect()
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

fn js_to_rust_key(js_key: &str) -> Option<Key> {
	Some(match js_key {
		"a" => Key::A,
		"d" => Key::D,
		"w" => Key::W,
		"e" => Key::E,
		"r" => Key::R,
		"f" => Key::F,
		"q" => Key::Q,
		"Space" => Key::Space,
		_ => None?, // TODO
	})
}

fn js_to_rust_button(js_button: u8) -> Key {
	match js_button {
		0 => Key::LeftMouse,
		1 => Key::MiddleMouse,
		2 => Key::RightMouse,
		x => Key::OtherMouse(x),
	}
}