use std::collections::HashMap;

pub type ApiResult<T> = Result<T, ApiError>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("error extracting zip file: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("failed to parse JSON: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type TranslationsMap = HashMap<String, HashMap<String, String>>;
