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
}
impl Material for Metal {
	fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
		let reflected = ray_in.direction.unit().reflect(&hit.normal);
		let scattered = Ray::new(hit.point, reflected);
		if scattered.direction.dot(hit.normal) > 0.0 {
			Some((scattered, self.albedo))
		} else {
			None
		}
	}
}
impl Metal {
	pub fn new_box(color: &Color) -> Box<Self> {
		Box::new(Metal { albedo: *color })
	}
}