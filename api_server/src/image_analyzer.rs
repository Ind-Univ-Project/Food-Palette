use crate::error::Error;
use crate::pixel_data::PixelData;

use image::{load_from_memory_with_format, DynamicImage, GenericImageView, ImageFormat, Pixel};
use uuid::Uuid;

use async_std::fs;

use tide::log::info;

pub struct ImageAnalyzer {
    image: DynamicImage,
}

impl ImageAnalyzer {
    pub fn new(buf: &Vec<u8>, format: &str) -> Result<Self, Error> {
        info!("Creating Image Size: {}, Format: {}", buf.len(), format);

        let format = ImageFormat::from_extension(format).ok_or(Error::UnresolvableImageFormat)?;
        let image = load_from_memory_with_format(buf, format).map_err(|e| Error::ImageError(e))?;

        Ok(Self { image })
    }

    pub fn pixel_data(&self) -> PixelData {
        let mut result = PixelData::new(4, 4, 4);

        for (_, _, pixel) in self.image.pixels() {
            result.count_color(pixel.to_rgb());
        }

        result
    }

    pub async fn save_with_format(&self, format: ImageFormat) -> Result<(), Error> {
        fs::create_dir_all("./data/images")
            .await
            .map_err(|e| Error::CreateDirectoryError(e))?;

        let path = format!("./data/images/{}", Uuid::new_v4());

        info!("Creating {}", path);
        self.image.save_with_format(path, format)?;
        info!("File created");

        Ok(())
    }
}
