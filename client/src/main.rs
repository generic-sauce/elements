#![feature(drain_filter)]

mod animation_state;
mod client;
mod texture_state;
mod shader_state;
mod font_state;
mod prelude;
mod draw_context;
mod app;
mod local;
mod draw;
mod input;

use crate::prelude::*;

fn main() {
	let server_arg = std::env::args().nth(1);
	match server_arg.as_ref().map(|s| s.as_str()) {
		Some("server") => Server::new().run(),
		Some(ip) => Client::new(ip).run(),
		None => Local::new().run(),
	}
}
