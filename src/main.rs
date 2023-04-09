mod scene;
use scene::Scene;
mod vector;
mod hittable;
mod sphere;
mod color;
mod ray;
mod utils;
mod camera;
use camera::Camera;
mod material;

use rayon::prelude::*;

use crate::{color::Color, vector::{Vector3, V3}};

fn main() {
	// Imagen
	let aspect_ratio: f64 = 16.0/9.0;
	let image_width: u16 = 1200;
	let image_height = (image_width as f64 /aspect_ratio) as u16;
	
	let samples_per_pixel: u16 = 500;
	let max_ray_iterations: u16 = 50;
	// Render
	let position = Vector3::new(13.0, 2.0, 3.0);
	let target = Vector3::new(0.0, 0.0, 0.0);
	let camera = Camera::new(
		position,
		target,
		Vector3::new(0.0, 1.0, 0.0),
		20.0,
		aspect_ratio,
		0.01,
		10.0,
	);
	// Esta en ascii
	println!("P3");
	// filas y columnas
	println!("{} {}", image_width, image_height);
	// valor maximo
	println!("255");

	let scene = Scene::random();

	for y in (0..image_height).rev() {
		eprintln!("{}/{} filas", image_height-y, image_height);
		let pixels: Vec<Color> = (0..image_width).into_par_iter().map(|x| {
			let mut color = Color::black();
			for _ in 0..samples_per_pixel {
				let u = (x as f64 + fastrand::f64())/ (image_width-1) as f64;
            	let v = (y as f64 + fastrand::f64())/ (image_height-1) as f64;
                let ray = camera.get_ray(u, v);
                color += ray.color(&scene, max_ray_iterations);
            }
			color *= 1.0/samples_per_pixel as f64;
			color.gamma_2()
		}).collect();
		for pixel in pixels {
			println!("{}", pixel.as_string_255());
		}
	}
}
