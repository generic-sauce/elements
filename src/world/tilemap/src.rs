use crate::prelude::*;

pub type RGBA = [u8; 4];

#[derive(Serialize, Deserialize)]
pub struct TileMapImage {
	pub pixels: Vec<Vec<RGBA>>, // pixels[x][y]; pixels[0][0] is left-bot
	pub size: TileVec,
}

pub trait MapSrc {
	fn image(self) -> TileMapImage;
}

impl MapSrc for TileMapImage {
	fn image(self) -> TileMapImage { self }
}

#[cfg(not(feature = "web-client"))]
mod native {
	use crate::prelude::*;
	use super::{RGBA, TileMapImage};

	impl MapSrc for &'static str {
		fn image(self) -> TileMapImage {
			use image::{GenericImageView, Rgba};

			let filename = res(self);

			let image = image::open(filename).unwrap();
			let (width, height) = image.dimensions();
			let mut pixels: Vec<Vec<RGBA>> = (0..width)
				.map(|_|
					(0..height).map(|_| [0; 4])
						       .collect()
				).collect();

			for y in 0..height {
				for x in 0..width {
					let Rgba(rgba) = image.get_pixel(x as u32, y as u32);
					pixels[x as usize][(height - y - 1) as usize] = rgba;
				}
			}

			let size = TileVec::new(width as i32, height as i32);
			TileMapImage {
				pixels,
				size,
			}
		}
	}
}
