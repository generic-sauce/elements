#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use std::process::Command;
use std::{str, thread};
use rocket_contrib::json::Json;
use std::sync::mpsc::{channel, Sender};
use rocket::State;
use std::sync::{Arc, Mutex};

const ELEMENTS_DEPLOY_DIRECTORY: &str = "/home/sauce/elements_deploy";

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPushCommit {
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPushHook {
	pub commits: Vec<GithubPushCommit>,
}

#[get("/")]
fn index() -> &'static str {
    "This is the elements frontpage. Have fun :3. Go to /elements/game for the game."
}

#[post("/deploy", data = "<event>")]
fn deploy(sender: State<Arc<Mutex<Sender<()>>>>, event: Json<GithubPushHook>) {
	// parse event
	let deploy_commit = &event.commits.iter().any(|c| c.message.contains("#deploy"));

	// TODO: do not call bash script, but use rust bindings
	if *deploy_commit {
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
			match Command::new("bash").arg("-c").arg("./deploy.sh").current_dir(ELEMENTS_DEPLOY_DIRECTORY).output() {
				Ok(x) => {
					println!("Deployed.status: {}", x.status);
					if let Ok(text) = str::from_utf8(&x.stdout) {
						println!("Deployed.stdout: {}", text);
					}
				}
				Err(e) => { println!("Error executing deploy.sh: {}", e) }
			}
		}
	});

    rocket::ignite().manage(Arc::new(Mutex::new(sender))).mount("/elements", routes![index, deploy]).launch();
}
