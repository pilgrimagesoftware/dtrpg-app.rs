//! Well-known service name and account key constants for the credential store.
//!
//! All credential store call sites MUST use these constants rather than
//! inline strings to ensure consistent namespacing and enable targeted
//! deletion on uninstall.

/// Reverse-DNS service namespace used for all keyring entries.
pub const SERVICE: &str = "com.pilgrimagesoftware.dtrpg";

/// Account key for the DriveThruRPG API key credential.
///
/// This is the only credential persisted to the keychain. Access tokens and
/// refresh tokens are kept in memory and re-acquired at startup.
pub const API_KEY: &str = "api-key";
