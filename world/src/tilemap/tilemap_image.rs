use crate::prelude::*;

pub type Rgba = [u8; 4];

#[derive(Serialize, Deserialize, Clone)]
pub struct TileMapImage {
	pub pixels: Vec<Vec<Rgba>>, // pixels[x][y]; pixels[0][0] is left-bot
	pub size: TileVec,
}

pub const DEFAULT_TILEMAP: &'static str = "map/map04.png";


#[cfg(not(target_arch = "wasm32"))]
pub fn load_tilemap_image(src: &str) -> TileMapImage {
	use image::GenericImageView;

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
