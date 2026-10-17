//! Linux (and any other non-macOS, non-Windows Unix) platform
//! implementation: present-only file state, no materialize concept,
//! `xdg-open` for reveal.

use std::io;
use std::path::Path;

use super::{Capabilities, FileManagerKind, FileState};

pub(super) fn file_state(path: &Path) -> FileState {
    if path.exists() { FileState::Present } else { FileState::Missing }
}

pub(super) fn materialize(_path: &Path) -> io::Result<()> {
    // Not applicable: this platform has no cloud-placeholder concept, so
    // every present file is already fully materialized.
    Ok(())
}

pub(super) fn reveal_in_file_manager(path: &Path) -> io::Result<()> {
    // Selecting a specific file via DBus
    // (`org.freedesktop.FileManager1.ShowItems`) is not yet implemented;
    // falls back to opening the parent directory.
    let parent = path.parent().unwrap_or(path);
    std::process::Command::new("xdg-open").arg(parent).status().map(|_| ())
}

pub(super) fn capabilities() -> Capabilities {
    Capabilities { supports_offload_detection: false,
                   supports_materialize: false,
                   file_manager: FileManagerKind::Generic }
}
