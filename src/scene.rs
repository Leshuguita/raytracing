use crate::{ray::Ray, sphere::Sphere, vector::{Vector3, V3}, hittable::{Hit, Hittable}, material::Lambertian, color::Color};
pub struct Scene {
	// El tutorial usa el equivalente a Vec<Arc<T>>, dice que para que puedan
	// compartir texturas y eso. No se si sea necesario, por ahora lo voy a
	// dejar sin, y despues lo cambio si se necesita
	pub hittables: Vec<Box<dyn Hittable>>,
}
impl Scene {
	pub fn default() -> Self {
		Scene {
			hittables: vec![
				Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new_box(&Color::grey()))),
				Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0,  Lambertian::new_box(&Color::grey()))),
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