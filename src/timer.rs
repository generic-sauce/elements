#[cfg(feature = "native-client")] mod native {
	use crate::prelude::*;

	pub fn main_loop(mut f: impl FnMut(), fps: u32) {
		TimedLoop::with_fps(fps)
			.for_each(move |_| f());
	}

	pub fn now() -> f64 {
		std::time::UNIX_EPOCH.elapsed().unwrap().as_micros() as f64 / 1000.
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

	pub fn now() -> f64 {
		date_now()
	}
}
#[cfg(feature = "web-client")] pub use web::*;
