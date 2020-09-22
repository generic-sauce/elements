use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WorldUpdate {
	pub players: [Player; 2],
	pub characters: [Character; 2],
	pub tilemap_update: TileMapUpdate,
	pub fluidmap_update: FluidMapUpdate,
	pub frame_id: u32,
	pub kills: [u32; 2],
	pub restart_state: RestartState,
}

impl World {
	pub fn update(&self) -> WorldUpdate {
		WorldUpdate {
			players: self.players.clone(),
			characters: self.characters.clone(),
			tilemap_update: self.tilemap.update(),
			fluidmap_update: self.fluidmap.update(),
			frame_id: self.frame_id,
			kills: self.kills,
			restart_state: self.restart_state,
		}
	}

	#[allow(unused)]
	pub fn apply_update(&mut self, u: WorldUpdate, handler: &mut impl EventHandler) {
		self.players = u.players;
		self.tilemap.apply_update(u.tilemap_update, handler);
		self.fluidmap.apply_update(u.fluidmap_update);
		self.frame_id = u.frame_id;
		self.kills = u.kills;
		self.restart_state = u.restart_state;

		// This exists to generate a compiler error whenever a field will be added to World.
		let Self { players: _, characters: _, tilemap: _, fluidmap: _, frame_id: _, kills: _, restart_state: _ } = self;
	}
}
