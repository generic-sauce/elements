#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "native-client")] mod client;
#[cfg(feature = "native-client")] mod client_world;
#[cfg(feature = "native-client")] mod app;
#[cfg(feature = "native-client")] mod local;
#[cfg(feature = "native-client")] mod draw;
#[cfg(feature = "native-client")] mod menu;
#[cfg(feature = "native-client")] mod graphics;

#[cfg(feature = "web-client")] mod web;

#[cfg(not(feature = "web-client"))] mod server;
#[cfg(not(feature = "web-client"))] mod resource;
#[cfg(not(feature = "web-client"))] mod timed_loop;

#[macro_use]
mod fps_timer;

mod net;
mod rng;
mod world;
mod vec;
mod animation;
mod prelude;
mod input;
mod backend;
