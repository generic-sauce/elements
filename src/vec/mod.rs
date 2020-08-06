mod generic;
mod iuf8;
mod pos;

pub use generic::*;
pub use iuf8::*;
pub use pos::*;

use std::marker::PhantomData;

pub struct DefaultParam;

pub struct Vec2t<T, P = DefaultParam> {
	pub x: T,
	pub y: T,
	_p: PhantomData<P>,
}
