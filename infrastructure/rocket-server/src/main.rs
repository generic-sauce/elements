#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::config::*;
use rocket::response::Redirect;
use std::process::{Command, Stdio};
use std::{str, thread};
use rocket_contrib::json::Json;
use std::sync::mpsc::{channel, Sender};
use rocket::State;
use std::sync::{Arc, Mutex};
use rocket_contrib::serve::StaticFiles;

fn main() {
	// redirect HTTP -> HTTPS
	thread::spawn(|| {
		let config = Config::build(Environment::Production)
			.port(80)
			.unwrap();


		#[get("/")]
		fn redirect() -> Redirect {
			Redirect::to("https://generic-sauce.de") // TODO make generic
		}

		rocket::custom(config)
			.mount("/", routes![redirect])
			.launch();
	});

    rocket::ignite()
		.mount("/", StaticFiles::from("./static"))
		.launch();
}
