use anyhow::Result;

pub const BYTES_PER_MB: f64 = 1_048_576.0; // (1024^2)
pub const HUGGING_FACE_BASE_URL: &str = "https://huggingface.co";

pub fn format_model_size(size: &str) -> Result<String> {
    let size_mb = size.parse::<f64>()? / BYTES_PER_MB;

    if size_mb >= 1024.0 {
        Ok(format!("{:.2} GB", s