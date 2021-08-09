use crate::prelude::*;

use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct NativeStorageBackend {
	cache: HashMap<String, String>,
}

impl NativeStorageBackend {
	pub fn new() -> NativeStorageBackend {
		let mut cache = HashMap::new();
		let s_in = read();

		for x in s_in.split('\n') {
			if x.is_empty() { continue; }

			match &x.split(":").collect::<Vec<_>>()[..] {
				&[k, v] => {
					cache.insert(k.to_string(), v.to_string());
				},
				_ => panic!("hey")
			}

		}
		NativeStorageBackend {
			cache
		}
	}
}

fn config_file() -> PathBuf {
	let mut p = dirs::config_dir().unwrap();
	p.push("elements.cfg");
	p
}

fn read() -> String {
	match File::open(config_file()) {
		Ok(mut x) => {
			let mut s = String::new();
			x.read_to_string(&mut s).unwrap();
			s
		},
		Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
		Err(e) => panic!("{}", e),
	}
}

// overwrites config_file() with contents x
fn write(x: String) {
	let mut f = File::create(config_file()).unwrap();
	f.write_all(x.as_bytes()).unwrap();
}

impl StorageBackend for NativeStorageBackend {
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

		self.cache.insert(key.to_string(), value.to_string());

		write(s_out);
	}

	fn get(&self, key: &str) -> Option<String> {
		assert!(!key.contains(':'));
		self.cache.get(key).map(|v| v.to_string())
	}
}
