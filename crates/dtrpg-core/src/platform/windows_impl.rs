//! Windows platform implementation: Cloud Files (OneDrive/placeholder)
//! offloaded-file detection via `GetFileAttributesW`'s
//! `FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS`/`FILE_ATTRIBUTE_RECALL_ON_OPEN`
//! flags, materialize by opening the file (which the OS recalls on access --
//! there is no separate "force download" API the way iCloud has one),
//! reveal via `explorer /select,`.
//!
//! Not compiled or tested on this machine (no Windows build available in
//! this session) -- written against the documented `windows` crate API
//! surface; verify on a Windows CI run before relying on it.

use std::io::{self, Read as _};
use std::path::Path;

use windows::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS, FILE_ATTRIBUTE_RECALL_ON_OPEN,
    GetFileAttributesW, INVALID_FILE_ATTRIBUTES,
};
use windows::core::HSTRING;

use super::{Capabilities, FileManagerKind, FileState};

pub(super) fn file_state(path: &Path) -> FileState {
    if !path.exists() {
        return FileState::Missing;
    }
    let wide = HSTRING::from(path.as_os_str());
    // SAFETY: `wide` is a valid, null-terminated UTF-16 string for the
    // lifetime of this call; `GetFileAttributesW` does not retain it.
    let attrs = unsafe { GetFileAttributesW(&wide) };
    if attrs == INVALID_FILE_ATTRIBUTES {
        return FileState::Missing;
    }
    let recall = (FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS.0 | FILE_ATTRIBUTE_RECALL_ON_OPEN.0)
                 & attrs
                 != 0;
    if recall { FileState::Offloaded } else { FileState::Present }
}

pub(super) fn materialize(path: &Path) -> io::Result<()> {
    // Windows has no separate "force download" call the way iCloud's
    // `startDownloadingUbiquitousItemAtURL` does -- opening (and reading
    // from) a Cloud Files placeholder is itself what triggers the OS to
    // recall its data, per the ADR's "open to trigger recall" decision.
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; 1];
    let _ = file.read(&mut buf);
    Ok(())
}

pub(super) fn reveal_in_file_manager(path: &Path) -> io::Result<()> {
    std::process::Command::new("explorer").arg(format!("/select,{}", path.display()))
                                          .status()
                                          .map(|_| ())
}

pub(super) fn capabilities() -> Capabilities {
    Capabilities { supports_offload_detection: true,
                   supports_materialize: true,
                   file_manager: FileManagerKind::Explorer }
}
