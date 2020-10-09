#[macro_use]
extern crate serde_derive;

mod local;
mod input;
mod window_vec;

#[macro_use]
mod fps_timer;

mod timed_loop;
mod world;
mod vec;
mod prelude;

use crate::prelude::*;
