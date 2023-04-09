use crate::{vector::{V3, Vector3}, ray::Ray};

pub struct Camera {
	origin: Vector3,
	lower_left_corner: Vector3,
	horizontal: Vector3,
	vertical: Vector3,
}
impl Camera {
	pub fn default() -> Self {
		Camera::new(
			Vector3::new(0.0,0.0,0.0),
			Vector3::new(0.0, 0.0, 1.0),
			Vector3::new(0.0, 1.0, 0.0),
			16.0/9.0,
			90.0
		)
	}
 	pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vertical_fov_deg: f64, ratio: f64) -> Self {
		let theta = vertical_fov_deg.to_radians();
		let h = (theta/2.0).tan();
		let viewport_height = 2.0*h;
		let viewport_width = ratio*viewport_height;
		
		let w = (lookfrom - lookat).unit();
		let u = vup.cross(w).unit();
		let v = w.cross(u);

		let horizontal = viewport_width * u;
		let vertical = viewport_height * v;
		Camera {
			origin: lookfrom,
			lower_left_corner: lookfrom-horizontal/2.0-vertical/2.0-w,
			horizontal,
			vertical
		}	
	}
	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
	}
}