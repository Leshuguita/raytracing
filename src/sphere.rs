use crate::vector::Vector3;

pub struct Sphere {
	pub center: Vector3,
	pub radius: f32,
}
impl Sphere {
	pub fn new(center: Vector3, radius: f32) -> Self {
		Sphere { center, radius }
	}
}