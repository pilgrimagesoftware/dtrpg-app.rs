//! Gravatar URL computation and avatar image fetching.

use std::path::PathBuf;
use std::time::Duration;

const APP_NAME: &str = "dtrpg";
const CACHE_FILE: &str = "avatar";

fn avatar_cache_path() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join(APP_NAME).join(CACHE_FILE))
}

fn load_cached_avatar() -> Option<Vec<u8>> {
    let path = avatar_cache_path()?;
    let bytes = std::fs::read(path).ok()?;
    if bytes.is_empty() { None } else { Some(bytes) }
}

fn save_cached_avatar(bytes: &[u8]) {
    let Some(path) = avatar_cache_path() else {
        tracing::warn!("avatar cache: cache_dir unavailable");
        return;
    };
    if let Some(parent) = path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        tracing::warn!("avatar cache: failed to create dir: {e}");
        return;
    }
    if let Err(e) = std::fs::write(&path, bytes) {
        tracing::warn!("avatar cache: failed to write {}: {e}", path.display());
    }
}

/// Computes a Gravatar avatar URL for the given email address.
///
/// Trims, lowercases, and MD5-hashes the email per the Gravatar specification.
/// `d=404` returns HTTP 404 for unregistered emails, allowing callers to detect
/// missing avatars rather than receiving a generic fallback image.
pub fn gravatar_url(email: &str) -> String {
    let hash = format!("{:x}", md5::compute(email.trim().to_lowercase()));
    format!("https://www.gravatar.com/avatar/{hash}?d=404&s=64")
}

/// Fetches raw avatar image bytes for the given email.
///
/// Checks the disk cache first; returns cached bytes if available. On a cache
/// miss, fetches from Gravatar, writes the result to disk, and returns the bytes.
///
/// Uses a blocking HTTP client — safe to call from gpui background executor threads,
/// which do not run a Tokio reactor. Returns `None` if the request fails, times
/// out, or the server returns a non-success status (including 404 for unregistered
/// emails).
pub fn fetch_avatar_bytes(email: String) -> Option<Vec<u8>> {
    if let Some(cached) = load_cached_avatar() {
        return Some(cached);
    }

    let url = gravatar_url(&email);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let response = client.get(&url).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    let bytes = response.bytes().ok().map(|b| b.to_vec())?;
    save_cached_avatar(&bytes);
    Some(bytes)
}
