use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, DivAssign, MulAssign};
use super::vector::V3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
	r: f64,
	g: f64,
	b: f64,
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
impl AddAssign for Color {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}
impl Sub for Color {
	type Output = Color;
	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}
impl Mul<Color> for f64 {
	type Output = Color;
	fn mul(self, rhs: Color) -> Self::Output {
		Color {
			r: self * rhs.r,
			g: self * rhs.g,
			b: self * rhs.b,
		}
	}
}
impl Mul<f64> for Color {
	type Output = Color;
	fn mul(self, rhs: f64) -> Self::Output {
		rhs * self
	}
}
impl MulAssign<f64> for Color {
	fn mul_assign(&mut self, rhs: f64) {
		*self = rhs * *self
	}
}
impl Div<f64> for Color {
	type Output = Color;
	fn div(self, rhs: f64) -> Self::Output {
		Color {
			r: self.r / rhs,
			g: self.g / rhs,
			b: self.b / rhs,
		}
	}
}
impl DivAssign<f64> for Color {
	fn div_assign(&mut self, rhs: f64) {
		*self = Color {
			r: self.r / rhs,
			g: self.g / rhs,
			b: self.b / rhs,
		}
	}
}
impl V3 for Color {
	fn new(r: f64, g: f64, b: f64) -> Self {
		Color { r, g, b }
	}
	fn x(&self) -> f64 {
		self.r
	}
	fn y(&self) -> f64 {
		self.g
	}
	fn z(&self) -> f64 {
		self.b
	}
}
impl Color {
	pub fn gamma_2(&self) -> Self {
		Color {
			r: self.r.sqrt(),
			g: self.g.sqrt(),
			b: self.b.sqrt(),
		}
	}
	pub fn as_string_255(&self) -> String {
		format!("{} {} {}", (255.0*self.r) as u8, (255.0*self.g) as u8, (255.0*self.b) as u8) 
	}
	pub fn black() -> Self {
		Color{r:0.0, g:0.0, b:0.0}
	}
}