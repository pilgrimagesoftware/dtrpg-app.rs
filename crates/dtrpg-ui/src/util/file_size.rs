//! On-disk file size lookup for the detail view's file size fields.
//!
//! Resolves a file's real byte size from the filesystem at render time, so a
//! `Downloaded` item's detail view can show its actual size instead of the
//! catalog-reported estimate. See `entry_dir` resolution convention in
//! `util::item_opener::ItemOpener::open_item`.

use std::path::Path;

use rust_i18n::t;

/// Returns the on-disk size in bytes of `entry_dir.join(file_name)`, or
/// `None` if the file doesn't exist, `file_name` is empty, or the path can't
/// be read.
pub fn on_disk_file_size(entry_dir: &Path, file_name: &str) -> Option<u64> {
    if file_name.is_empty() {
        return None;
    }
    std::fs::metadata(entry_dir.join(file_name)).ok()
                                                .map(|m| m.len())
}

/// Formats a byte count as a `"{:.1} MB"`-style string, matching the
/// existing catalog-size formatting used elsewhere in the detail view. The
/// unit label is localized via the `size.mb` translation key.
pub fn format_bytes(bytes: u64) -> String {
    format!("{:.1} {}", bytes as f64 / (1024.0 * 1024.0), t!("size.mb"))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn on_disk_file_size_returns_size_for_existing_file() {
        let dir = std::env::temp_dir().join("dtrpg_on_disk_file_size_present_test");
        let _ = std::fs::create_dir_all(&dir);
        let file_path = dir.join("book.pdf");
        let Ok(mut file) = File::create(&file_path)
        else {
            return;
        };
        let _ = file.write_all(b"0123456789");
        drop(file);

        let result = on_disk_file_size(&dir, "book.pdf");

        let _ = std::fs::remove_file(&file_path);
        let _ = std::fs::remove_dir(&dir);

        assert_eq!(result, Some(10));
    }

    #[test]
    fn on_disk_file_size_returns_none_for_missing_file() {
        let dir = std::env::temp_dir().join("dtrpg_on_disk_file_size_missing_test");
        let result = on_disk_file_size(&dir, "does-not-exist.pdf");
        assert_eq!(result, None);
    }

    #[test]
    fn on_disk_file_size_returns_none_for_empty_file_name() {
        let dir = std::env::temp_dir();
        let result = on_disk_file_size(&dir, "");
        assert_eq!(result, None);
    }

    #[test]
    fn format_bytes_matches_existing_mb_style() {
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1_572_864), "1.5 MB");
    }
}
