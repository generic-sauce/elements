use world::prelude::*;
use image::GenericImageView;

pub fn load_tilemap_image(src: &str) -> TileMapImage {
	let filename = res(src);

	let image = image::open(filename).unwrap();
	let (width, height) = image.dimensions();
	let mut pixels: Vec<Vec<Rgba>> = (0..width)
		.map(|_| (0..height)
			.map(|_| [0; 4])
			.collect()
		)
		.collect();

	for y in 0..height {
		for x in 0..width {
			let image::Rgba(rgba) = image.get_pixel(x as u32, y as u32);
			pixels[x as usize][(height - y - 1) as usize] = rgba;
		}
	}

	let size = TileVec::new(width as i32, height as i32);

	TileMapImage {
		pixels,
		size,
	}
}
