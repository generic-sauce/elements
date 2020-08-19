use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display, Debug, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::iter::Sum;

use super::Vec2t;
use num_traits::ToPrimitive;

impl<T, P> Vec2t<T, P> {
	pub const fn new(x: T, y: T) -> Vec2t<T, P> {
		Vec2t { x, y, _p: PhantomData }
	}
}

impl<T: Copy, P> From<T> for Vec2t<T, P> {
	fn from(t: T) -> Vec2t<T, P> {
		Vec2t::new(t, t)
	}
}

impl<T, P> From<(T, T)> for Vec2t<T, P> {
	fn from(t: (T, T)) -> Vec2t<T, P> {
		Vec2t::new(t.0, t.1)
	}
}

impl<T: Hash, P> Hash for Vec2t<T, P> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.x.hash(h);
		self.y.hash(h);
		h.finish();
	}
}

impl<T: PartialEq, P> PartialEq for Vec2t<T, P> {
	fn eq(&self, rhs: &Self) -> bool {
		(self.x == rhs.x) && (self.y == rhs.y)
	}
}

impl<T: Eq, P> Eq for Vec2t<T, P> {}

impl<T, P> Vec2t<T, P> {
	pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2t<U, P> {
		Vec2t::new(
			f(self.x),
			f(self.y),
		)
	}
}

impl<T: Clone, P> Clone for Vec2t<T, P> {
	fn clone(&self) -> Self {
		Vec2t::new(
			self.x.clone(),
			self.y.clone(),
		)
	}
}

impl<T: Copy, P> Copy for Vec2t<T, P> { }

impl<T: Display, P> Display for Vec2t<T, P> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl<T: Debug, P> Debug for Vec2t<T, P> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl<T, P> Vec2t<T, P> where T: Add<Output=T> + Mul<Output=T> + Copy {
	pub fn magnitude_sqr(self) -> T {
		self.x * self.x + self.y + self.y
	}
}

// operator overloading

impl<T, P, U: Into<Vec2t<T, P>>> Add<U> for Vec2t<T, P> where T: Add<Output=T> {
	type Output = Vec2t<T, P>;

	fn add(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl<T: Copy, P, U: Into<Vec2t<T, P>>> AddAssign<U> for Vec2t<T, P> where T: Add<Output=T> {
	fn add_assign(&mut self, other: U) {
		*self = *self + other.into();
	}
}

impl<T, P, U: Into<Vec2t<T, P>>> Sub<U> for Vec2t<T, P> where T: Sub<Output=T> {
	type Output = Vec2t<T, P>;

	fn sub(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl<T: Copy, P, U: Into<Vec2t<T, P>>> SubAssign<U> for Vec2t<T, P> where T: Sub<Output=T> {
	fn sub_assign(&mut self, other: U) {
		*self = *self - other.into();
	}
}

impl<T, P, U: Into<Vec2t<T, P>>> Mul<U> for Vec2t<T, P> where T: Mul<Output=T> {
	type Output = Vec2t<T, P>;

	fn mul(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl<T: Copy, P, U: Into<Vec2t<T, P>>> MulAssign<U> for Vec2t<T, P> where T: Mul<Output=T> {
	fn mul_assign(&mut self, other: U) {
		*self = *self * other.into();
	}
}

impl<T, P, U: Into<Vec2t<T, P>>> Div<U> for Vec2t<T, P> where T: Div<Output=T> {
	type Output = Vec2t<T, P>;

	fn div(self, other: U) -> Vec2t<T, P> {
		let other = other.into();
		Vec2t::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl<T: Copy, U: Into<Vec2t<T, P>>, P> DivAssign<U> for Vec2t<T, P> where T: Div<Output=T> {
	fn div_assign(&mut self, other: U) {
		*self = *self / other.into();
	}
}

impl<T: Copy, P> Vec2t<T, P> where T: Add<Output=T> + Mul<Output=T> {
	pub fn dot(self, other: impl Into<Vec2t<T, P>>) -> T {
		let other = other.into();
		self.x * other.x + self.y * other.y
	}
}

impl<T, P> Vec2t<T, P> where T: Copy + Ord + Mul<T, Output=T> + Add<T, Output=T> + Div<T, Output=T> + num_traits::cast::NumCast + Default {
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

	pub fn length_squared(self) -> T where T: Mul<T> + Add<T> {
		self.x * self.x + self.y * self.y
	}

	pub fn length(self) -> T {
		let ls = self.length_squared().to_f64().unwrap();
		T::from::<f64>(ls.sqrt().floor()).unwrap()
	}

	pub fn with_length(self, l: T) -> Vec2t<T, P> {
		let orig_len_sqr = self.x * self.x + self.y * self.y;
		let orig_len = T::from::<f64>((orig_len_sqr.to_f64().unwrap()).sqrt().floor()).unwrap();
		if orig_len == Default::default() { return Vec2t::default(); }
		(self * l) / orig_len
	}
}


impl<T: Default, P> Default for Vec2t<T, P> {
	fn default() -> Self {
		Vec2t::new(
			Default::default(),
			Default::default()
		)
	}
}

// this could be written by only requiring Sum!
impl<T: Add<Output=T> + Default, P> Sum<Self> for Vec2t<T, P> {
	fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
		let null: Vec2t<T, P> = Default::default();
		iter.fold(null, |a, b| Vec2t::new(a.x + b.x, a.y + b.y))
	}
}
