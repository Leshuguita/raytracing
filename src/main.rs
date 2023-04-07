mod vector;
use vector::V3;
mod color;

fn main() {
	// Imagen
	let width: u16 = 256;
	let height: u16 = 256;

	// Render

	// Esta en ascii
	println!("P3");
	// filas y columnas
	println!("{} {}", width, height);
	// valor maximo
	println!("255");

	for y in (0..height).rev() {
		for x in 0..width {
			let color = color::Color::new(x as f32 / (width-1) as f32, y as f32 / (height-1) as f32, 0.25);
			println!("{}", color.as_string_255());
		}
	}
}
