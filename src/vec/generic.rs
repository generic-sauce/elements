use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display, Debug, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::iter::Sum;

use crate::prelude::*;

impl<T: Primitive, P> Vec2t<T, P> {
	pub const fn new(x: T, y: T) -> Vec2t<T, P> {
		Vec2t { x, y, _p: PhantomData }
	}
}

#[allow(unused)]
impl<T1: Primitive, P1> Vec2t<T1, P1> {
	// use with care!
	pub fn cast_with<T2: Primitive, P2>(self, f: impl Fn(T1) -> T2) -> Vec2t<T2, P2> {
		Vec2t {
			x: f(self.x),
			y: f(self.y),
			_p: PhantomData,
		}
	}

	// use with care!
	pub const fn cast<P2>(self) -> Vec2t<T1, P2> {
		Vec2t {
			x: self.x,
			y: self.y,
			_p: PhantomData,
		}
	}
}


impl<T: Primitive, P> From<T> for Vec2t<T, P> {
	fn from(t: T) -> Vec2t<T, P> {
		Vec2t::new(t, t)
	}
}

impl<T: Primitive, P> Default for Vec2t<T, P> {
	fn default() -> Self {
		Vec2t::new(
			Default::default(),
			Default::default()
		)
	}
}

impl<T: Primitive, P> From<(T, T)> for Vec2t<T, P> {
	fn from(t: (T, T)) -> Vec2t<T, P> {
		Vec2t::new(t.0, t.1)
	}
}

impl<T: Primitive + Hash, P> Hash for Vec2t<T, P> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.x.hash(h);
		self.y.hash(h);
		h.finish();
	}
}

impl<T: Primitive, P> PartialEq for Vec2t<T, P> {
	fn eq(&self, rhs: &Self) -> bool {
		(self.x == rhs.x) && (self.y == rhs.y)
	}
}

impl<T: Primitive + Eq, P> Eq for Vec2t<T, P> {}

impl<T: Primitive, P> Vec2t<T, P> {
	pub fn map<U: Primitive, F: Fn(T) -> U>(self, f: F) -> Vec2t<U, P> {
		Vec2t::new(
			f(self.x),
			f(self.y),
		)
	}
}

impl<T: Primitive, P> Clone for Vec2t<T, P> {
	fn clone(&self) -> Self {
		Vec2t::new(
			self.x,
			self.y,
		)
	}
}

impl<T: Primitive, P> Copy for Vec2t<T, P> { }

impl<T: Primitive, P> Display for Vec2t<T, P> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl<T: Primitive, P> Debug for Vec2t<T, P> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

// operator overloading

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> Add<U> for Vec2t<T, P> {
	type Output = Vec2t<T, P>;

	fn add(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> AddAssign<U> for Vec2t<T, P> {
	fn add_assign(&mut self, other: U) {
		*self = *self + other.into();
	}
}

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> Sub<U> for Vec2t<T, P> {
	type Output = Vec2t<T, P>;

	fn sub(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> SubAssign<U> for Vec2t<T, P> {
	fn sub_assign(&mut self, other: U) {
		*self = *self - other.into();
	}
}

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> Mul<U> for Vec2t<T, P> {
	type Output = Vec2t<T, P>;

	fn mul(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> MulAssign<U> for Vec2t<T, P> {
	fn mul_assign(&mut self, other: U) {
		*self = *self * other.into();
	}
}

impl<T: Primitive, P, U: Into<Vec2t<T, P>>> Div<U> for Vec2t<T, P> {
	type Output = Vec2t<T, P>;

	fn div(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl<T: Primitive, U: Into<Vec2t<T, P>>, P> DivAssign<U> for Vec2t<T, P> {
	fn div_assign(&mut self, other: U) {
		*self = *self / other.into();
	}
}

// this could be written by only requiring Sum!
impl<T: Primitive, P> Sum<Self> for Vec2t<T, P> {
	fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
		let null: Vec2t<T, P> = Default::default();
		iter.fold(null, |a, b| a + b)
	}
}

// other functionality

impl<T: Primitive, P> Vec2t<T, P> {
	pub fn dot(self, other: impl Into<Vec2t<T, P>>) -> T {
		let other = other.into();
		self.x * other.x + self.y * other.y
	}
}

pub trait FloorIfInt {
	fn floor_if_int(arg: f64) -> f64;
}

impl FloorIfInt for i32 {
	fn floor_if_int(arg: f64) -> f64 {
		arg.floor()
	}
}

impl FloorIfInt for u32 {
	fn floor_if_int(arg: f64) -> f64 {
		arg.floor()
	}
}

impl FloorIfInt for f32 {
	fn floor_if_int(arg: f64) -> f64 {
		arg
	}
}

impl FloorIfInt for f64 {
	fn floor_if_int(arg: f64) -> f64 {
		arg
	}
}

impl<T: Primitive, P> Vec2t<T, P> {
	#[allow(unused)]
	pub fn clamped(self, min: T, max: T) -> Vec2t<T, P> {
		Vec2t::new(
			if self.x < min { min } else if self.x > max { max } else { self.x },
			if self.y < min { min } else if self.y > max { max } else { self.y },
		)
	}

	pub fn length_clamped(self, max: T) -> Vec2t<T, P> {
		if self.length_squared() > max*max {
			self.with_length(max)
		} else {
			self
		}
	}

	pub fn length_squared(self) -> T {
		self.x * self.x + self.y * self.y
	}

	pub fn length(self) -> T {
		let ls = self.length_squared().to_f64().unwrap();
		T::from::<f64>(T::floor_if_int(ls.sqrt())).unwrap()
	}

	pub fn with_length(self, l: T) -> Vec2t<T, P> {
		let orig_len_sqr = self.x * self.x + self.y * self.y;
		let orig_len = T::from::<f64>(T::floor_if_int((orig_len_sqr.to_f64().unwrap()).sqrt())).unwrap();
		if orig_len == Default::default() { return Vec2t::default(); }
		(self * l) / orig_len
	}

	pub fn as_short_as(self, l: T) -> bool { self.length_squared() <= l * l }

	pub fn projected_on(self, other: Self) -> Self {
		if other == Default::default() {
			return other;
		}
		other * self.dot(other) / other.dot(other)
	}

	pub fn mix(self, other: Vec2t<T, P>, self_weight: T, other_weight: T) -> Vec2t<T, P> {
		self * self_weight + other * other_weight / (self_weight + other_weight)
	}
}
