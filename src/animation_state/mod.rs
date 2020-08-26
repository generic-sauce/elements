pub mod animation;

use crate::prelude::*;
use std::fs::read_dir;


struct AnimationBuffer {
	frames: Vec<SfBox<Texture>>,
	interval: usize,
}

macro_rules! setup {
	($($id:ident : $dir:expr, $interval:expr),*$(,)?) => {
		#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
		#[repr(usize)]
		pub enum AnimationId {
			$($id),*
		}

		impl AnimationState {
			pub fn new() -> AnimationState {
				let mut animation_buffers = Vec::new();
				$(
					animation_buffers.push(AnimationBuffer::from_directory($dir, $interval));
				)*
				AnimationState { animation_buffers }
			}
		}
	};
}

impl AnimationBuffer {
	fn from_directory(directory: &str, interval: usize) -> AnimationBuffer {
		let directory = res(directory);
		let files = read_dir(&directory).expect(&format!("Could not read animation directory {}", directory));
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
			interval,
		}
	}
}

pub struct AnimationState {
	animation_buffers: Vec<AnimationBuffer>,
}

impl AnimationState {
	pub fn get_animation_texture(&self, animation: Animation) -> &'_ Texture {
		let animation_buffer = &self.animation_buffers[animation.animation_id as usize];
		let index = animation.index / animation_buffer.interval;
		&animation_buffer.frames[index]
	}

	pub fn get_frame_count(&self, animation: Animation) -> usize {
		self.animation_buffers[animation.animation_id as usize].frames.len()
	}

	pub fn get_interval(&self, animation: Animation) -> usize {
		self.animation_buffers[animation.animation_id as usize].interval
	}
}

setup!(
	BluePlayerIdle: "images/player_blue/player_idle", 4,
	BluePlayerRun: "images/player_blue/player_run", 4,
	BluePlayerJump: "images/player_blue/player_jump", 4,
	BluePlayerFall: "images/player_blue/player_fall", 4,
	BluePlayerFallSlow: "images/player_blue/player_fall_slow", 4,
	RedPlayerIdle: "images/player_red/player_idle", 4,
	RedPlayerRun: "images/player_red/player_run", 4,
	RedPlayerJump: "images/player_red/player_jump", 4,
	RedPlayerFall: "images/player_red/player_fall", 4,
	RedPlayerFallSlow: "images/player_red/player_fall_slow", 4
);
