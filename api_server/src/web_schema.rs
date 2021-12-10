use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CategorizedImage {
    pub category: String,
    pub image_type: String,
    pub image_buffer: String,
}

#[derive(Deserialize, Debug)]
pub struct ImageSelectionFilter {
    pub colors: Vec<u32>,
    pub foods: Vec<String>,
}
