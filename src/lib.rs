#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

#[wasm_bindgen]
pub fn start_game() {
	alert("starting!");
	run(Local::new())
}
