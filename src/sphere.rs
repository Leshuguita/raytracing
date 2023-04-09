use std::sync::Arc;

use crate::{vector::{V3, Vector3}, hittable::{Hit, Hittable}, ray::Ray, material::Material};

pub struct Sphere {
	pub center: Vector3,
	pub radius: f64,
	pub material: Arc<Box<dyn Material>>,
}
impl Sphere {
	pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Self {
		Sphere { center, radius, material: Arc::new(material) }
	}
}
impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
		let oc = ray.origin - self.center;
    	let a = ray.direction.length_squared();
   		let half_b = oc.dot(ray.direction);
    	let c = oc.length_squared() - self.radius*self.radius;
   		let discriminant = half_b*half_b - a*c;
		if discriminant < 0.0 {
			return None;
		}
		let discriminant_root = discriminant.sqrt();
		let mut root = (-half_b - discriminant_root) / a;
		if root < min_dist || max_dist < root {
			root = (-half_b + discriminant_root) / a;
			if root < min_dist || max_dist < root{
				return None;
			}
		}
		
		let hit_point = ray.at(root);
		let normal = (hit_point - self.center) / self.radius;
		let front_face = ray.direction.dot(normal) < 0.0;
        let normal = if front_face {
			normal
		} else {
			-normal
		};

		Some(
			Hit {
				point: hit_point,
				normal,
				distance: root,
				front_face,
				material: self.material.clone(),
			}
		)
	}
}