mod scene;
use scene::Scene;
mod vector;
use vector::{V3, Vector3};
mod hittable;
mod sphere;
mod color;
mod ray;
use ray::Ray;
mod utils;
mod camera;
use camera::Camera;

use crate::color::Color;

fn main() {
	// Imagen
	let aspect_ratio: f32 = 16.0/9.0;
	let image_width: u16 = 400;
	let image_height = (image_width as f32 /aspect_ratio) as u16;
	let samples_per_pixel: u8 = 50;

	// Render
	let camera = Camera::default();
	// Esta en ascii
	println!("P3");
	// filas y columnas
	println!("{} {}", image_width, image_height);
	// valor maximo
	println!("255");

	let scene = Scene::default();

	for y in (0..image_height).rev() {
		eprintln!("{}/{} filas", image_height-y, image_height);
		for x in 0..image_width {
			let mut color = Color::black();
			for _ in 0..samples_per_pixel {
				let u = (x as f32 + fastrand::f32())/ (image_width-1) as f32;
            	let v = (y as f32 + fastrand::f32())/ (image_height-1) as f32;
                let ray = camera.get_ray(u, v);
                color += ray.color(&scene);
            }
			color /= samples_per_pixel as f32;
			println!("{}", color.as_string_255());
		}
	}
}
