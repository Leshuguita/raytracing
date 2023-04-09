use std::f64::consts::PI;

use crate::{ray::Ray, sphere::Sphere, vector::{Vector3, V3}, hittable::{Hit, Hittable, self}, material::{Lambertian, Metal, Dielectric, Hemisphere}, color::{Color, self}};
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
	pub fn glass_bubble() -> Self {
		Scene {
			hittables: vec![
				Box::new(Sphere::new(
					Vector3::new(-1.0, 0.0, -1.0),
					0.5,
					Dielectric::new_box(Color::white(), 1.5)
				)),
				Box::new(Sphere::new(
					Vector3::new(-1.0, 0.0, -1.0),
					-0.45,
					Dielectric::new_box(Color::white(), 1.5)
				)),
				Box::new(Sphere::new(
					Vector3::new(0.0, 0.0, -1.0),
					0.5, 
					Lambertian::new_box(Color::new(0.1, 0.2, 0.5))
				)),
				Box::new(Sphere::new(
					Vector3::new(1.0, 0.0, -1.0),
					0.5,
					Metal::new_box(Color::new(0.8, 0.6, 0.2), 0.1)
				)),
				Box::new(Sphere::new(
					Vector3::new(0.0, -100.5, -1.0),
					100.0,
					Lambertian::new_box(Color::new(0.8, 0.8, 0.0))
				)),
			]
		}
	}
	pub fn glass_color_balls() -> Self {
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
	pub fn two_balls() -> Self {
		let r = (PI/4.0).cos();
		Scene {
			hittables: vec![
				Box::new(Sphere::new(
					Vector3::new(-r, 0.0, -1.0),
					r,
					Lambertian::new_box(Color::new(0.0, 0.0, 1.0))
				)),
				Box::new(Sphere::new(
					Vector3::new(r, 0.0, -1.0),
					r,
					Lambertian::new_box(Color::new(1.0, 0.0, 0.0))
				)),
			]
		}
	}
	pub fn random() -> Self {
		let mut hittables: Vec<Box<dyn Hittable>> = vec![
			Box::new(Sphere::new(
				Vector3::new(0.0, -1000.0, 0.0),
				1000.0,
				Lambertian::new_box(Color::grey())
			))
		];
		for a in -11..11 {
			for b in -11..11 {
				let choose_material = fastrand::f64();
				let center = Vector3::new(a as f64 + 0.9*fastrand::f64(), 0.2, b as f64 + 0.9*fastrand::f64());
				if (center-Vector3::new(40.0, 0.2, 0.0)).length() > 0.9 {
					hittables.push(
						Box::new(Sphere::new(
							center,
							0.2,
							if choose_material < 0.8 {
								// difuso
								let albedo = Color::random()*Color::random();
								Lambertian::new_box(albedo)
							} else if choose_material < 0.95 {
								// metal
								let albedo = Color::random() * 0.5 + Color::grey();
								let fuzz = fastrand::f64();
								Metal::new_box(albedo, fuzz)
							} else {
								// glass
								let albedo = Color::random() * 0.5 + Color::grey();
								Dielectric::new_box(albedo, fastrand::f64() * 2.0 + 1.5)
							}
						))
					)
				}
			}
		}

		hittables.push(
			Box::new(Sphere::new(
				Vector3::new(0.0, 1.0, 0.0),
				1.0,
				Dielectric::new_box(Color::white(), 1.5)
			))
		);

		hittables.push(
			Box::new(Sphere::new(
				Vector3::new(-4.0, 1.0, 0.0),
				1.0,
				Lambertian::new_box(Color::new(0.4, 0.2, 0.1))
			))
		);

		hittables.push(
			Box::new(Sphere::new(
				Vector3::new(4.0, 1.0, 0.0),
				1.0,
				Metal::new_box(Color::new(0.7, 0.6, 0.5), 0.01)
			))
		);

		Scene { hittables }
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