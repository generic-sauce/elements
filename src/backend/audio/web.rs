use crate::prelude::*;

pub struct WebAudioBackend {
	last_music_id: Option<SoundId>,
}

#[derive(Serialize, Deserialize)]
pub enum WebAudioCommand {
	QueueMusic(String),
	PlaySound(String, f32),
}

impl AudioBackend for WebAudioBackend {
	fn new() -> Self {
		Self {
			last_music_id: None,
		}
	}

	fn tick(&mut self) {} // ticking happens in javascript!

	fn queue_music(&mut self, music_id: SoundId) {
		if Some(music_id) == self.last_music_id { return; }
		self.last_music_id = Some(music_id);

		let cmd = WebAudioCommand::QueueMusic(music_id.filename().to_string());
		handle_audio_command(JsValue::from_serde(&cmd).unwrap());
	}

	fn play_sound(&mut self, sound_id: SoundId, volume: f32) {
		let cmd = WebAudioCommand::PlaySound(sound_id.filename().to_string(), volume);
		handle_audio_command(JsValue::from_serde(&cmd).unwrap());
	}
}
