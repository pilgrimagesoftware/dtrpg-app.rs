//! Platform-integration trait: one `Platform` with a per-OS implementation,
//! gated behind `[target.'cfg(...)'.dependencies]` so `objc2` and `windows`
//! never compile into the wrong target. See
//! `docs/adr/0004-platform-integration-layer.md`.
//!
//! `Capabilities` is the runtime contract the UI (and the MCP server)
//! queries before offering any OS-native affordance -- Linux does not get a
//! materialize button it cannot act on, and Windows does not get a
//! Finder-only reveal label.

#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod other;
#[cfg(target_os = "windows")]
mod windows_impl;

use std::io;
use std::path::Path;

/// Where a file's data actually is, from the platform's perspective.
///
/// Distinct from [`crate::data::enums::ItemStatus`] -- that is this app's
/// own "has the user downloaded this" concept; `FileState` is what the
/// platform's cloud-storage integration (iCloud Drive on macOS, Cloud Files
/// on Windows) reports about a file this app already believes is
/// downloaded. A `Downloaded` item can still be `Offloaded` here if the OS
/// evicted its local data to reclaim disk space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    /// The file's data is fully present on local disk.
    Present,
    /// The platform evicted the file's local data to reclaim disk space;
    /// reading it would trigger an on-demand re-download.
    Offloaded,
    /// The platform is currently re-downloading the file's data.
    Downloading,
    /// No file exists at the given path.
    Missing,
}

/// Which native file manager [`Platform::reveal_in_file_manager`] opens, so
/// UI copy can name it correctly without its own `cfg(target_os)` branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileManagerKind {
    Finder,
    Explorer,
    /// A generic file manager (Linux has no single standard one;
    /// `xdg-open` defers to whatever the desktop environment registered).
    Generic,
}

/// Runtime capabilities the current platform supports, queried before
/// offering an OS-native affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    /// Whether [`Platform::file_state`] can distinguish [`FileState::Offloaded`]/
    /// [`FileState::Downloading`] from [`FileState::Present`] on this
    /// platform. `false` on Linux, which has no equivalent cloud-placeholder
    /// concept -- a present file is just present.
    pub supports_offload_detection: bool,
    /// Whether [`Platform::materialize`] can force a placeholder file to
    /// download. `false` on Linux, where it is not applicable.
    pub supports_materialize: bool,
    /// Which native file manager [`Platform::reveal_in_file_manager`] opens.
    pub file_manager: FileManagerKind,
}

/// One platform-integration surface: file state, forcing a placeholder to
/// download, and revealing a path in the native file manager.
///
/// Implemented once per OS (macOS, Windows, "other" covering Linux and any
/// other Unix), selected at compile time via `cfg(target_os = ...)` --
/// never more than one implementation's dependencies (`objc2`, `windows`)
/// are compiled into a given build.
pub trait Platform {
    /// Reports where `path`'s data actually is. Returns
    /// [`FileState::Missing`] if no file exists at `path`.
    fn file_state(&self, path: &Path) -> FileState;

    /// Forces a placeholder file to download its data, if the platform
    /// supports it (see [`Capabilities::supports_materialize`]).
    ///
    /// A no-op `Ok(())` on a platform with no materialize concept (Linux),
    /// or if `path` is already [`FileState::Present`].
    ///
    /// # Errors
    ///
    /// Returns an error if the platform call fails.
    fn materialize(&self, path: &Path) -> io::Result<()>;

    /// Opens the native file manager with `path` selected or revealed.
    ///
    /// # Errors
    ///
    /// Returns an error if the OS command/call fails to launch. A missing
    /// file is NOT an error at this layer; callers should check existence
    /// beforehand.
    fn reveal_in_file_manager(&self, path: &Path) -> io::Result<()>;

    /// Returns this platform's runtime capabilities.
    fn capabilities(&self) -> Capabilities;
}

/// The current build's `Platform` implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct CurrentPlatform;

/// Returns the current build's `Platform` implementation.
#[must_use]
pub fn current() -> CurrentPlatform {
    CurrentPlatform
}

impl Platform for CurrentPlatform {
    fn file_state(&self, path: &Path) -> FileState {
        #[cfg(target_os = "macos")]
        return macos::file_state(path);
        #[cfg(target_os = "windows")]
        return windows_impl::file_state(path);
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        return other::file_state(path);
    }

    fn materialize(&self, path: &Path) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        return macos::materialize(path);
        #[cfg(target_os = "windows")]
        return windows_impl::materialize(path);
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        return other::materialize(path);
    }

    fn reveal_in_file_manager(&self, path: &Path) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        return macos::reveal_in_file_manager(path);
        #[cfg(target_os = "windows")]
        return windows_impl::reveal_in_file_manager(path);
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        return other::reveal_in_file_manager(path);
    }

    fn capabilities(&self) -> Capabilities {
        #[cfg(target_os = "macos")]
        return macos::capabilities();
        #[cfg(target_os = "windows")]
        return windows_impl::capabilities();
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        return other::capabilities();
    }
}
