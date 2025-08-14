#![cfg(feature = "camera")]
use anyhow::Result;
use nokhwa::{pixel_format::RgbFormat, Camera};

pub fn snapshot(output: &str) -> Result<()> {
	let mut cam = Camera::new(0, None)?;
	cam.open_stream()?;
	let frame = cam.frame()?;
	let decoded = frame.decode_image::<RgbFormat>()?;
	let (w, h) = (decoded.width(), decoded.height());
	let buffer = decoded.buffer();
	image::save_buffer(output, buffer, w, h, image::ExtendedColorType::Rgb8)?;
	Ok(())
} 