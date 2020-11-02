use crate::prelude::*;

mod webclient;
pub use webclient::*;

mod weblocal;
pub use weblocal::*;

mod render;
pub use render::*;

mod js;
pub use js::*;

const FPS: f64 = 60.0;

pub struct Webapp {
	data: WebappData,
	mode: WebappMode,
}

pub struct WebappData {
	tick_counter: u32,
	start_time: f64,
	pub world: World,
}

enum WebappMode {
	Menu,
	Client(WebClient),
	Local(WebLocal),
}

impl Webapp {
	pub fn schedule(mut self) {
		let cb = Closure::<dyn FnMut()>::wrap(Box::new(move || { self.run_once(); }));
		let leaked_cb = Box::leak(Box::new(cb)); // TODO
		setInterval(leaked_cb, 1000.0/FPS);
	}

	fn fps(&self) -> f64 {
		self.data.tick_counter as f64 * 1000.0 / (now() - self.data.start_time)
	}

	fn run_once(&mut self) {
		for _ in 0..10 {
			if self.fps() >= FPS { break; }

			self.tick();
			self.data.tick_counter += 1;
		}
		self.draw();
	}

	pub fn tick(&mut self) {
		match &mut self.mode {
			WebappMode::Menu => {
				self.mode = match &*prompt("local / ip") {
					"local" => WebappMode::Local(WebLocal::new()),
					ip => WebappMode::Client(WebClient::new(ip)),
				};
			},
			WebappMode::Client(c) => c.tick(&mut self.data),
			WebappMode::Local(l) => l.tick(&mut self.data),
		}
	}

	pub fn draw(&self) {
		RenderWorld::draw(&self.data.world);
	}
}

#[wasm_bindgen(start)]
pub fn client_main() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	init_js();

	let cb = Closure::<dyn Fn(JsValue)>::wrap(Box::new(|map_src| {
			let map_src: TileMapImage = map_src.into_serde().unwrap();
			let world = World::new_by_source(0,  map_src);
			Webapp {
				data: WebappData {
					tick_counter: 0,
					start_time: now(),
					world,
				},
				mode: WebappMode::Menu,
			}.schedule();
	}));
	let leaked_cb = Box::leak(Box::new(cb)); // TODO
	load_tilemap("map/map02.png", leaked_cb);
}
