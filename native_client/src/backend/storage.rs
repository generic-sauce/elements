use crate::prelude::*;

use std::fs::File;
use std::io::ErrorKind;

pub struct NativeStorageBackend;

static CONFIG_FILE: &'static str = "elements.cfg";

fn read() -> String {
	match File::open(CONFIG_FILE) {
		Ok(mut x) => {
			let mut s = String::new();
			x.read_to_string(&mut s).unwrap();
			s
		},
		Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
		Err(e) => panic!("{}", e),
	}
}

// overwrites CONFIG_FILE with contents x
fn write(x: String) {
	let mut f = File::create(CONFIG_FILE).unwrap();
	f.write_all(x.as_bytes()).unwrap();
}

impl StorageBackend for NativeStorageBackend {
	// this doesn't scale for large files, but for now thats fine.
	fn set(&mut self, key: &str, value: &str) {
		assert!(!key.contains(':'));
		let s_in = read();
		let mut s_out = String::new();

		for x in s_in.split('\n') {
			if x.is_empty() { continue; }

			let k = x.split(":").next().unwrap();
			if k != key {
				s_out.push_str(x);
				s_out.push('\n');
			}
		}
		s_out.push_str(key);
		s_out.push(':');
		s_out.push_str(value);

		write(s_out);
	}

	fn get(&self, key: &str) -> Option<String> {
		assert!(!key.contains(':'));

		for x in read().split('\n') {
			if x.is_empty() { continue; }

			let vec: Vec<_> = x.split(':').collect();
			assert!(vec.len() == 2);
			let (k, v) = (vec[0], vec[1]);

			if k == key {
				return Some(v.to_string());
			}
		}

		None
	}
}
