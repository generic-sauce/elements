#![feature(drain_filter)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod fps_timer;

mod animation_state;
mod client;
mod texture_state;
mod shader_state;
mod font_state;
mod draw_context;
mod app;
mod local;
mod draw;
mod input;
mod window_vec;

mod timed_loop;
mod world;
mod vec;
mod server;
mod net;
mod animation;
mod resource;

mod prelude;

use crate::prelude::*;

fn main() {
	let server_arg = std::env::args().nth(1);
	match server_arg.as_ref().map(|s| s.as_str()) {
		Some("server") => Server::new().run(),
		Some(ip) => Client::new(ip).run(),
		None => Local::new().run(),
	}
}
