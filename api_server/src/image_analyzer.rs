use crate::error::Error;
use crate::pixel_data::PixelData;

use image::{load_from_memory_with_format, DynamicImage, GenericImageView, ImageFormat, Pixel};
use uuid::Uuid;

use std::path::Path;
use tokio::fs;

use tracing::{event, info, instrument, Level};

pub struct ImageAnalyzer {
    image: DynamicImage,
}

impl ImageAnalyzer {
    #[instrument(level = "debug")]
    pub fn new(buf: &[u8], format: &str) -> Result<Self, Error> {
        let format = ImageFormat::from_extension(format).ok_or(Error::UnresolvableImageFormat)?;
        let image = load_from_memory_with_format(buf, format).map_err(|e| Error::ImageError(e))?;

        Ok(Self { image })
    }

    /// ImageAnalyzer 초기화 시 입력한 이미지의 픽셀 색 구성 정보를 얻는다
    #[instrument(level = "debug", skip(self))]
    pub async fn pixel_data(&self) -> PixelData {
        let mut result = PixelData::new(4, 4, 4);

        for (_, _, pixel) in self.image.pixels() {
            result.count_color(pixel.to_rgb()).await;
        }

        result
    }

    /// 주어진 포맷 정보에 맞춰 이미지를 저장한다.
    ///
    /// # Arguments
    /// *`format` - 이미지의 포맷
    #[instrument(skip(self))]
    pub async fn save_with_format(&self, format: ImageFormat) -> Result<String, Error> {
        if Path::new("./data/images").exists() == false {
            event!(Level::INFO, "Create directory /data/images");

            fs::create_dir_all("./data/images")
                .await
                .map_err(|e| Error::CreateDirectoryError(e))?;
        }

        let path = format!("./data/images/{}", Uuid::new_v4());

        self.image.save_with_format(&path, format)?;

        info!("save success [Path: {}]", path);

        Ok(path)
    }
}

#[cfg(test)]
mod test {
    use tokio::test as async_test;

    use super::ImageAnalyzer;

    #[async_test]
    async fn image_analyzer_test() {
        let image_data = include_bytes!("../test/image_analyzer_test1.png");

        let image_analyzer = ImageAnalyzer::new(image_data, "png").unwrap();
        let data = image_analyzer.pixel_data().await;

        let data = data.into_string(3).await;

        assert_eq!("030000000000030303", data);
    }

    #[async_test]
    async fn look_sample() {
        let image_data = include_bytes!("../test/analyzer_sample_4.bmp");

        let image_analyzer = ImageAnalyzer::new(image_data, "bmp").unwrap();
        let data = image_analyzer.pixel_data().await;
        let data = data.into_string(8).await;

        println!("{}", data);
    }
}
