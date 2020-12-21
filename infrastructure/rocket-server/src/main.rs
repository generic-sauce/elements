#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::config::*;
use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use std::{str, thread};
use rocket_contrib::json::Json;
use std::sync::mpsc::{channel, Sender};
use rocket::State;
use std::sync::{Arc, Mutex};
use rocket_contrib::serve::StaticFiles;

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPushCommit {
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPushHook {
	pub commits: Vec<GithubPushCommit>,
}

#[post("/", data = "<event>")]
fn deploy(sender: State<Arc<Mutex<Sender<()>>>>, event: Json<GithubPushHook>) {
	// parse event
	let deploy_commit = event.commits.iter().any(|c| c.message.contains("#deploy"));

	// TODO: do not call bash script, but use rust bindings
	if deploy_commit {
		sender.lock()
			.unwrap()
			.send(())
			.unwrap();
	}
}

fn main() {
	let (sender, receiver) = channel::<()>();

	thread::spawn(move || {
		loop {
			receiver.recv().unwrap();

			println!("starting new deploy");
			let mut command = Command::new("bash");
			command.arg("-c").arg("./deploy.sh")
				.current_dir("../deploy")
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit());

			match command.output() {
				Ok(_) => println!("Successfully executed deploy.sh"),
				Err(e) => println!("Error executing deploy.sh: {}", e),
			}
		}
	});

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
		.manage(Arc::new(Mutex::new(sender)))
		.mount("/deploy", routes![deploy])
		.mount("/", StaticFiles::from("./static"))
		.launch();
}
