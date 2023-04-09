use crate::{hittable::Hit, color::Color, ray::Ray, vector::{Vector3, V3}};

pub trait Material {
	fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
	albedo: Color,
}
impl Material for Lambertian {
	fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
		let mut target = hit.normal + Vector3::random_unit();
		if target.near_zero() {
			target = hit.normal;
		}
		let scattered = Ray::new(hit.point, target);
		Some((scattered, self.albedo))
	}
}
impl Lambertian {
	pub fn new_box(color: &Color) -> Box<Self> {
		Box::new(Lambertian { albedo: *color })
	}
}

pub struct Hemisphere {
	albedo: Color,
}
impl Material for Hemisphere {
	fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
		let mut target = hit.normal + Vector3::random_in_hemisphere(&hit.normal);
		if target.near_zero() {
			target = hit.normal;
		}
		let scattered = Ray::new(hit.point, target);
		Some((scattered, self.albedo))
	}
}
impl Hemisphere {
	pub fn new_box(color: &Color) -> Box<Self> {
		Box::new(Hemisphere { albedo: *color })
	}
}

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}
impl Material for Metal {
	fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
		let reflected = ray_in.direction.unit().reflect(&hit.normal);
		let scattered = Ray::new(hit.point, reflected + self.fuzz*Vector3::random_unit());
		if scattered.direction.dot(hit.normal) > 0.0 {
			Some((scattered, self.albedo))
		} else {
			None
		}
	}
}
impl Metal {
	pub fn new_box(color: &Color, fuzz: f64) -> Box<Self> {
		Box::new(Metal { albedo: *color, fuzz})
	}
}

pub struct Dielectric {
	refraction_index: f64,
}
impl Material for Dielectric {
	fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
		let atennuation = Color::white();
		let refraction_ratio = if hit.front_face {
			1.0/self.refraction_index
		} else {
			self.refraction_index
		};
		let unit_direction = ray_in.direction.unit();
		let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
		let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

		let can_refract = refraction_ratio*sin_theta <= 1.0;
		
		let direction = if can_refract {
			unit_direction.refract(&hit.normal, refraction_ratio)
		} else {
			unit_direction.reflect(&hit.normal)
		};

		let scattered = Ray::new(hit.point, direction);
		Some((scattered, atennuation))
	}
}
impl Dielectric {
	pub fn new_box(refraction_index: f64) -> Box<Self> {
		Box::new(Dielectric { refraction_index})
	}
}