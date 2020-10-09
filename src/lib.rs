#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

use std::panic;

#[wasm_bindgen]
pub fn init() -> *mut Local {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
	Box::leak(Box::new(Local::new())) as *mut Local
}
