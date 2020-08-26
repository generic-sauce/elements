#![feature(drain_filter)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod fps_timer;

mod timed_loop;
mod resource;
mod animation_state;
mod input;
mod client;
mod world;
mod texture_state;
mod shader_state;
mod font_state;
mod vec;
mod prelude;
mod draw_context;
mod server;

use crate::prelude::*;

fn main() {
	let server_arg = std::env::args().nth(1);
	match server_arg.as_ref().map(|s| s.as_str()) {
		Some(ip) => Client::new(ip).run(),
		None => Server::new().run(),
	}
}
