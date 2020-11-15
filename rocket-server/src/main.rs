#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use std::process::Command;
use std::str;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use rocket_contrib::json::Json;

const ELEMENTS_DEPLOY_DIRECTORY: &str = "/home/sauce/elements_deploy";

pub enum GithubEvent {
	Push
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPushCommit {
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPushHook {
	pub commits: Vec<GithubPushCommit>,
}

impl<'r, 'a> FromRequest<'r, 'a> for GithubEvent {
	type Error = ();

	fn from_request(request: &'r Request<'a>) -> Outcome<Self, ()> {
		let keys = request.headers().get("X-Github-Event").collect::<Vec<_>>();
		if keys.len() != 1 {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		let event = match keys[0] {
			"push" => GithubEvent::Push {},
			_ => { return Outcome::Failure((Status::BadRequest, ())) },
		};

		Outcome::Success(event)
	}
}

#[get("/")]
fn index() -> &'static str {
    "This is the elements frontpage. Have fun :3. Go to /elements/game for the game."
}

#[post("/deploy", data = "<event>")]
fn deploy(event: Json<GithubPushHook>) {
	// parse event
	let deploy_commit = &event.commits.iter().any(|c| c.message.contains("#deploy"));

	// TODO: do not call bash script, but use rust bindings
	if deploy_commit {
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
}

fn main() {
    rocket::ignite().mount("/elements", routes![index, deploy]).launch();
}
