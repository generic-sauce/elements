use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct TileMapUpdateWall {
	team: u8,
	remaining_lifetime: u32,
	position: TileVec,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TileMapUpdate {
	walls: Vec<TileMapUpdateWall>,
}

impl TileMap {
	pub fn update(&self) -> TileMapUpdate {
		let iter = self.iter()
			.filter_map(|position| {
				if let Tile::Wall{ team, remaining_lifetime } = self.get(position) {
					Some(
						TileMapUpdateWall {
							team,
							remaining_lifetime,
							position,
						}
					)
				} else { None }
			});
		TileMapUpdate {
			walls: iter.collect(),
		}
	}

	pub fn apply_update(&mut self, u: TileMapUpdate, handler: &mut impl EventHandler) {
		for x in &mut self.tiles {
			if let Tile::Wall { .. } = x {
				*x = Tile::Void;
			}
		}
		for x in u.walls {
			*self.get_mut(x.position) = Tile::Wall { remaining_lifetime: x.remaining_lifetime, team: x.team};
		}
		handler.tilemap_changed();

		// This exists to generate a compiler error whenever a field will be added to TileMap.
		let Self { tiles: _, size: _, spawn_points: _, details: _ } = self;
	}
}
