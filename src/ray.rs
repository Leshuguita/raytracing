use crate::{vector::{Vector3, V3}, color::Color};

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
	pub fn color(&self) -> Color {
		let unit_direction = self.direction.unit();
    	let t = 0.5*(unit_direction.y() + 1.0);
    	(1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
	}
}