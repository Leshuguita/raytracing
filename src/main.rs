mod scene;
use scene::Scene;
mod vector;
use vector::{V3, Vector3};
mod sphere;
mod color;
mod ray;
use ray::Ray;

fn main() {
	// Imagen
	let aspect_ratio: f32 = 16.0/9.0;
	let image_width: u16 = 400;
	let image_height = (image_width as f32 /aspect_ratio) as u16;

	// Camara

	let viewport_height = 2.0;
	let viewport_width = aspect_ratio*viewport_height;
	let focal_length = 1.0;

	let origin = Vector3::zero();
	let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
	let vertical = Vector3::new(0.0, viewport_height, 0.0);

	let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0, 0.0, focal_length);
	
	// Render

	// Esta en ascii
	println!("P3");
	// filas y columnas
	println!("{} {}", image_width, image_height);
	// valor maximo
	println!("255");

	let scene = Scene::default();

	for y in (0..image_height).rev() {
		for x in 0..image_width {
			let u = x as f32/ (image_width-1) as f32;
            let v = y as f32/ (image_height-1) as f32;
			let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);

			let color = r.color(&scene);
			println!("{}", color.as_string_255());
		}
	}
}
