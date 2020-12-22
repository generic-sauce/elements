use crate::prelude::*;

use rodio::*;
use once_cell::sync::OnceCell;

const START_MUSIC_OFFSET: Duration = Duration::from_micros(1000);
const NUM_PARTS: usize = 4;

static SOUNDS: OnceCell<Vec<Sound>> = OnceCell::new();

struct Sound {
	channels: u16,
	sample_rate: u32,
	data: Vec<f32>,
}

pub struct NativeAudioBackend {
	device: Device,
	music_sink: Sink,
	pub current_music_id: Option<SoundId>,
	pub next_music_id: Option<SoundId>,
	pub next_part: u8,
	next_music_refresh_time: Instant,
}

impl AudioBackend for NativeAudioBackend {
	fn new() -> Self {
		let device = get_device();
		let music_sink = Sink::new(&device);

		thread::spawn(|| {
			let samples = load_samples();
			SOUNDS.set(samples).unwrap_or_else(|_| panic!("SOUNDS is already defined?!"));
		});

		Self {
			device,
			music_sink,
			current_music_id: None,
			next_music_id: None,
			next_part: 0,
			next_music_refresh_time: Instant::now(),
		}
	}

	fn tick(&mut self) {
		self.check_start_music();
		self.check_restart_music();
	}

	fn queue_music(&mut self, music_id: SoundId) {
		if self.current_music_id.map_or(true, |x| x != music_id) {
			self.next_music_id = Some(music_id);
		}
	}

	fn play_sound(&mut self, sound_id: SoundId, volume: f32) {
		if let Some(buf) = get_sample_buffer(sound_id) {
			play_raw(&self.device, buf.amplify(volume));
		}
	}

	fn set_music_volume(&mut self, volume: f32) {
		self.music_sink.set_volume(volume);
	}
}

fn load_samples() -> Vec<Sound> {
	SoundId::iter().map(|sound_id| {
		let file = File::open(res(sound_id.filename())).unwrap();
		let source = Decoder::new(BufReader::new(file)).unwrap();
		let channels = source.channels();
		let sample_rate = source.sample_rate();
		let data = source.convert_samples().collect();
		Sound {
			channels,
			sample_rate,
			data,
		}
	}).collect()
}

fn get_part_sample_buffer(sound_id: SoundId, part: u8) -> Option<static_buffer::StaticSamplesBuffer<f32>> {
	let part = part as usize;
	let sample = get_sound(sound_id)?;
	let part_size = sample.data.len() / NUM_PARTS;

	Some(static_buffer::StaticSamplesBuffer::new(
		sample.channels,
		sample.sample_rate,
		&sample.data[part*part_size..(part+1)*part_size]
	))
}

fn get_sound(sound_id: SoundId) -> Option<&'static Sound> {
	SOUNDS.get().map(|v| &v[sound_id as usize])
}

fn get_sample_buffer(sound_id: SoundId) -> Option<static_buffer::StaticSamplesBuffer<f32>> {
	let sample = get_sound(sound_id)?;
	Some(static_buffer::StaticSamplesBuffer::new(
		sample.channels,
		sample.sample_rate,
		&sample.data[..],
	))
}

impl NativeAudioBackend {
	fn start_music_sample(&mut self, music_id: SoundId) {
		if let Some(sample) = get_part_sample_buffer(music_id, self.next_part) {
			let sample_duration = sample.total_duration().unwrap();
			self.music_sink.append(sample);
			self.current_music_id = Some(music_id);
			self.next_music_refresh_time += sample_duration;
			self.next_part = (self.next_part + 1) % NUM_PARTS as u8;
		}
	}

	fn check_start_music(&mut self) {
		if self.current_music_id.is_none() {
			if let Some(next_music_id) = self.next_music_id {
				self.next_music_refresh_time = Instant::now();
				self.start_music_sample(next_music_id);
			}
		}
	}

	fn check_restart_music(&mut self) {
		let should_refresh = self.next_music_refresh_time.saturating_duration_since(Instant::now()) < START_MUSIC_OFFSET;
		if should_refresh {
			if let Some(next_music_id) = self.next_music_id {
				// assert_eq!(self.music_sink.len(), 1);
				self.start_music_sample(next_music_id);
			}
		}
	}
}

// this is a fallback for the case that default_output_device().default_output_format() is Err
fn get_device() -> Device {
	default_output_device().into_iter()
		.chain(devices().into_iter().flatten())
		.find(|d| d.default_output_format().is_ok())
		.expect("no output device found!")
}
