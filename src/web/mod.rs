use crate::prelude::*;

mod render;
pub use render::*;

mod js;
pub use js::*;

#[wasm_bindgen(start)]
pub fn client_main() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	js_init();

	let mut runnable = match &*prompt("menu / local / ip") {
		"menu" => Runnable::Menu,
		"" | "local" => Runnable::Local(Local::<WebBackend>::new(0)),
		ip => Runnable::Client(Client::<WebBackend>::new(ip)),
	};

	let input_backend = WebInputBackend;
	let graphics_backend = WebGraphicsBackend;
	let mut app = App::<WebBackend>::new(graphics_backend, input_backend, runnable.build_menu());
	main_loop(move || app.tick_draw(&mut runnable), 60);
}
