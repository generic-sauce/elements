use super::*;

use sfml::system::*;

#[allow(dead_code)]
pub type Vec2f = Vec2t<f32>;
#[allow(dead_code)]
pub type Vec2i = Vec2t<i32>;
#[allow(dead_code)]
pub type Vec2u = Vec2t<u32>;
#[allow(dead_code)]
pub type Vec2u8 = Vec2t<u8>;


impl<T, P> From<Vec2t<T, P>> for Vector2<T> {
	fn from(t: Vec2t<T, P>) -> Vector2<T> {
		Vector2::new(t.x, t.y)
	}
}

impl<T, P> From<Vector2<T>> for Vec2t<T, P> {
	fn from(t: Vector2<T>) -> Vec2t<T, P> {
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

impl Vec2f {
	pub fn magnitude(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}
