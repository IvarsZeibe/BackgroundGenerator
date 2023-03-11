use image::{ImageBuffer, Rgb};
use imageproc::{drawing, pixelops::interpolate};
use noise::{Simplex, NoiseFn, Perlin};
use rand::{SeedableRng, Rng};

pub fn generate(width: u32, height: u32, chain_count: u32, circle_radius: u32, spacing: f32, background_color: [u8; 3], color1: [u8; 3], color2: [u8; 3], seed: u64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
	let circle_radius = circle_radius as i32;
	let step = circle_radius as f32 * 2. + spacing;
	
	let mut image = ImageBuffer::from_pixel(width as u32, height as u32, Rgb(background_color));
	
	let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
	let noise = Perlin::new(rng.gen());
	
	let mut points = vec![];

	for _ in 0..chain_count {
		let point = (rng.gen_range(0..width) as f32, rng.gen_range(0..height) as f32);
		let n = (noise.get([point.0 as f64 / 3., point.1 as f64 / 3.])).abs() as f32;
		let color = interpolate(Rgb(color1), Rgb(color2), n);

		if has_point_near(&point, &points, step as f32) {
			continue;
		}

		let mut previous_point = point;
		let mut new_points = vec![];
		let is_long = rng.gen_bool(0.2);
		while (rng.gen_bool(0.95) || is_long) && new_points.len() < 200 {
			let n: f32 = (noise.get([previous_point.0 as f64 / 400., previous_point.1 as f64 / 400.])) as f32;
			let angle_in_radians = n * 1.;
			let cos = angle_in_radians.cos();
			let sin = angle_in_radians.sin();
			let new_point = (previous_point.0 + step * sin, previous_point.1 + step * cos);
			if new_point.0 > width as f32 || new_point.0 < 0. || new_point.1 > height as f32 || new_point.1 < 0. {
				break;
			}
			if has_point_near(&new_point, &points, step) {
				break;
			}
			new_points.push(new_point);
			previous_point = new_point;
		}
		if new_points.len() < 3 {
			continue;
		}
		for p in new_points.iter() {
			drawing::draw_filled_circle_mut(&mut image, (p.0.round() as i32, p.1.round() as i32), circle_radius, color);
		}
		points.append(&mut new_points);

		
	}

	image
}
fn has_point_near(point: &(f32, f32), points: &Vec<(f32, f32)>, distance: f32) -> bool {
	for p in points {
		if (point.0 - p.0).powi(2) + (point.1 - p.1).powi(2) < distance.powi(2) {
			return true;
		}
	}
	false
}