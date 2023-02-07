use std::io::{Bytes, Cursor};

use image::{ImageBuffer, Rgb};

pub mod colorful_image_generator;
pub mod triangle_generator;
pub mod circles_generator;
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// pub trait Generator<T> {
//     fn generate() -> T;
// }

// pub struct TriangleGenerator;
// impl Generator<Vec<u8>> for TriangleGenerator {
//     fn generate() -> Vec<u8> {
//         let image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(500, 500, Rgb([255, 0, 0]));
//         let mut bytes: Vec<u8> = Vec::new();
//         image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).expect("image to bytes failed");
//         bytes
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
