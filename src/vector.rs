use std::{
	f64::consts::PI,
	ops::{Add, Sub, Mul, Div, Neg, AddAssign, DivAssign, MulAssign},
};

pub trait V3: Sized + Add + AddAssign + Sub + Neg + Mul + Mul<f64, Output = Self> + MulAssign<f64> + Div<f64, Output = Self> + DivAssign<f64> + Copy {
	fn new(x: f64, y: f64, z: f64) -> Self;
	fn x(&self) -> f64;
	fn y(&self) -> f64;
	fn z(&self) -> f64;
	fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}
	fn length_squared(&self) -> f64 {
		self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
	}
	fn dot<T: V3>(&self, other: T) -> f64 {
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
	fn near_zero(&self) -> bool {
		self.x() < f64::EPSILON && self.y() < f64::EPSILON && self.z() < f64::EPSILON
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	x: f64,
	y: f64,
	z: f64,
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
impl AddAssign for Vector3 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}
impl Sub for Vector3 {
	type Output = Vector3;
	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}
impl Mul<Vector3> for f64 {
	type Output = Vector3;
	fn mul(self, rhs: Vector3) -> Self::Output {
		Vector3 {
			x: self * rhs.x,
			y: self * rhs.y,
			z: self * rhs.z,
		}
	}
}
impl Mul for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: Vector3) -> Self::Output {
		Vector3 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
		}
	}
}
impl Mul<f64> for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: f64) -> Self::Output {
		rhs * self
	}
}
impl MulAssign<f64> for Vector3 {
	fn mul_assign(&mut self, rhs: f64) {
		*self = rhs * *self
	}
}
impl Div<f64> for Vector3 {
	type Output = Vector3;
	fn div(self, rhs: f64) -> Self::Output {
		Vector3 {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}
impl DivAssign<f64> for Vector3 {
	fn div_assign(&mut self, rhs: f64) {
		*self = Vector3 {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}
impl V3 for Vector3 {
	fn new(x: f64, y: f64, z: f64) -> Self {
		Vector3 { x, y, z }
	}
	fn x(&self) -> f64 {
		self.x
	}
	fn y(&self) -> f64 {
		self.y
	}
	fn z(&self) -> f64 {
		self.z
	}
}
impl Vector3 {
	pub fn zero() -> Self {
		Vector3::new(0.0,0.0,0.0)
	}
	pub fn random_unit() -> Self {
		// Algoritmo sacado de https://mathworld.wolfram.com/SpherePointPicking.html
		let u = fastrand::f64();
		let v = fastrand::f64();
		let phi = (2.0*v-1.0).acos();
		let theta = 2.0*PI*u;

		let x = theta.cos()*phi.sin();
		let y = theta.sin()*phi.cos();
		let z = phi.cos();
		Vector3 { x, y, z }
	}
	pub fn random() -> Self {
		Vector3 { x: fastrand::f64()*2.0-1.0, y: fastrand::f64()*2.0-1.0, z: fastrand::f64()*2.0-1.0 }
	}
	pub fn random_unit_discard() -> Self {
		loop {
			let v = Vector3::random();
			if v.length_squared() >= 1.0 {
				continue
			}
			return v;
		}
	}
	pub fn random_in_hemisphere(normal: &Vector3) -> Self {
		let unit = Vector3::random_unit();
		if unit.dot(*normal) > 0.0 {
			unit
		} else {
			-unit
		}
	}
	pub fn reflect(&self, normal: &Vector3) -> Vector3 {
		*self - 2.0*self.dot(*normal)*(*normal)
	}
}