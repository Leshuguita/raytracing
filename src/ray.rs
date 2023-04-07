use crate::{vector::{Vector3, V3}, color::Color, sphere::Sphere, scene::Scene};

pub struct Ray {
	origin: Vector3,
	direction: Vector3,
}
impl Ray {
	pub fn new(origin: Vector3, direction: Vector3) -> Self {
		Ray { origin, direction }
	}
	fn at(&self, t: f32) -> Vector3 {
		self.origin + t*self.direction
	}
	pub fn color(&self, scene: &Scene) -> Color {
		for sphere in &scene.spheres {
			if self.hit_sphere(sphere) {
				return Color::new(1.0, 0.0, 0.0);
			}
		}
		let unit_direction = self.direction.unit();
    	let t = 0.5*(unit_direction.y() + 1.0);
    	(1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
	}
	pub fn hit_sphere(&self, sphere: &Sphere) -> bool {
		let oc = self.origin - sphere.center;
    	let a = self.direction.dot(self.direction);
   		let b = 2.0 * oc.dot(self.direction);
    	let c = oc.dot(oc) - sphere.radius*sphere.radius;
   		let discriminant = b*b - 4.0*a*c;
    	discriminant > 0.0
	}
}