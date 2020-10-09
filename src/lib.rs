#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

pub fn start_game() {
	run(Local::new())
}
