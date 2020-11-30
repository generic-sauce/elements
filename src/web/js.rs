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
		repeat: Option<bool>,
	}

	let mut out = Vec::new();
	for x in peripherals_events_js().into_serde::<Vec<Ev>>().unwrap() {
		match &*x.peri_type {
			"keydown" => {
				let k = &*x.key.unwrap();
				if !x.repeat.unwrap() {
					if let Some(key) = js_to_rust_key(k) {
						out.push(PeripheralsUpdate::KeyPress(key));
					}
				}

				// PeripheralsUpdate::Text
				match k {
					"Backspace" => out.push(PeripheralsUpdate::Text(Character::Backspace)),
					"Delete" => out.push(PeripheralsUpdate::Text(Character::Delete)),
					"ArrowRight" => out.push(PeripheralsUpdate::Text(Character::Right)),
					"ArrowLeft" => out.push(PeripheralsUpdate::Text(Character::Left)),
					k => {
						let chrs: Vec<_> = k.chars().collect();
						if chrs.len() == 1 { // something like "a" or "-", and no symbolic thingy
							out.push(PeripheralsUpdate::Text(Character::Char(chrs[0])));
						}
					},
				}
			},
			"keyup" => {
				let k = &*x.key.unwrap();
				if let Some(key) = js_to_rust_key(k) {
					out.push(PeripheralsUpdate::KeyRelease(key));
				}
			},
			"mousedown" => out.push(PeripheralsUpdate::KeyPress(js_to_rust_button(x.button.unwrap()))),
			"mouseup" => out.push(PeripheralsUpdate::KeyRelease(js_to_rust_button(x.button.unwrap()))),
			"mousemove" => out.push(PeripheralsUpdate::MouseMove(x.movement.unwrap())),
			_ => panic!("unexpected peri_type!"),
		}
	}

	out
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
