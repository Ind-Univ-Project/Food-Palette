use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategorizedImage {
    pub category: String,
    pub image_type: String,
    pub image_buffer: String,
}

