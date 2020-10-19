use crate::prelude::*;

pub struct GraphicsWorld {
	pub tilemap_size: TileVec,
	// fluidmap_size: FluidVec,
	pub players: [Player; 2],
	pub tilemap_data: Vec<u8>,
}

impl GraphicsWorld {
	pub fn new(tilemap_size: TileVec, players: [Player; 2], tilemap: &TileMap) -> GraphicsWorld {
		let tilemap_data: Vec<u8> = tilemap.iter()
			.map(|p| tilemap.get(p))
			.map(|t| match t {
				Tile::Void => 0,
				Tile::Ground => 1,
				Tile::Wall { owner, .. } => 2 + owner as u8,
			})
			.collect();

		GraphicsWorld {
			tilemap_size,
			players,
			tilemap_data,
		}
	}
}

impl Graphics {
	pub fn draw_players(&mut self, world: &GraphicsWorld) {
		for p in &world.players {
			let player_size = PLAYER_SIZE.to_canvas(world.tilemap_size);
			self.triangles.draw_sprite(p.left_bot.to_canvas(world.tilemap_size), player_size, Some(wgpu::Color::RED));
		}
	}
}
