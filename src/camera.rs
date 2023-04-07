use crate::{vector::{V3, Vector3}, ray::Ray};

pub struct Camera {
	origin: Vector3,
	lower_left_corner: Vector3,
	horizontal: Vector3,
	vertical: Vector3,
}
impl Camera {
	pub fn default() -> Self {
		Camera::new(16.0/9.0, 2.0)
	}
 	pub fn new(ratio: f32, viewport_height: f32) -> Self {
		let viewport_width = ratio*viewport_height;
		let focal_length = 1.0;
	
		let origin = Vector3::zero();
		let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
		let vertical = Vector3::new(0.0, viewport_height, 0.0);
	
		let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0, 0.0, focal_length);

		Camera { origin, lower_left_corner, horizontal, vertical }	
	}
	pub fn get_ray(&self, u: f32, v: f32) -> Ray {
		Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
	}
}