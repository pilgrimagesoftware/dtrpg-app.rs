//! Service trait and error types for library data access.

use crate::data::library::LibraryItem;

/// The type of service failure returned by library operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibraryServiceErrorKind {
    /// Request failed due to transient connectivity or SDK configuration.
    Network,
    /// Request failed due to session or authentication state.
    Session,
    /// Request referenced a non-existent item.
    NotFound,
    /// Session has expired and re-authentication is required.
    ///
    /// # TODO
    ///
    /// Full token-refresh handling is deferred until `connect-sdk-to-rust-app` lands.
    NeedsReauth,
}

/// Error returned by library service operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryServiceError {
    /// The machine-classified failure kind.
    pub kind: LibraryServiceErrorKind,
    /// Human-readable baseline error message.
    pub message: String,
}

impl LibraryServiceError {
    /// Creates a new service error.
    pub fn new(kind: LibraryServiceErrorKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }

    /// Returns a multi-line string suitable for display in the activity panel,
    /// combining the error message with a kind-specific user hint.
    pub fn panel_detail(&self) -> String {
        let hint = match self.kind {
            LibraryServiceErrorKind::Network => {
                "Check your internet connection and try again."
            }
            LibraryServiceErrorKind::Session => {
                "Your access token may be missing or expired. Try signing out and back in."
            }
            LibraryServiceErrorKind::NotFound => {
                "The requested item could not be found."
            }
            LibraryServiceErrorKind::NeedsReauth => {
                "Your session has expired. Please sign out and sign back in."
            }
        };
        format!("{self}\n{hint}")
    }
}

impl std::fmt::Display for LibraryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.message, self.kind)
    }
}

impl std::error::Error for LibraryServiceError {}

/// Service boundary consumed by the library view model.
///
/// Implementations may be SDK-backed HTTP adapters or deterministic test stubs.
/// The service is responsible for fetching and mapping all data; callers receive
/// fully-formed [`LibraryItem`] values ready for presentation.
pub trait LibraryService: Send + Sync + 'static {
    /// Loads the full library item list.
    ///
    /// # Errors
    ///
    /// Returns a [`LibraryServiceError`] if the request fails or the session is invalid.
    fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError>;

    /// Loads library items page-by-page, invoking `on_page` after each page arrives.
    ///
    /// The default implementation calls [`list_items`] and delivers all items in a
    /// single `on_page` call. Implementations that have access to pagination should
    /// override this to call `on_page` incrementally so callers can update the UI
    /// without waiting for all pages.
    ///
    /// # Errors
    ///
    /// Returns a [`LibraryServiceError`] if any page request fails. Items delivered
    /// to `on_page` before the failure are not rolled back.
    ///
    /// [`list_items`]: LibraryService::list_items
    fn list_items_paged(
        &self,
        on_page: &mut dyn FnMut(Vec<LibraryItem>),
    ) -> Result<(), LibraryServiceError> {
        on_page(self.list_items()?);
        Ok(())
    }

    /// Loads detail data for a selected item by its numeric API identifier.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryServiceError`] with kind [`LibraryServiceErrorKind::NotFound`]
    /// if the id does not match any item, or a network/session error if the request fails.
    fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError>;
}

// ── LoginService ──────────────────────────────────────────────────────────────

/// Tokens returned by a successful login.
pub struct LoginTokens {
    /// Short-lived JWT bearer token for API requests.
    pub access_token: String,
    /// Long-lived token used to refresh the access token.
    pub refresh_token: String,
    /// Unix timestamp (seconds) at which the refresh token expires.
    pub refresh_token_ttl: u64,
}

/// Error returned by [`LoginService::authenticate`].
#[derive(Debug, Clone)]
pub struct LoginError(pub String);

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Service boundary for the login flow.
///
/// Implementations exchange a DriveThruRPG API key for JWT session tokens.
pub trait LoginService: Send + Sync + 'static {
    /// Authenticates with the given API key and returns session tokens.
    ///
    /// # Errors
    ///
    /// Returns [`LoginError`] if authentication fails (network error, invalid key, etc.).
    fn authenticate(&self, api_key: &str) -> Result<LoginTokens, LoginError>;
}

#[cfg(test)]
pub mod stub;
