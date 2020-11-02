use super::*;

pub(super) fn create_animation_texture_iter<'a>(device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> impl Iterator<Item=wgpu::Texture> + 'a {
	AnimationId::iter()
		.map(AnimationId::dir)
		.flat_map(|dir| std::fs::read_dir(res(&dir)).unwrap_or_else(|_| panic!("could not read animation directory {}", dir)))
		.map(|filepath| {
			let filepath = filepath.expect("could not find file");
			let filepath = filepath.path();
			filepath.into_os_string().into_string().expect("could not get filepath")
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
