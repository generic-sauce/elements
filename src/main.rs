mod animation_state;
mod input;
mod app;
mod world;
mod texture_state;
mod shader_state;
mod font_state;
mod vec;
mod prelude;
mod draw_context;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
