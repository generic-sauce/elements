mod input;
mod app;
mod world;
mod texture_state;
mod shader_state;
mod vec;
mod prelude;
mod context;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
