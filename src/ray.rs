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
	pub fn color(&self, scene: &Scene, depth: u16) -> Color {
		if depth==0 {
			return Color::black();
		}
		if let Some(hit) = scene.hit(self, 0.0, f32::INFINITY) {
			//let target = hit.point + hit.normal + Vector3::random_unit();
			//let new_ray = Ray::new(hit.point, target-hit.point);
			//return 0.5*new_ray.color(scene, depth-1);
			return 0.5*Color::new(hit.normal.x()+1.0, hit.normal.y()+1.0,hit.normal.z()+1.0);
		}
		let unit_direction = self.direction.unit();
    	let t = 0.5*(unit_direction.y() + 1.0);
    	(1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
	}
}