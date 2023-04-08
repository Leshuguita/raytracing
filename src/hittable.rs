use std::sync::Arc;

use crate::{vector::Vector3, ray::Ray, material::Material, color::Color};

pub struct Hit {
	pub point: Vector3,
	pub normal: Vector3,
	pub distance: f64,
	pub front_face: bool,
	pub material: Arc<Box<dyn Material>>,
}
pub trait Hittable {
	fn hit(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<Hit>;
}