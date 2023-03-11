use std::{io::Cursor, ops::Deref};

use image::{EncodableLayout, ImageBuffer, PixelWithColorType};

#[derive(Responder)]
#[response(content_type = "image/png")]
pub struct PngImage(pub Vec<u8>);

impl<P, Container> From<ImageBuffer<P, Container>> for PngImage
where
    P: PixelWithColorType,
    [P::Subpixel]: EncodableLayout,
    Container: Deref<Target = [P::Subpixel]>,
{
    fn from(image: ImageBuffer<P, Container>) -> Self {
        let mut bytes: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .expect("image to bytes failed");
        PngImage(bytes)
    }
}
