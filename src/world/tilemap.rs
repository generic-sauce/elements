use sfml::system::Vector2i;
use sfml::graphics::{Color, Texture};

#[derive(Clone, Copy)]
pub enum Tile {
	Void,
	Ground,
}

pub struct TileMap {
	tiles: Vec<Tile>,
}

impl TileMap {
	pub fn new(filename: &str) -> TileMap {
		let s = TileMap::size();
		let mut tiles = Vec::with_capacity((s.x * s.y) as usize);

		let texture = Texture::from_file(filename).unwrap();
		let image = texture.copy_to_image().unwrap();

		for y in 0..s.y {
			for x in 0..s.x {
				let tile = match image.pixel_at(x as u32, y as u32) {
					Color { r, g, b, a } => Tile::Void, // TODO do anything else than void!
				};
				tiles.push(tile);
			}
		}

		TileMap {
			tiles
		}
	}

	pub fn size() -> Vector2i {
		Vector2i::new(20, 20)
	}

	pub fn get_mut(&mut self, v: Vector2i) -> &'_ mut Tile {
		&mut self.tiles[(v.x + v.y * TileMap::size().x) as usize]
	}

	pub fn get(&self, v: Vector2i) -> Tile {
		self.tiles[(v.x + v.y * TileMap::size().x) as usize]
	}
}
