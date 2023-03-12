use image::{ImageBuffer, Rgb};
use imageproc::{drawing, pixelops::interpolate};
use rand::{Rng, SeedableRng};
use std::f32::consts::SQRT_2;

pub fn generate(
    width: u32,
    height: u32,
    circle_count: u32,
    max_size: u32,
    color1: [u8; 3],
    color2: [u8; 3],
    background_color: [u8; 3],
    seed: u64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let color1 = Rgb(color1);
    let color2 = Rgb(color2);
    let background_color = Rgb(background_color);

    let mut image = ImageBuffer::from_pixel(width, height, background_color);

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let gap = max_size as f32 * SQRT_2;
    let y_max = (height as f32 / gap).ceil() as i32 + 1;
    let x_max = (width as f32 / gap).ceil() as i32 + 1;
    for y in 0..y_max {
        for x in 0..x_max {
            let center: (i32, i32) = ((gap * x as f32) as i32, (gap * y as f32) as i32);

            let lerp_coord = center.0 + center.1;
            let lerp_by = lerp_coord as f32 / (width + height) as f32;
            let color = interpolate(color1, color2, lerp_by);

            drawing::draw_filled_circle_mut(&mut image, center, max_size as i32, color.into());
        }
    }

    for _ in 0..circle_count {
        let center = (
            rng.gen_range(0..width) as i32,
            rng.gen_range(0..height) as i32,
        );
        let radius = rng.gen_range(1..max_size) as i32;

        let lerp_coord = center.0 + center.1;
        let lerp_by = lerp_coord as f32 / (width + height) as f32;
        let color = interpolate(color1, color2, lerp_by);

        drawing::draw_filled_circle_mut(&mut image, center, radius, color.into());
    }

    image
}
