use image::{DynamicImage, ImageResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("[image] Can not resolve image format str")]
    UnresolvableImageFormat,
    #[error("[image] Fail to load image with format")]
    ImageLoadWithFormatError(#[from] image::ImageError),
    #[error("[tide] Fail to get peer address")]
    PeerAddressError,
}
