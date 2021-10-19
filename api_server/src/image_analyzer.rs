use crate::error::Error;
use crate::pixel_data::PixelData;

use image::{load_from_memory_with_format, DynamicImage, ImageFormat};

use tide::log::info;

pub struct ImageAnalyzer {
    image: DynamicImage,
}

impl ImageAnalyzer {
    pub fn new(buf: &Vec<u8>, format: &str) -> Result<Self, Error> {
        info!("Creating Image Size: {}, Format: {}", buf.len(), format);

        let format = ImageFormat::from_extension(format).ok_or(Error::UnresolvableImageFormat)?;
        let image = load_from_memory_with_format(buf, format)
            .map_err(|e| Error::ImageLoadWithFormatError(e))?;

        Ok(Self { image })
    }

    pub fn pixel_data(&self) -> PixelData {
        todo!()
    }

    pub fn save_with_format(&self, format: ImageFormat) {
        todo!()
    }
}
