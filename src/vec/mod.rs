use sfml::system::Vector2;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display, Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

pub struct Vec2t<T> {
	pub x: T,
	pub y: T,
}

#[allow(dead_code)]
pub type Vec2f = Vec2t<f32>;
#[allow(dead_code)]
pub type Vec2i = Vec2t<i32>;
#[allow(dead_code)]
pub type Vec2u = Vec2t<u32>;
#[allow(dead_code)]
pub type Vec2u8 = Vec2t<u8>;

impl<T> From<Vec2t<T>> for Vector2<T> {
	fn from(t: Vec2t<T>) -> Vector2<T> {
		Vector2::new(t.x, t.y)
	}
}

impl<T> From<Vector2<T>> for Vec2t<T> {
	fn from(t: Vector2<T>) -> Vec2t<T> {
		Vec2t::new(t.x, t.y)
	}
}

#[allow(dead_code)]
impl Vec2f {
	pub fn to_i(self) -> Vec2i { self.map(|i| i as i32) }
	pub fn to_u(self) -> Vec2u { self.map(|x| x as u32) }
	pub fn to_u8(self) -> Vec2u8 { self.map(|x| x as u8) }
}

#[allow(dead_code)]
impl Vec2i {
	pub fn to_f(self) -> Vec2f { self.map(|x| x as f32) }
	pub fn to_u(self) -> Vec2u { self.map(|x| x as u32) }
	pub fn to_u8(self) -> Vec2u8 { self.map(|x| x as u8) }
}

#[allow(dead_code)]
impl Vec2u {
	pub fn to_f(self) -> Vec2f { self.map(|x| x as f32) }
	pub fn to_i(self) -> Vec2i { self.map(|x| x as i32) }
	pub fn to_u8(self) -> Vec2u8 { self.map(|x| x as u8) }
}

#[allow(dead_code)]
impl Vec2u8 {
	pub fn to_f(self) -> Vec2f { self.map(|x| x as f32) }
	pub fn to_i(self) -> Vec2i { self.map(|x| x as i32) }
	pub fn to_u(self) -> Vec2u { self.map(|x| x as u32) }
}

impl<T> Vec2t<T> {
	pub const fn new(x: T, y: T) -> Vec2t<T> {
		Vec2t { x, y }
	}
}

impl<T: Copy> From<T> for Vec2t<T> {
	fn from(t: T) -> Vec2t<T> {
		Vec2t::new(t, t)
	}
}

impl<T> From<(T, T)> for Vec2t<T> {
	fn from(t: (T, T)) -> Vec2t<T> {
		Vec2t::new(t.0, t.1)
	}
}

impl<T: Hash> Hash for Vec2t<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.x.hash(h);
		self.y.hash(h);
		h.finish();
	}
}

impl<T: PartialEq> PartialEq for Vec2t<T> {
	fn eq(&self, rhs: &Self) -> bool {
		(self.x == rhs.x) && (self.y == rhs.y)
	}
}

impl<T: Eq> Eq for Vec2t<T> {}

impl<T> Vec2t<T> {
	pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2t<U> {
		Vec2t::new(
			f(self.x),
			f(self.y),
		)
	}
}

impl<T: Clone> Clone for Vec2t<T> {
	fn clone(&self) -> Self {
		Vec2t::new(
			self.x.clone(),
			self.y.clone(),
		)
	}
}

impl<T: Copy> Copy for Vec2t<T> { }

impl<T: Display> Display for Vec2t<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl<T: Debug> Debug for Vec2t<T> {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2t({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl Vec2f {
	pub fn magnitude(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

impl<T> Vec2t<T> where T: Add<Output=T> + Mul<Output=T> + Copy {
	pub fn magnitude_sqr(self) -> T {
		self.x * self.x + self.y + self.y
	}
}

// operator overloading

impl<T, U: Into<Vec2t<T>>> Add<U> for Vec2t<T> where T: Add<Output=T> {
	type Output = Vec2t<T>;

	fn add(self, other: U) -> Vec2t<T> {
		let other = other.into();
		Vec2t::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl<T: Copy, U: Into<Vec2t<T>>> AddAssign<U> for Vec2t<T> where T: Add<Output=T> {
	fn add_assign(&mut self, other: U) {
		*self = *self + other.into();
	}
}

impl<T, U: Into<Vec2t<T>>> Sub<U> for Vec2t<T> where T: Sub<Output=T> {
	type Output = Vec2t<T>;

	fn sub(self, other: U) -> Vec2t<T> {
		let other = other.into();
		Vec2t::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl<T: Copy, U: Into<Vec2t<T>>> SubAssign<U> for Vec2t<T> where T: Sub<Output=T> {
	fn sub_assign(&mut self, other: U) {
		*self = *self - other.into();
	}
}

impl<T, U: Into<Vec2t<T>>> Mul<U> for Vec2t<T> where T: Mul<Output=T> {
	type Output = Vec2t<T>;

	fn mul(self, other: U) -> Vec2t<T> {
		let other = other.into();
		Vec2t::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl<T: Copy, U: Into<Vec2t<T>>> MulAssign<U> for Vec2t<T> where T: Mul<Output=T> {
	fn mul_assign(&mut self, other: U) {
		*self = *self * other.into();
	}
}

impl<T, U: Into<Vec2t<T>>> Div<U> for Vec2t<T> where T: Div<Output=T> {
	type Output = Vec2t<T>;

	fn div(self, other: U) -> Vec2t<T> {
		let other = other.into();
		Vec2t::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl<T: Copy, U: Into<Vec2t<T>>> DivAssign<U> for Vec2t<T> where T: Div<Output=T> {
	fn div_assign(&mut self, other: U) {
		*self = *self / other.into();
	}
}

impl<T: Copy> Vec2t<T> where T: Add<Output=T> + Mul<Output=T> {
	pub fn dot(self, other: impl Into<Vec2t<T>>) -> T {
		let other = other.into();
		self.x * other.x + self.y * other.y
	}
}
