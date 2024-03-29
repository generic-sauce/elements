use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct WorldUpdate {
	pub players: Vec<Player>,
	pub teams: Vec<u8>,
	pub tilemap_update: TileMapUpdate,
	pub fluidmap_update: FluidMapUpdate,
	pub frame_id: u32,
	pub wins: [u32; 2],
	pub restart_state: RestartState,
	pub best_of_n: u32,
}

impl World {
	pub fn update(&self) -> WorldUpdate {
		WorldUpdate {
			players: self.players.clone(),
			teams: self.teams.clone(),
			tilemap_update: self.tilemap.update(),
			fluidmap_update: self.fluidmap.update(),
			frame_id: self.frame_id,
			wins: self.wins,
			restart_state: self.restart_state,
			best_of_n: self.best_of_n,
		}
	}

	#[allow(unused)]
	pub fn apply_update(&mut self, u: WorldUpdate, handler: &mut impl EventHandler) {
		self.players = u.players;
		self.teams = u.teams;
		self.tilemap.apply_update(u.tilemap_update, handler);
		self.fluidmap.apply_update(u.fluidmap_update);
		self.frame_id = u.frame_id;
		self.wins = u.wins;
		self.restart_state = u.restart_state;
		self.best_of_n = u.best_of_n;

		// This exists to generate a compiler error whenever a field will be added to World.
		let Self { players: _, teams: _, tilemap: _, fluidmap: _, frame_id: _, wins: _, restart_state: _, best_of_n: _, bird: _ } = self;
	}
}
