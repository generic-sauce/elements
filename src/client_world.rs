use crate::prelude::*;

pub struct ClientWorld<B: Backend> {
	pub world: World,
	phantom: PhantomData<B>,
}

impl<B: Backend> ClientWorld<B> {
	pub fn new(best_of_n: u32) -> ClientWorld<B> {
		let world = World::new_defaultmap(best_of_n);
		ClientWorld {
			world,
			phantom: PhantomData,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		let mut handler = AppEventHandler::new();
		self.world.tick(&mut handler);
		self.handle(&handler, &mut app.sound_manager);
		self.update_music(&mut app.sound_manager);
	}

	fn handle(&mut self, handler: &AppEventHandler, sound_manager: &mut SoundManager) {
		if let Some(dmg) = (0..2).map(|p| handler.damages[p]).max() {
			if dmg > 0 {
				let volume = (dmg as f32 / 100.0).max(0.5).min(2.0);
				sound_manager.play_sound(SoundId::Whiz, volume);
			}
		}
	}

	pub fn apply_update(&mut self, update: WorldUpdate, sound_manager: &mut SoundManager) {
		let mut handler = AppEventHandler::new();
		self.world.apply_update(update, &mut handler);
		self.handle(&handler, sound_manager);
	}

	pub fn update_music(&mut self, sound_manager: &mut SoundManager) {
		let mut critical_level = 0;
		for player in &self.world.players {
			if player.health < MAX_HEALTH / 2 {
				critical_level += 1;
			}
		}
		let sound_id = [SoundId::APart, SoundId::BPart, SoundId::DPart][critical_level];
		if sound_manager.current_music_id.map_or(true, |music_id| music_id != sound_id) {
			sound_manager.play_music(sound_id);
		}
	}
}
