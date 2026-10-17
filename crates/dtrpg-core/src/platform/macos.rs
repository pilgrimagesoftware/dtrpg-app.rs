//! macOS platform implementation: iCloud Drive offloaded-file detection via
//! `NSURL`'s `NSURLUbiquitousItemDownloadingStatusKey`/
//! `NSURLUbiquitousItemIsDownloadingKey` resource values, materialize via
//! `startDownloadingUbiquitousItemAtURL:error:`, reveal via `open -R`
//! (Finder).

use std::io;
use std::path::Path;

use objc2_foundation::{
    NSArray, NSFileManager, NSNumber, NSString, NSURL, NSURLUbiquitousItemDownloadingStatusKey,
    NSURLUbiquitousItemDownloadingStatusNotDownloaded, NSURLUbiquitousItemIsDownloadingKey,
};

use super::{Capabilities, FileManagerKind, FileState};

fn file_url(path: &Path) -> objc2::rc::Retained<NSURL> {
    NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()))
}

/// The three `NSURLResourceKey`/status-value constants `file_state` needs,
/// read once from their `extern "C"` statics.
///
/// # Safety
///
/// None of these is actually unsafe to read -- they are immutable,
/// process-lifetime `&'static` constants exported by Apple's Foundation
/// framework -- but `extern` statics require an `unsafe` block to
/// reference regardless of mutability, per Rust's FFI rules.
#[allow(unsafe_code)]
fn ubiquitous_item_keys()
-> (&'static objc2_foundation::NSURLResourceKey, &'static objc2_foundation::NSURLResourceKey,
    &'static NSString) {
    unsafe {
        (NSURLUbiquitousItemDownloadingStatusKey, NSURLUbiquitousItemIsDownloadingKey,
         NSURLUbiquitousItemDownloadingStatusNotDownloaded)
    }
}

pub(super) fn file_state(path: &Path) -> FileState {
    if !path.exists() {
        return FileState::Missing;
    }
    let url = file_url(path);
    let manager = NSFileManager::defaultManager();
    if !manager.isUbiquitousItemAtURL(&url) {
        // Not an iCloud Drive item at all; its data is simply local.
        return FileState::Present;
    }

    let (status_key, is_downloading_key, not_downloaded) = ubiquitous_item_keys();
    let keys = NSArray::from_slice(&[status_key, is_downloading_key]);
    let Ok(values) = url.resourceValuesForKeys_error(&keys)
    else {
        return FileState::Present;
    };

    let is_downloading = values.objectForKey(is_downloading_key)
                               .and_then(|v| v.downcast::<NSNumber>().ok())
                               .is_some_and(|n| n.as_bool());
    if is_downloading {
        return FileState::Downloading;
    }

    let status = values.objectForKey(status_key).and_then(|v| v.downcast::<NSString>().ok());
    match status {
        Some(s) if &*s == not_downloaded => FileState::Offloaded,
        // `Downloaded`/`Current`, or an unreadable status -- either way the
        // data is locally present.
        _ => FileState::Present,
    }
}

pub(super) fn materialize(path: &Path) -> io::Result<()> {
    let url = file_url(path);
    NSFileManager::defaultManager().startDownloadingUbiquitousItemAtURL_error(&url)
                                   .map_err(|e| io::Error::other(e.to_string()))
}

pub(super) fn reveal_in_file_manager(path: &Path) -> io::Result<()> {
    std::process::Command::new("open").args(["-R", &path.to_string_lossy()])
                                      .status()
                                      .map(|_| ())
}

pub(super) fn capabilities() -> Capabilities {
    Capabilities { supports_offload_detection: true,
                   supports_materialize: true,
                   file_manager: FileManagerKind::Finder }
}
