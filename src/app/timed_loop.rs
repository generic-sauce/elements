use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub struct TimedLoop {
	pub interval: Duration,
	current: SystemTime,
}

impl TimedLoop {
	pub fn new(interval: Duration) -> TimedLoop {
		let now = SystemTime::now();
		TimedLoop {
			interval,
			current: now,
		}
	}

	pub fn with_fps(fps: u32) -> TimedLoop {
		TimedLoop::new(Duration::from_millis((1000/fps) as u64))
	}
}

impl Iterator for TimedLoop {
	type Item = Duration;

	fn next(&mut self) -> Option<Duration> {
		let next = self.current + self.interval;

		let sleep_duration = next.duration_since(SystemTime::now());

		self.current = next;

		Some(match sleep_duration {
			Ok(duration) => {
				sleep(duration);
				self.interval
			},
			Err(err) => err.duration() + self.interval,
		})
	}
}