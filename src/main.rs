mod app;
mod world;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
