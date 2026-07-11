//! Streams a prepared download to disk.
//!
//! `prepare_download`'s response (`data.attributes.url`) is a watermarking
//! portal URL that redirects to a pre-signed object-storage URL with a short
//! expiry (observed: 30 seconds) — the fetch below happens immediately after
//! preparation, in the same call, so there is no gap for that URL to expire.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use dtrpg_ui::services::{LibraryServiceError, LibraryServiceErrorKind};

use super::gateway::SdkLibraryGateway;

const CHUNK_SIZE: usize = 64 * 1024;

/// Downloads the file identified by `(order_product_id, index)` to `dest`,
/// via `gateway.prepare_download`. See
/// [`LibraryService::download_item`](dtrpg_ui::services::LibraryService::download_item)
/// for the cancellation and temp-file contract.
pub(super) fn download_item(gateway: &dyn SdkLibraryGateway, order_product_id: u64, index: u32,
                            dest: &Path, cancel: &AtomicBool)
                            -> Result<(), LibraryServiceError> {
    let response = gateway.prepare_download(order_product_id, index)?;
    let url = response.pointer("/data/attributes/url")
                       .and_then(|v| v.as_str())
                       .ok_or_else(|| {
                           LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                "prepare_download response missing data.attributes.url",
            )
                       })?;

    let part_path = part_path_for(dest);
    match stream_to_file(url, &part_path, cancel) {
        Ok(()) => std::fs::rename(&part_path, dest).map_err(|e| {
                      LibraryServiceError::new(LibraryServiceErrorKind::Network,
                                               format!("failed to finalize download at {}: {e}",
                                                       dest.display()))
                  }),
        Err(e) => {
            let _ = std::fs::remove_file(&part_path);
            Err(e)
        }
    }
}

/// Derives `{dest}.part` — appends rather than replaces `dest`'s extension,
/// so `Foo.pdf` becomes `Foo.pdf.part`, not `Foo.part`.
fn part_path_for(dest: &Path) -> PathBuf {
    let mut name = dest.file_name()
                       .map(std::ffi::OsStr::to_os_string)
                       .unwrap_or_default();
    name.push(".part");
    dest.with_file_name(name)
}

fn stream_to_file(url: &str, part_path: &Path, cancel: &AtomicBool)
                  -> Result<(), LibraryServiceError> {
    if let Some(parent) = part_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
                                           LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                format!("failed to create download directory {}: {e}", parent.display()),
            )
                                       })?;
    }

    let mut response = reqwest::blocking::get(url).map_err(|e| {
                           LibraryServiceError::new(LibraryServiceErrorKind::Network,
                                                    format!("download request failed: {e}"))
                       })?;
    if !response.status().is_success() {
        return Err(LibraryServiceError::new(LibraryServiceErrorKind::Network,
                                            format!("download failed: HTTP {}",
                                                    response.status())));
    }

    let mut file = std::fs::File::create(part_path).map_err(|e| {
                                                       LibraryServiceError::new(
            LibraryServiceErrorKind::Network,
            format!("failed to create {}: {e}", part_path.display()),
        )
                                                   })?;

    let mut buf = [0u8; CHUNK_SIZE];
    loop {
        if cancel.load(Ordering::SeqCst) {
            return Err(LibraryServiceError::new(LibraryServiceErrorKind::Network,
                                                "download cancelled"));
        }
        let n = response.read(&mut buf).map_err(|e| {
                                            LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                format!("download read failed: {e}"),
            )
                                        })?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n]).map_err(|e| {
                                       LibraryServiceError::new(LibraryServiceErrorKind::Network,
                                                                format!(
                    "download write failed: {e}"
                ))
                                   })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_path_appends_extension() {
        let dest = Path::new("/tmp/items/b1/Book.pdf");
        assert_eq!(part_path_for(dest),
                   Path::new("/tmp/items/b1/Book.pdf.part"));
    }

    #[test]
    fn part_path_handles_no_extension() {
        let dest = Path::new("/tmp/items/b1/README");
        assert_eq!(part_path_for(dest), Path::new("/tmp/items/b1/README.part"));
    }
}
