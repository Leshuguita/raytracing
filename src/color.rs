use std::ops::{Add, Sub, Mul, Div, Neg};
use super::vector::V3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
}

impl Neg for Color {
	type Output = Color;
	fn neg(self) -> Self::Output {
		Color {
			r: - self.r,
			g: - self.g,
			b: - self.b,
		}
	}
}
impl Add for Color {
	type Output = Color;
	fn add(self, rhs: Self) -> Self::Output {
		Color {
			r: self.r + rhs.r,
			g: self.g + rhs.g,
			b: self.b + rhs.b,
		}
	}
}
impl Sub for Color {
	type Output = Color;
	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}
impl Mul<Color> for f32 {
	type Output = Color;
	fn mul(self, rhs: Color) -> Self::Output {
		Color {
			r: self * rhs.r,
			g: self * rhs.g,
			b: self * rhs.b,
		}
	}
}
impl Mul<f32> for Color {
	type Output = Color;
	fn mul(self, rhs: f32) -> Self::Output {
		rhs * self
	}
}
impl Div<f32> for Color {
	type Output = Color;
	fn div(self, rhs: f32) -> Self::Output {
		Color {
			r: self.r / rhs,
			g: self.g / rhs,
			b: self.b / rhs,
		}
	}
}
impl V3 for Color {
	fn new(x: f32, y: f32, z: f32) -> Self {
		Color { r:x, g:y, b:z }
	}
	fn x(&self) -> f32 {
		self.r
	}
	fn y(&self) -> f32 {
		self.g
	}
	fn z(&self) -> f32 {
		self.b
	}
}
impl Color {
	pub fn as_string_255(&self) -> String {
		format!("{} {} {}", (255.0*self.r) as u8, (255.0*self.g) as u8, (255.0*self.b) as u8) 
	}
}