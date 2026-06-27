//! Gravatar URL computation and avatar image fetching.

use std::time::Duration;

/// Computes a Gravatar avatar URL for the given email address.
///
/// Trims, lowercases, and MD5-hashes the email per the Gravatar specification.
/// `d=404` returns HTTP 404 for unregistered emails, allowing callers to detect
/// missing avatars rather than receiving a generic fallback image.
pub fn gravatar_url(email: &str) -> String {
    let hash = format!("{:x}", md5::compute(email.trim().to_lowercase()));
    format!("https://www.gravatar.com/avatar/{hash}?d=404&s=64")
}

/// Fetches raw avatar image bytes from Gravatar for the given email.
///
/// Uses a blocking HTTP client — safe to call from gpui background executor threads,
/// which do not run a Tokio reactor.  Returns `None` if the request fails, times
/// out, or the server returns a non-success status (including 404 for unregistered
/// emails).
pub fn fetch_avatar_bytes(email: String) -> Option<Vec<u8>> {
    let url = gravatar_url(&email);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let response = client.get(&url).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    response.bytes().ok().map(|b| b.to_vec())
}
