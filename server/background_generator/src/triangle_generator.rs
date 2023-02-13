use image::{ImageBuffer, Rgb, GenericImage};
use imageproc::{definitions::Image, point::Point, drawing};
use rand::{SeedableRng, Rng};
use bracket_color::prelude::*;

#[derive(Copy, Clone)]
pub enum TriangleGeneratorMode {
    Quad,
    Diagonal
}

pub fn generate(width: u32, height: u32, edge_count: u32, color1: Option<String>, color2: Option<String>, seed: Option<u64>, mode: TriangleGeneratorMode) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let seed = match seed {
        Some(s) => s,
        None => rand::thread_rng().gen()
    };
    let mut image = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));
    let edge_count = edge_count;
    let point_count = edge_count + 1;
    
    let mut points = vec![Point::new(0, 0); (point_count*point_count) as usize];
    
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let color1 = match color1 {
        Some(c) => RGB::from_hex(c).unwrap(),
        None => RGB::from_u8(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
    };
    let color2 = match color2 {
        Some(c) => RGB::from_hex(c).unwrap(),
        None => RGB::from_u8(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
    };
    let w = (width / (edge_count - 2)) as i32;
    let h = (height / (edge_count - 2)) as i32;
    for y in 0..=edge_count as usize {
        for x in 0..=edge_count as usize {
            let mut el = &mut points[y * point_count as usize + x];
            el.x = x as i32 * w as i32 + rng.gen_range(-w/2..w/2) - w/2;
            el.y = y as i32 * h as i32 + rng.gen_range(-h/2..h/2) - h/2;
            
        }
    }
    // println!("{points:?}");
    for y in 0..(edge_count as usize) {
        for x in 0..(edge_count as usize) {
            
            // let red = RGB::named(YELLOW);
            // let blue = RGB::named(PURPLE);
            let lerp_coord = x+y;
            let lerp_by = lerp_coord as f32 / (edge_count*2) as f32;
            let color = color1.lerp(color2, lerp_by);
            let color = Rgb([(color.r * 255f32) as u8, (color.g * 255f32) as u8, (color.b * 255f32) as u8]);
            // println!("{}", (255f64 * ((x + y) as f64 / 18f64)) as u8);
            let triangle_points = [points[y * point_count as usize + x], points[y * point_count as usize + x + 1], points[(y + 1) * point_count as usize + x]];
            drawing::draw_polygon_mut(&mut image, &triangle_points, color);
            
            let lerp_coord = match mode {
                TriangleGeneratorMode::Quad => x+y,
                TriangleGeneratorMode::Diagonal => x+y+1
            };
            let lerp_by = lerp_coord as f32 / (edge_count*2) as f32;
            let color = color1.lerp(color2, lerp_by);
            let color = Rgb([(color.r * 255f32) as u8, (color.g * 255f32) as u8, (color.b * 255f32) as u8]);
            let triangle_points = [points[y * point_count as usize + x + 1], points[(y + 1) * point_count as usize + x + 1], points[(y + 1) * point_count as usize + x]];
            drawing::draw_polygon_mut(&mut image, &triangle_points, color);
        }
    }
    image
}