use crate::{vector::Vector3, ray::Ray};

pub struct Hit {
	pub point: Vector3,
	pub normal: Vector3,
	pub distance: f32,
	pub front_face: bool,
}
pub trait Hittable {
	fn hit(&self, ray: &Ray, min_dist: f32, max_dist: f32) -> Option<Hit>;
}