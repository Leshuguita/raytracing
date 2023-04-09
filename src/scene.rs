use crate::{ray::Ray, sphere::Sphere, vector::{Vector3, V3}, hittable::{Hit, Hittable}, material::{Lambertian, Metal, Dielectric}, color::Color};
pub struct Scene {
	// El tutorial usa el equivalente a Vec<Arc<T>>, dice que para que puedan
	// compartir texturas y eso. No se si sea necesario, por ahora lo voy a
	// dejar sin, y despues lo cambio si se necesita
	pub hittables: Vec<Box<dyn Hittable>>,
}
impl Scene {
	pub fn gray_balls() -> Self {
		Scene {
			hittables: vec![
				Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new_box(Color::grey()))),
				Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0,  Lambertian::new_box(Color::grey()))),
			]
		}
	}
	pub fn metal_balls() -> Self {
		Scene {
			hittables: vec![
				Box::new(Sphere::new(
					Vector3::new(-1.0, 0.1, -1.0),
					0.6,
					Metal::new_box(Color::new(0.8, 0.8, 0.8), 0.1)
				)),
				Box::new(Sphere::new(
					Vector3::new(0.0, -0.2, -1.0),
					0.4, 
					Lambertian::new_box(Color::new(0.7, 0.3, 0.3))
				)),
				Box::new(Sphere::new(
					Vector3::new(1.0, 0.0, -1.0),
					0.5,
					Metal::new_box(Color::new(0.8, 0.6, 0.2), 0.7)
				)),
				Box::new(Sphere::new(
					Vector3::new(0.0, -100.5, -1.0),
					100.0,
					Lambertian::new_box(Color::rgb(34, 130, 25))
				)),
			]
		}
	}
	pub fn glass_balls() -> Self {
		Scene {
			hittables: vec![
				Box::new(Sphere::new(
					Vector3::new(-1.0, 0.1, -1.0),
					0.6,
					Dielectric::new_box(Color::rgb(255, 55, 48), 1.5)
				)),
				Box::new(Sphere::new(
					Vector3::new(0.0, -0.2, -1.0),
					0.4, 
					Dielectric::new_box(Color::rgb(255, 209, 84), 10.0)
				)),
				Box::new(Sphere::new(
					Vector3::new(1.0, 0.0, -1.0),
					0.5,
					Metal::new_box(Color::new(0.8, 0.6, 0.2), 0.7)
				)),
				Box::new(Sphere::new(
					Vector3::new(0.0, -100.5, -1.0),
					100.0,
					Lambertian::new_box(Color::rgb(34, 130, 25))
				)),
			]
		}
	}
}
impl Hittable for Scene {
	fn hit(&self, ray: &Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
		let mut hit: Option<Hit> = None;
    	for object in &self.hittables {
			let old_distance = hit.as_ref().map(|h| h.distance).unwrap_or(max_dist);
			if let Some(new_hit) = object.hit(ray, min_dist, old_distance) {
				if new_hit.distance < old_distance {
					hit = Some(new_hit);
				}
			};
		}
		hit
	}
}