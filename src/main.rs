#![feature(drain_filter)]
#![feature(const_fn)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "client")] mod animation_state;
#[cfg(feature = "client")] mod client;
#[cfg(feature = "client")] mod texture_state;
#[cfg(feature = "client")] mod shader_state;
#[cfg(feature = "client")] mod font_state;
#[cfg(feature = "client")] mod draw_context;
#[cfg(feature = "client")] mod app;
#[cfg(feature = "client")] mod local;
#[cfg(feature = "client")] mod draw;
#[cfg(feature = "client")] mod input;
#[cfg(feature = "client")] mod window_vec;
#[cfg(feature = "client")] mod px;
#[cfg(feature = "client")] mod menu;

#[macro_use]
mod fps_timer;

mod timed_loop;
mod world;
mod vec;
mod server;
mod net;
mod animation;
mod resource;
mod prelude;

use crate::prelude::*;

#[cfg(feature = "client")]
fn main() {
	let server_arg = std::env::args().nth(1);
	match server_arg.as_ref().map(|s| s.as_str()) {
		Some("server") => Server::new().run(),
		Some(ip) => App::new().run_client(ip),
		None => App::new().run_local(),
	}
}

#[cfg(not(feature = "client"))]
fn main() {
	Server::new().run();
}
