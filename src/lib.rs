#![feature(drain_filter)]
#![feature(const_fn)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]

#[cfg(feature = "web-client")]
include!("base.rs");
