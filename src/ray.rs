use crate::{vector::{Vector3, V3}, color::Color, scene::Scene, hittable::Hittable};

pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3,
}
impl Ray {
	pub fn new(origin: Vector3, direction: Vector3) -> Self {
		Ray { origin, direction }
	}
	pub fn at(&self, t: f32) -> Vector3 {
		self.origin + t*self.direction
	}
	pub fn color(&self, scene: &Scene) -> Color {
		for hittable in &scene.hittables {
			if let Some(hit) = hittable.hit(self, 0.0, f32::INFINITY) {
				return 0.5*Color::new(hit.normal.x()+1.0, hit.normal.y()+1.0,hit.normal.z()+1.0);
			}
		}
		let unit_direction = self.direction.unit();
    	let t = 0.5*(unit_direction.y() + 1.0);
    	(1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
	}
}