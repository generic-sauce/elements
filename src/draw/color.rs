use crate::prelude::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)] // TODO web only
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

#[allow(unused)]
impl Color {
	pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
		Color {
			r, g, b, a: 1.0,
		}
	}

	pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			r, g, b, a,
		}
	}

	pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
	pub const GRAY: Color = Color::rgb(0.2, 0.2, 0.2);
	pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
	pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
	pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
	pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
}

impl Mul for Color {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Color {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
			a: self.a * rhs.a,
		}
	}
}

impl Mul<f32> for Color {
	type Output = Color;

	fn mul(self, rhs: f32) -> Self::Output {
		Color {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
			a: self.a * rhs,
		}
	}
}
