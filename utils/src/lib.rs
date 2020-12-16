use std::time::{Duration, Instant};
use std::thread::sleep;

#[derive(Clone)]
pub struct TimedLoop {
	pub interval: Duration,
	current: Instant,
	start_time: Instant,
	prev_second: Instant,
	frames_since_prev_second: u32,
	duration_since_prev_second: Duration,
	fps: u32,
	load: f32,
}

impl TimedLoop {
	pub fn new(interval: Duration) -> TimedLoop {
		let now = Instant::now();
		TimedLoop {
			interval,
			current: now,
			start_time: now,
			prev_second: now,
			frames_since_prev_second: 0,
			duration_since_prev_second: Duration::from_secs(0),
			fps: 0,
			load: 0.0,
		}
	}

	pub fn with_fps(fps: u32) -> TimedLoop {
		TimedLoop::new(Duration::from_micros(1000000 / fps as u64))
	}

	pub fn elapsed_time(&self) -> Duration {
		Instant::now().duration_since(self.start_time)
	}
}

pub struct TimedLoopInfo {
	pub elapsed_time: Duration,
	pub delta_time: Duration,
	pub fps: u32,
	pub load: f32,
	pub interval: Duration,
}

impl Iterator for TimedLoop {
	type Item = TimedLoopInfo;

	fn next(&mut self) -> Option<Self::Item> {
		let now = Instant::now();
		let next = self.current + self.interval;
		let delta_time = match next.checked_duration_since(now) {
			Some(duration) => {
				self.current = next;
				sleep(duration);
				self.interval - duration
			},
			None => {
				self.current = now;
				self.interval + now.duration_since(next)
			},
		};

		let next_second = self.prev_second + Duration::from_secs(1);
		if now.checked_duration_since(next_second).is_some() {
			self.fps = self.frames_since_prev_second;
			self.frames_since_prev_second = 0;

			self.load = self.duration_since_prev_second.as_secs_f32();
			self.duration_since_prev_second = Duration::from_secs(0);

			self.prev_second = next_second;
		}

		self.duration_since_prev_second += delta_time;
		self.frames_since_prev_second += 1;
		Some(
			TimedLoopInfo {
				elapsed_time: self.elapsed_time(),
				delta_time,
				fps: self.fps,
				load: self.load,
				interval: self.interval,
			}
		)
	}
}
