use image::{ImageBuffer, Rgb, GenericImage};
use imageproc::{definitions::Image, point::Point, drawing, pixelops::interpolate};
use rand::{SeedableRng, Rng};
// use bracket_color::prelude::*;

#[derive(Copy, Clone)]
pub enum TriangleGeneratorMode {
	Quad,
	Diagonal
}

pub fn generate(width: u32, height: u32, edge_count: u32, color1: [u8; 3], color2: [u8; 3], seed: u64, mode: TriangleGeneratorMode) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
	let mut image = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));
	let edge_count = edge_count;
	let point_count = edge_count + 1;
	
	let mut points = vec![Point::new(0, 0); (point_count*point_count) as usize];
	
	let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

	let color1 = Rgb(color1);
	let color2 = Rgb(color2);
	let w = (width / (edge_count - 2)) as i32;
	for y in 0..=edge_count as usize {
		for x in 0..=edge_count as usize {
			let mut el = &mut points[y * point_count as usize + x];
			el.x = x as i32 * w as i32 + rng.gen_range(-w/2..w/2) - w/2;
			el.y = y as i32 * w as i32 + rng.gen_range(-w/2..w/2) - w/2;
			
		}
	}
	for y in 0..(edge_count as usize) {
		for x in 0..(edge_count as usize) {
			let lerp_coord = x+y;
			let lerp_by = lerp_coord as f32 / (edge_count*2) as f32;
			let color = interpolate(color1, color2, lerp_by);
			let triangle_points = [points[y * point_count as usize + x], points[y * point_count as usize + x + 1], points[(y + 1) * point_count as usize + x]];
			drawing::draw_polygon_mut(&mut image, &triangle_points, color);
			
			let lerp_coord = match mode {
				TriangleGeneratorMode::Quad => x+y,
				TriangleGeneratorMode::Diagonal => x+y+1
			};
			let lerp_by = lerp_coord as f32 / (edge_count*2) as f32;
			let color = interpolate(color1, color2, lerp_by);
			let triangle_points = [points[y * point_count as usize + x + 1], points[(y + 1) * point_count as usize + x + 1], points[(y + 1) * point_count as usize + x]];
			drawing::draw_polygon_mut(&mut image, &triangle_points, color);
		}
	}
	image
}