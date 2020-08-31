use crate::prelude::*;

struct AnimationBuffer {
	frames: Vec<SfBox<Texture>>,
}

impl AnimationState {
	pub fn new() -> AnimationState {
		let mut animation_buffers = Vec::new();
		for id in AnimationId::iter() {
			let buffer = AnimationBuffer::from_directory(id.dir());
			assert_eq!(buffer.frames.len(), id.frame_count(), "{}", id);
			animation_buffers.push(buffer);
		}
		AnimationState { animation_buffers }
	}
}

impl AnimationBuffer {
	fn from_directory(directory: &str) -> AnimationBuffer {
		use std::fs::read_dir;

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
		}
	}
}

pub struct AnimationState {
	animation_buffers: Vec<AnimationBuffer>,
}

impl AnimationState {
	pub fn get_animation_texture(&self, animation: Animation) -> &'_ Texture {
		let animation_buffer = &self.animation_buffers[animation.animation_id as usize];
		let index = animation.index / animation.animation_id.interval();
		&animation_buffer.frames[index]
	}
}
