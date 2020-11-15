#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::process::Command;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;

const ELEMENTS_DEPLOY_DIRECTORY: &str = "/home/sauce/elements_deploy";

#[derive(Debug)]
pub struct GithubPushHook {
}

impl<'r, 'a> FromRequest<'r, 'a> for GithubPushHook {
	type Error = ();

	fn from_request(request: &'r Request<'a>) -> Outcome<Self, ()> {
		let keys = request.headers().get("X-Github-Event").collect::<Vec<_>>();
		if keys.len() != 1 {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		let event = match keys[0] {
			"push" => GithubPushHook {},
			_ => { return Outcome::Failure((Status::BadRequest, ())) },
		};

		Outcome::Success(event)
	}
}

#[get("/")]
fn index() -> &'static str {
    "This is the elements frontpage. Have fun :3. Go to /elements/game for the game."
}

#[post("/deploy")]
fn deploy(event: Option<GithubPushHook>) {
	println!("got event: {:?}", event);
	match Command::new("bash").arg("-c").arg("./deploy.sh").current_dir(ELEMENTS_DEPLOY_DIRECTORY).output() {
		Ok(x) => {
			println!("Deployed.status: {}", x.status);
			if let Ok(text) = str::from_utf8(x.stdout) {
				println!("Deployed.stdout: {}", text);
			}
		}
		Err(e) => { println!("Error executing deploy.sh: {}", e) }
	}
}

fn main() {
    rocket::ignite().mount("/elements", routes![index, deploy]).launch();
}
