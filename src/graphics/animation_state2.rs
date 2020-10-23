use crate::prelude::*;

struct AnimationFrames {
	textures: Vec<wgpu::Texture>,
	texture_views: Vec<wgpu::TextureView>,
}

pub struct AnimationState2 {
	animations: Vec<AnimationFrames>,
}
impl AnimationState2 {
	pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> AnimationState2 {
		let animations: Vec<AnimationFrames> = AnimationId::iter()
			.map(|id| AnimationId::dir(id))
			.map(|dir| std::fs::read_dir(res(&dir)).expect(&format!("could not read animation directory {}", dir)))
			.map(|filepaths| {
				filepaths.into_iter()
					.map(|filepath| {
						let filepath = filepath.expect("could not find file");
						let filepath = filepath.path();
						let filepath = filepath.into_os_string().into_string().expect("could not get filepath");
						filepath
					})
					.filter(|filepath| filepath.ends_with(".png"))
					.map(|filepath| image::open(&filepath).unwrap().flipv().into_rgba())
					.map(|image| {
						let size: Vec2u = image.dimensions().into();
						let texture = create_texture(device, size);
						write_texture(queue, &texture, size, &image.as_raw()[..]);
						texture
					})
			})
			.map(|frames|{
				let textures: Vec<wgpu::Texture> = frames.collect();
				let texture_views = textures.iter()
					.map(|texture| texture.create_view(&wgpu::TextureViewDescriptor::default()))
					.collect();

				AnimationFrames {
					textures,
					texture_views,
				}
			})
			.collect();
	
		AnimationState2 {
			animations,
		}
	}

	#[allow(unused)]
	pub fn texture(&self, animation: Animation) -> &wgpu::Texture {
		&self.animations[animation.animation_id as usize].textures[animation.index]
	}

	#[allow(unused)]
	pub fn texture_view(&self, animation: Animation) -> &wgpu::TextureView {
		&self.animations[animation.animation_id as usize].texture_views[animation.index]
	}

	#[allow(unused)]
	pub fn texture_count(&self) -> u32 {
		self.animations.iter()
			.map(|animation| animation.textures.len() as u32)
			.fold(0u32, |acc, x| acc + x)
	}
}
