use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
pub fn res(s: &str) -> String {
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

	let mut p = res_dir();
	p.push(s);
	p.to_str()
		.unwrap()
		.to_string()
}

#[cfg(target_arch = "wasm32")]
pub fn res(s: &str) -> String {
	format!("res/{}", s)
}