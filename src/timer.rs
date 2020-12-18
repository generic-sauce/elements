#[cfg(feature = "native-client")] mod native {
	use crate::prelude::*;

	// TODO: this is code duplication, also defined in backend
	pub fn now() -> f64 {
		std::time::UNIX_EPOCH.elapsed().unwrap().as_micros() as f64 / 1000.
	}

	#[derive(Copy, Clone)]
	pub struct Timer {
		start_time: f64,
	}

	impl Timer {
		pub fn new() -> Timer {
			Timer {
				start_time: now(),
			}
		}

		pub fn elapsed_ms(self) -> f32 {
			(now() - self.start_time) as f32
		}
	}
}
#[cfg(feature = "native-client")] pub use native::*;

#[cfg(feature = "web-client")] mod web {
	use crate::prelude::*;

	// TODO: this is code duplication, also defined in backend
	pub fn now() -> f64 {
		date_now()
	}
}
#[cfg(feature = "web-client")] pub use web::*;
