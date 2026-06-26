//! Well-known service name and account key constants for the credential store.
//!
//! All credential store call sites MUST use these constants rather than
//! inline strings to ensure consistent namespacing and enable targeted
//! deletion on uninstall.

/// Reverse-DNS service namespace used for all keyring entries.
pub const SERVICE: &str = "com.pilgrimagesoftware.dtrpg";

/// Account key for the DriveThruRPG API key credential.
pub const API_KEY: &str = "api-key";

/// Account key for the OAuth access token credential.
pub const ACCESS_TOKEN: &str = "access-token";

/// Account key for the OAuth refresh token credential.
pub const REFRESH_TOKEN: &str = "refresh-token";
