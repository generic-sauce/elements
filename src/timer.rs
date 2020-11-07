#[cfg(feature = "native-client")] mod native {
	use crate::prelude::*;

	pub fn main_loop(mut f: impl FnMut(), fps: u32) {
		TimedLoop::with_fps(fps)
			.for_each(move |_| f());
	}

	pub struct Timer(std::time::Instant);

	impl Timer {
		pub fn new() -> Self {
			Self(std::time::Instant::now())
		}

		pub fn elapsed_ms(&self) -> u64 {
			self.0.elapsed().as_millis() as u64
		}
	}
}
#[cfg(feature = "native-client")] pub use native::*;

#[cfg(feature = "web-client")] mod web {
	use crate::prelude::*;

	pub fn main_loop(f: impl FnMut() + 'static, fps: u32) {
		let cb = Closure::<dyn FnMut()>::wrap(Box::new(f));
		let leaked_cb = Box::leak(Box::new(cb)); // TODO
		setInterval(leaked_cb, 1000 as f64 / fps as f64);
	}

	pub struct Timer { start: u64 }

	fn now() -> u64 {
		web_sys::window().unwrap()
			.performance().unwrap()
			.now() as u64
	}

	impl Timer {
		pub fn new() -> Self {
			Self { start: now() }
		}

		pub fn elapsed_ms(&self) -> u64 {
			now() - self.start
		}
	}

}
#[cfg(feature = "web-client")] pub use web::*;
