use std::io::Cursor;

use image::{ImageBuffer, Rgb, Rgba};

#[derive(Responder)]
#[response(content_type = "image/png")]
pub struct PngImage(pub Vec<u8>);

impl From<ImageBuffer<Rgb<u8>, Vec<u8>>> for PngImage {
    fn from(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).expect("image to bytes failed");
        PngImage(bytes)
    }
}
impl From<ImageBuffer<Rgba<u8>, Vec<u8>>> for PngImage {
    fn from(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).expect("image to bytes failed");
        PngImage(bytes)
    }
}