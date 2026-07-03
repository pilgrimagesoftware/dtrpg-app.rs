pub const APPLICATION_KEY_ENV: &str = "DTRPG_APPLICATION_KEY";
pub const ACCESS_TOKEN_ENV: &str = "DTRPG_ACCESS_TOKEN";
pub const REFRESH_TOKEN_ENV: &str = "DTRPG_REFRESH_TOKEN";
pub const REFRESH_TOKEN_TTL_ENV: &str = "DTRPG_REFRESH_TOKEN_TTL";
pub const API_BASE_URL_ENV: &str = "DTRPG_API_BASE_URL";

pub const DEFAULT_COLOR: &str = "#2E3A45";
pub const BYTES_PER_MB: f64 = 1_048_576.0;

/// Base URL that relative cover image paths (`OrderProductInfo::image`,
/// `thumbnail`, etc.) are appended to in order to form a fetchable thumbnail
/// URL.
pub const DTRPG_IMAGES_BASE_URL: &str = "https://api.drivethrurpg.com/images/";
