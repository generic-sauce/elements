#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
pub fn res(s: &str) -> String {
	let mut p = res_dir();
	p.push(s);
	p.to_str()
		.unwrap()
		.to_string()
}

#[cfg(feature = "server")]
fn res_dir() -> PathBuf {
	let s = std::env::args()
		.next()
		.unwrap();

	let mut p = PathBuf::from(s);
	p.pop();
	p.pop();
	p.pop();
	p.push("res");
	p
}

#[cfg(feature = "web-client")]
pub fn res(s: &str) -> String {
	format!("res/{}", s)
}
