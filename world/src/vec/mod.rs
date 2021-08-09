mod generic;
pub use generic::*;

mod custom;
pub use custom::*;

mod draw;
pub use draw::*;

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug};

pub trait Primitive:             Default + Display + Debug + PartialEq + Copy + PartialOrd + Mul<Self, Output=Self> + Add<Self, Output=Self> + Div<Self, Output=Self> + Sub<Self, Output=Self> + num_traits::cast::NumCast + PartialEq + FloorIfInt + Trunc {}
impl<T> Primitive for T where T: Default + Display + Debug + PartialEq + Copy + PartialOrd + Mul<Self, Output=Self> + Add<Self, Output=Self> + Div<Self, Output=Self> + Sub<Self, Output=Self> + num_traits::cast::NumCast + PartialEq + FloorIfInt + Trunc {}

pub struct Vec2t<T: Primitive, P> {
	pub x: T,
	pub y: T,
	_p: PhantomData<P>,
}

pub const fn v<T: Primitive, P>(x: T, y: T) -> Vec2t<T, P> {
	Vec2t::<T, P>::new(x, y)
}
