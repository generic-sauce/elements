use crate::prelude::*;

mod render;
pub use render::*;

mod js;
pub use js::*;

#[wasm_bindgen]
pub fn client_main(answer: &str) {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	let texture_filenames: Vec<_> = texture_filenames_iter().collect();
	let texture_filenames = JsValue::from_serde(&texture_filenames).unwrap();
	js_init(texture_filenames);

	let mut runnable = match answer {
		"menu" => Runnable::Menu,
		"" | "local" => Runnable::Local(Local::<WebBackend>::new(0)),
		ip => Runnable::Client(Client::<WebBackend>::new(ip)),
	};

	let input_backend = WebInputBackend;
	let graphics_backend = WebGraphicsBackend;
	let mut app = App::<WebBackend>::new(graphics_backend, input_backend, runnable.build_menu());
	main_loop(move || app.tick_draw(&mut runnable), 60);
}
