use std::time::{Duration, SystemTime};
use std::thread::sleep;

#[derive(Clone)]
pub struct TimedLoop {
	pub interval: Duration,
	current: SystemTime,
}

impl TimedLoop {
	pub fn new(interval: Duration) -> TimedLoop {
		TimedLoop {
			interval,
			current: SystemTime::now(),
		}
	}

	pub fn with_fps(fps: u32) -> TimedLoop {
		TimedLoop::new(Duration::from_millis((1000/fps) as u64))
	}
}

impl Iterator for TimedLoop {
	type Item = Duration;

	fn next(&mut self) -> Option<Duration> {
		let now = SystemTime::now();
		let next = self.current + self.interval;

		let sleep_duration = next.duration_since(now);

		Some(match sleep_duration {
			Ok(duration) => {
				self.current = next;
				sleep(duration);
				self.interval - duration
			},
			Err(err) => {
				self.current = now;
				err.duration() + self.interval
			},
		})
	}
}
