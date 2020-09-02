use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WorldUpdate {
	pub players: [Player; 2],
	pub tilemap_update: TileMapUpdate,
	pub fluidmap_update: FluidMapUpdate,
	pub frame_id: u32,
	pub kills: [u32; 2],
}

impl World {
	pub fn update(&self) -> WorldUpdate {
		WorldUpdate {
			players: self.players.clone(),
			tilemap_update: self.tilemap.update(),
			fluidmap_update: self.fluidmap.update(),
			frame_id: self.frame_id,
			kills: self.kills,
		}
	}

	#[allow(unused)]
	pub fn apply_update(&mut self, u: WorldUpdate, handler: &mut impl EventHandler) {
		self.players = u.players;
		self.tilemap.apply_update(u.tilemap_update, handler);
		self.fluidmap.apply_update(u.fluidmap_update);
		self.frame_id = u.frame_id;
		self.kills = u.kills;
	}
}
