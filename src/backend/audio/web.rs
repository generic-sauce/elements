use crate::prelude::*;

pub struct WebAudioBackend;

impl AudioBackend for WebAudioBackend {
	fn new() -> Self {
		Self
	}

	fn tick(&mut self) {
		// TODO
	}

	fn queue_music(&mut self, music_id: SoundId) {
		// TODO
	}

	fn play_sound(&mut self, sound_id: SoundId, volume: f32) {
		// TODO
	}
}
