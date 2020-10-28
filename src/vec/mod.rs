mod generic;
mod custom;

pub use generic::*;
pub use custom::*;

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Debug};

pub trait Primitive:             Default + Display + Debug + PartialEq + Copy + PartialOrd + Mul<Self, Output=Self> + Add<Self, Output=Self> + Div<Self, Output=Self> + Sub<Self, Output=Self> + num_traits::cast::NumCast + PartialEq + FloorIfInt {}
impl<T> Primitive for T where T: Default + Display + Debug + PartialEq + Copy + PartialOrd + Mul<Self, Output=Self> + Add<Self, Output=Self> + Div<Self, Output=Self> + Sub<Self, Output=Self> + num_traits::cast::NumCast + PartialEq + FloorIfInt {}

pub struct Vec2t<T: Primitive, P> {
	pub x: T,
	pub y: T,
	_p: PhantomData<P>,
}

#[allow(unused)]
pub struct DefaultParam;
#[allow(unused)]
pub type Vec2u = Vec2t::<u32, DefaultParam>;
#[allow(unused)]
pub type Vec2f = Vec2t::<f32, DefaultParam>;

pub const fn v<T: Primitive, P>(x: T, y: T) -> Vec2t<T, P> {
	Vec2t::<T, P>::new(x, y)
}
