use image::{DynamicImage, ImageResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[image] Can not resolve image format str")]
    UnresolvableImageFormat,
    #[error("[image] Fail to load image with format")]
    ImageError(#[from] image::ImageError),
    #[error("[async-std] Fail to create directory all")]
    CreateDirectoryError(tokio::io::Error),
    #[error("[sqlx] DB error")]
    DBError(#[from] sqlx::Error),
}
