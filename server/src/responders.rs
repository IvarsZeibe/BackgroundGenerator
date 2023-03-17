use image::DynamicImage;
use std::io::Cursor;

#[derive(Responder)]
#[response(content_type = "image/png")]
pub struct PngImage(pub Vec<u8>);

impl From<DynamicImage> for PngImage {
	fn from(image: DynamicImage) -> Self {
		let mut bytes: Vec<u8> = Vec::new();
		image
			.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
			.expect("image to bytes failed");
		PngImage(bytes)
	}
}
