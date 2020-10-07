#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

#[cfg(feature = "client")]
fn main() {
	let server_arg = std::env::args().nth(1);
	match server_arg.as_ref().map(|s| s.as_str()) {
		Some("server") => Server::new().run(),
		Some("menu") => App::new().run_menu(),
		Some(ip) => App::new().run_client(ip),
		None => App::new().run_local(),
	}
}

#[cfg(not(feature = "client"))]
fn main() {
	Server::new().run();
}
