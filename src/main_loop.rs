use crate::prelude::*;

#[cfg(feature = "native-client")]
pub fn main_loop(mut f: impl FnMut(), fps: u32) {
	TimedLoop::with_fps(fps)
		.for_each(move |_| f());
}

#[cfg(feature = "web-client")]
pub fn main_loop(mut f: impl FnMut() + 'static, fps: u32) {
	let cb = Closure::<dyn FnMut()>::wrap(Box::new(f));
	let leaked_cb = Box::leak(Box::new(cb)); // TODO
	setInterval(leaked_cb, 1000 as f64 / fps as f64);
}

