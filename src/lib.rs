#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

use std::panic;

#[wasm_bindgen]
pub fn start_game() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

	alert("starting!");
	run(Local::new())
}
