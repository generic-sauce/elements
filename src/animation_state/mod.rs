mod animation;

use crate::prelude::*;
use std::fs::read_dir;

macro_rules! setup {
	($($id:ident : $dir:expr),*$(,)?) => {
		#[derive(Copy, Clone, Debug)]
		#[repr(usize)]
		pub enum AnimationId {
			$($id),*
		}

		impl AnimationState {
			pub fn new() -> AnimationState {
				let mut animation_buffers = Vec::new();
				$(
					animation_buffers.push(AnimationBuffer::from_directory($dir));
				)*
				AnimationState { animation_buffers }
			}
		}
	};
}

impl AnimationBuffer {
	fn from_directory(directory: &str) -> AnimationBuffer {
		let files = read_dir(directory).expect(&format!("Could not read animation directory {}", directory));
		let mut filenames = Vec::new();
		for file in files {
			let file = file.expect("Could not find file");
			let path = file.path();
			let file = path.into_os_string().into_string().expect("Could not get filepath");
			if !file.ends_with(".png") {
				continue;
			}
			filenames.push(file);
		}

		filenames.sort();

		let mut frames = Vec::new();
		for filename in filenames {
			frames.push(Texture::from_file(&filename).expect("could not load file"));
		}

		AnimationBuffer {
			frames,
		}
	}
}

pub struct AnimationState {
	animation_buffers: Vec<AnimationBuffer>,
}

struct AnimationBuffer {
	frames: Vec<SfBox<Texture>>,
}

setup!(
	BluePlayerIdle: "res/images/player_blue/player_idle",
);