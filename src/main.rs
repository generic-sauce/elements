mod input;
mod app;
mod world;
mod texture_state;
mod vec;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
