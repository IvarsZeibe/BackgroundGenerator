use bracket_color::{rgb::RGB, rgba::RGBA};
use image::{ImageBuffer, Rgb, Pixel, Rgba};
use imageproc::drawing;
use rand::{SeedableRng, Rng};

pub fn generate(width: u32, height: u32, circle_count: u32, max_size: u32, alpha: u8, seed: u64) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    
    let mut image = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
    
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let color1 = MyRgb(RGBA::from_u8(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255), 255));
    let color2 = MyRgb(RGBA::from_u8(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255), 255));

    for _ in 0..circle_count {
        let center = (rng.gen_range(0..width) as i32, rng.gen_range(0..height) as i32);
        let radius = rng.gen_range(1..max_size) as i32;

        
        let lerp_coord = center.0 + center.1;
        let lerp_by = lerp_coord as f32 / (width + height) as f32;
        let mut color = color1.lerp(&color2, lerp_by);
        // color.0.a = alpha as f32 / 255.0;

        drawing::draw_filled_circle_mut(&mut image, center, radius, color.into());
    }

    image
}

struct MyRgb(RGBA);

impl MyRgb {
    // fn new(r: u8, g: u8, b: u8) -> Self {
    //     MyRgb(r, g, b)
    // }
    fn lerp(&self, color: &MyRgb, percent: f32) -> MyRgb {
        Self(self.0.lerp(color.0, percent))
    }
}

// impl From<RGB> for MyRgb {
//     fn from(color: RGB) -> Self {
//         // MyRgb(Rgb([(color.r * 255f32) as u8, (color.g * 255f32) as u8, (color.b * 255f32) as u8]))
//     }
// }
// impl Into<RGB> for MyRgb {
//     fn into(self) -> RGB {
//         RGB::from_u8(self.0, self.1, self.2)
//     }
// }
impl Into<Rgba<u8>> for MyRgb {
    fn into(self) -> Rgba<u8> {
        Rgba([(self.0.r * 255.0) as u8, (self.0.g * 255.0) as u8, (self.0.b * 255.0) as u8, (self.0.a * 255.0) as u8])
    }
}
impl Into<Rgba<f32>> for MyRgb {
    fn into(self) -> Rgba<f32> {
        Rgba([self.0.r, self.0.g, self.0.b, self.0.a])
    }
}