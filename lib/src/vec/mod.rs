mod generic;
mod custom;

pub use generic::*;
pub use custom::*;

use std::marker::PhantomData;

pub struct Vec2t<T, P> {
	pub x: T,
	pub y: T,
	_p: PhantomData<P>,
}
