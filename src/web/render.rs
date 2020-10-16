use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RenderWorld {
	tilemap_size: TileVec,
	tilemap_data: Vec<u8>,
	fluidmap_size: FluidVec,
	fluidmap_data: Vec<u8>,
	players: [Player; 2],
	player_size: GameVec,
}

impl World {
	pub(super) fn render_world(&self) -> RenderWorld {
		let tilemap_size = self.tilemap.size;
		let fluidmap_size = self.fluidmap.size;

		let mut tilemap_data = Vec::new();

		for y in 0..tilemap_size.y {
			for x in 0..tilemap_size.x {
				let color: [u8; 4] =
					match self.tilemap.get(TileVec::new(x, y)) {
						Tile::Ground => [50, 50, 10, 255],
						Tile::Void => [200, 200, 255, 255],
						Tile::Wall { owner: 0, .. } => [0, 0, 40, 255],
						Tile::Wall { owner: 1, .. } => [40, 0, 0, 255],
						Tile::Wall { owner: _, .. } => panic!("more than two players are not yet supported"),
					};
				tilemap_data.extend(&color[..]);
			}
		}

		let mut fluidmap_data = Vec::new();
		for y in 0..fluidmap_size.y {
			for x in 0..fluidmap_size.x {
				let fluids = self.fluidmap.index(FluidVec::new(x, y));
				let mut color = [0; 4];
				for f in fluids {
					color[3] = 255;
					if f.owner == 0 {
						color[2] = 255;
					} else if f.owner == 1 {
						color[0] = 255;
					} else {
						panic!("more than two players are not yet supported (2)");
					}
				}
				fluidmap_data.extend(&color[..]);
			}
		}

		RenderWorld {
			tilemap_size,
			fluidmap_size,
			tilemap_data,
			fluidmap_data,
			players: self.players.clone(),
			player_size: PLAYER_SIZE,
		}

	}
}
