use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct TileMapUpdateWall {
	owner: usize,
	remaining_lifetime: u32,
	position: TileVec,
}

#[derive(Serialize, Deserialize)]
pub struct TileMapUpdate {
	walls: Vec<TileMapUpdateWall>,
}

impl TileMap {
	pub fn update(&self) -> TileMapUpdate {
		let iter = self.iter()
			.filter_map(|position| {
				if let Tile::Wall{ owner, remaining_lifetime } = self.get(position) {
					Some(
						TileMapUpdateWall {
							owner,
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

	#[must_use]
	pub fn apply_update(&mut self, u: TileMapUpdate) -> Vec<Command> {
		for x in &mut self.tiles {
			if let Tile::Wall { .. } = x {
				*x = Tile::Void;
			}
		}
		for x in u.walls {
			*self.get_mut(x.position) = Tile::Wall { remaining_lifetime: x.remaining_lifetime, owner: x.owner};
		}
		vec![Command::UpdateTileMapTexture]
	}
}
