use crate::prelude::*;

#[derive(Clone)]
pub struct TimedLoop {
	pub interval: Duration,
	current: SystemTime,
	start_time: SystemTime,
	prev_second: SystemTime,
	frames_since_prev_second: u32,
	duration_since_prev_second: Duration,
	fps: u32,
	load: f32,
}

impl TimedLoop {
	pub fn new(interval: Duration) -> TimedLoop {
		let now = SystemTime::now();
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
		(SystemTime::now().duration_since(self.start_time)).unwrap()
	}
}

impl Iterator for TimedLoop {
	// TODO: use struct instead of tuple
	type Item = (Duration, Duration, u32, f32);

	fn next(&mut self) -> Option<Self::Item> {
		let now = SystemTime::now();
		let next = self.current + self.interval;
		let sleep_duration = next.duration_since(now);
		let delta_time = match sleep_duration {
			Ok(duration) => {
				self.current = next;
				sleep(duration);
				self.interval - duration
			},
			Err(err) => {
				self.current = now;
				self.interval + err.duration()
			},
		};

		let next_second = self.prev_second + Duration::from_secs(1);
		match now.duration_since(next_second) {
			Ok(_) => {
				self.fps = self.frames_since_prev_second;
				self.frames_since_prev_second = 0;

				self.load = self.duration_since_prev_second.as_secs_f32();
				self.duration_since_prev_second = Duration::from_secs(0);

				self.prev_second = next_second;
			},
			_ => {},
		}

		self.duration_since_prev_second += delta_time;
		self.frames_since_prev_second += 1;
		Some((self.elapsed_time(), delta_time, self.fps, self.load))
	}
}
