#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::process::Command;

const ELEMENTS_DEPLOY_DIRECTORY: &str = "/home/sauce/elements_deploy";

#[get("/")]
fn index() -> &'static str {
    "This is the elements frontpage. Have fun :3. Go to /elements/game for the game."
}

#[post("/deploy")]
fn deploy() {
	match Command::new("bash").arg("-c").arg("deploy.sh").current_dir(ELEMENTS_DEPLOY_DIRECTORY).output() {
		Ok(x) => { println!("Deployed: {:?}", x) }
		Err(e) => { println!("Error executing deploy.sh: {}", e) }
	}
}

fn main() {
    rocket::ignite().mount("/elements", routes![index, deploy]).launch();
}
