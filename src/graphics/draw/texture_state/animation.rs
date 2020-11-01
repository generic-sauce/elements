use crate::prelude::*;
use super::*;

pub(super) fn create_animation_texture_iter<'a>(device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> impl Iterator<Item=wgpu::Texture> + 'a {
	AnimationId::iter()
		.map(|id| AnimationId::dir(id))
		.flat_map(|dir| std::fs::read_dir(res(&dir)).expect(&format!("could not read animation directory {}", dir)))
		.map(|filepath| {
			let filepath = filepath.expect("could not find file");
			let filepath = filepath.path();
			let filepath = filepath.into_os_string().into_string().expect("could not get filepath");
			filepath
		})
		.filter(|filepath| filepath.ends_with(".png"))
		.map(|filepath| image::open(&filepath).unwrap().flipv().into_rgba())
		.map(move |image| {
			let size: Vec2u = image.dimensions().into();
			let texture = create_texture(device, size);
			write_texture(queue, &texture, size, &image.as_raw()[..]);
			texture
		})
}

impl IntoTextureIndex for Animation {
	fn into_texture_index(self) -> usize {
		let texture_offset = TextureId::iter().count();
		let animation_offset = AnimationId::iter()
			.enumerate()
			.filter(|(index, _)| *index < self.animation_id as usize)
			.map(|(_, id)| id)
			.fold(0, |acc, id| acc + AnimationId::frame_count(id));

		texture_offset + animation_offset + self.texture_index()
	}
}
