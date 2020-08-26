use std::path::PathBuf;

pub fn res(s: &str) -> String {
	let mut p = res_dir();
	p.push(s);
	p.to_str()
		.unwrap()
		.to_string()
}

// well, this is a hack too
fn res_dir() -> PathBuf {
	use std::env;

	let s = env::args()
		.next()
		.unwrap();

	let mut p = PathBuf::from(s);
	p.pop();
	p.pop();
	p.pop();
	p.push("res");
	p
}
