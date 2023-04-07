use crate::{sphere::Sphere, vector::{Vector3, V3}};
pub struct Scene {
	pub spheres: Vec<Sphere>,
}
impl Scene {
	pub fn default() -> Self {
		Scene {
			spheres: vec![
				Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
			]
		}
	}
}