use crate::{vector::{Vector3, V3}, color::Color, scene::Scene, hittable::Hittable};

pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3,
}
impl Ray {
	pub fn new(origin: Vector3, direction: Vector3) -> Self {
		Ray { origin, direction }
	}
	pub fn at(&self, t: f64) -> Vector3 {
		self.origin + t*self.direction
	}
	pub fn color(&self, scene: &Scene, depth: u16) -> Color {
		if depth==0 {
			return Color::black();
		}
		if let Some(hit) = scene.hit(self, 0.001, f64::INFINITY) {
			if let Some((scattered_ray, atennuation)) = hit.material.scatter(self, &hit) {
				return atennuation*scattered_ray.color(scene, depth-1);
			}
			return Color::black();
		}
		// Fondo
		let unit_direction = self.direction.unit();
    	let t = 0.5*(unit_direction.y() + 1.0);
    	(1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
	}
}