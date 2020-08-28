#![feature(drain_filter)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod fps_timer;

mod timed_loop;
mod world;
mod vec;
mod prelude;
mod server;
mod net;
mod animation;
mod resource;

pub use crate::prelude::*;
