use std::ops::{Add, Sub, Mul, Div, Neg};

pub trait V3: Sized + Add + Sub + Neg + Mul<f32, Output = Self> + Div<f32, Output = Self> + Copy {
	fn new(x: f32, y: f32, z: f32) -> Self;
	fn x(&self) -> f32;
	fn y(&self) -> f32;
	fn z(&self) -> f32;
	fn length(&self) -> f32 {
		self.length_squared().sqrt()
	}
	fn length_squared(&self) -> f32 {
		self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
	}
	fn dot<T: V3>(&self, other: T) -> f32 {
		self.x() * other.x() +
		self.y() * other.y() +
		self.z() * other.z()
	}
	fn cross<T: V3>(&self, other: T) -> Self {
		Self::new(
			self.y()*other.z() - self.z()*other.y(),
			self.z()*other.x() - self.x()*other.z(),
			self.x()*other.y() - self.y()*other.x(),
		)
	}
	fn unit(&self) -> Self {
		*self/self.length()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	x: f32,
	y: f32,
	z: f32,
}
impl Neg for Vector3 {
	type Output = Vector3;
	fn neg(self) -> Self::Output {
		Vector3 {
			x: - self.x,
			y: - self.y,
			z: - self.z,
		}
	}
}
impl Add for Vector3 {
	type Output = Vector3;
	fn add(self, rhs: Self) -> Self::Output {
		Vector3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}
impl Sub for Vector3 {
	type Output = Vector3;
	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}
impl Mul<Vector3> for f32 {
	type Output = Vector3;
	fn mul(self, rhs: Vector3) -> Self::Output {
		Vector3 {
			x: self * rhs.x,
			y: self * rhs.y,
			z: self * rhs.z,
		}
	}
}
impl Mul<f32> for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: f32) -> Self::Output {
		rhs * self
	}
}
impl Div<f32> for Vector3 {
	type Output = Vector3;
	fn div(self, rhs: f32) -> Self::Output {
		Vector3 {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}
impl V3 for Vector3 {
	fn new(x: f32, y: f32, z: f32) -> Self {
		Vector3 { x, y, z }
	}
	fn x(&self) -> f32 {
		self.x
	}
	fn y(&self) -> f32 {
		self.y
	}
	fn z(&self) -> f32 {
		self.z
	}
}