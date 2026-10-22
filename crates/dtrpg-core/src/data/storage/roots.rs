//! Named storage roots: each download location the user has configured,
//! with on-demand capacity queries (used/total megabytes).

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// One configured download location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageRoot {
    /// Stable identifier, referenced by [`super::placement::PlacementRule`]
    /// and by `StorageConfig::default_root_id`. Not reused after a root is
    /// removed, so a stale rule pointing at a removed root's id resolves to
    /// "no match" rather than silently matching whatever root is later
    /// assigned the same id.
    pub id:   u32,
    /// Display name shown in the Settings roots list, e.g. "Internal SSD"
    /// or "External Archive".
    pub name: String,
    /// The root directory itself.
    pub path: PathBuf,
}

/// Disk capacity for a [`StorageRoot`], in megabytes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootCapacity {
    /// Total size of the volume the root lives on, if it could be measured.
    pub total_mb:     Option<f64>,
    /// Space already used by this root's own contents (`items/` and
    /// `collections/` combined), if it could be measured.
    pub used_mb:      Option<f64>,
    /// Free space remaining on the volume the root lives on, if it could
    /// be measured (e.g. an unmounted volume reports `None`).
    pub available_mb: Option<f64>,
}

/// Operational state of a configured storage root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootHealth {
    /// The root exists as a directory and its volume reports free space.
    Ready,
    /// The configured path does not exist.
    Missing,
    /// The path exists but is not a directory.
    NotDirectory,
    /// The path or its volume could not be queried.
    Unavailable,
}

/// What kind of place a storage root is, which the settings page shows as a
/// badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootKind {
    /// A folder on the computer's own disk.
    Local,
    /// A folder on a disk that can be unplugged (an external or network drive).
    External,
    /// A folder that a cloud service (iCloud Drive, OneDrive, Dropbox, ...)
    /// keeps in step with the cloud.
    Cloud,
}

/// Folder names that mark a folder as kept by a cloud service, ignoring case.
const CLOUD_FOLDER_MARKERS: [&str; 7] = ["mobile documents",
                                         "icloud",
                                         "onedrive",
                                         "dropbox",
                                         "google drive",
                                         "googledrive",
                                         "cloudstorage"];

/// Where drives that can be unplugged are mounted, on macOS and Linux.
const EXTERNAL_MOUNT_PREFIXES: [&str; 4] = ["/Volumes/", "/media/", "/mnt/", "/run/media/"];

/// Guesses the kind of place `path` is from where it is: a cloud service's
/// folder, a mounted external drive, or the computer's own disk. It is a guess
/// from the path alone, so an unusual setup can be called the wrong thing.
///
/// A cloud folder on an external drive counts as cloud.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use dtrpg_core::data::storage::roots::{RootKind, root_kind};
///
/// assert_eq!(root_kind(Path::new("/Volumes/Vault/RPG")),
///            RootKind::External);
/// assert_eq!(root_kind(Path::new("/Users/me/Dropbox/Libri")),
///            RootKind::Cloud);
/// assert_eq!(root_kind(Path::new("/Users/me/Documents/Libri")),
///            RootKind::Local);
/// ```
#[must_use]
pub fn root_kind(path: &std::path::Path) -> RootKind {
    let is_cloud = path.components().any(|component| {
                                        let name =
                                            component.as_os_str().to_string_lossy().to_lowercase();
                                        CLOUD_FOLDER_MARKERS.iter()
                                                            .any(|marker| name.contains(marker))
                                    });
    if is_cloud {
        return RootKind::Cloud;
    }
    let text = path.to_string_lossy();
    let on_other_windows_drive =
        text.len() >= 3 && text.as_bytes()[1] == b':' && !text[..1].eq_ignore_ascii_case("c");
    if EXTERNAL_MOUNT_PREFIXES.iter()
                              .any(|prefix| text.starts_with(prefix))
       || on_other_windows_drive
    {
        return RootKind::External;
    }
    RootKind::Local
}

/// Returns the total size, in megabytes, of every regular file under
/// `path`, recursing into subdirectories.
///
/// Returns `None` if `path` does not exist or cannot be read at all;
/// a partially-readable tree (e.g. one unreadable subdirectory) still
/// returns the sum of what could be read rather than `None`, since a
/// capacity estimate that's slightly low is more useful than no estimate.
#[must_use]
pub fn used_space_mb(path: &std::path::Path) -> Option<f64> {
    if !path.exists() {
        return None;
    }
    let mut total_bytes: u64 = 0;
    let mut stack = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir)
        else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(metadata) = entry.metadata()
            else {
                continue;
            };
            if metadata.is_dir() {
                stack.push(entry.path());
            }
            else {
                total_bytes += metadata.len();
            }
        }
    }
    Some(total_bytes as f64 / 1_048_576.0)
}

/// Returns `root`'s capacity: used space under its own directory tree, and
/// free space on the volume it lives on.
#[must_use]
pub fn root_capacity(root: &StorageRoot) -> RootCapacity {
    RootCapacity { total_mb:
                       fs4::total_space(&root.path).ok()
                                                   .map(|bytes| bytes as f64 / 1_048_576.0),
                   used_mb:      used_space_mb(&root.path),
                   available_mb:
                       fs4::available_space(&root.path).ok()
                                                       .map(|bytes| bytes as f64 / 1_048_576.0), }
}

/// Free and total space, in megabytes, of the volume `root` lives on. Unlike
/// [`root_capacity`] it does not walk the root's files, so it is cheap.
#[must_use]
pub fn volume_space_mb(root: &StorageRoot) -> (Option<f64>, Option<f64>) {
    let megabytes = |bytes: u64| bytes as f64 / 1_048_576.0;
    (fs4::available_space(&root.path).ok().map(megabytes),
     fs4::total_space(&root.path).ok().map(megabytes))
}

/// Checks whether a root is usable without creating files or changing it.
#[must_use]
pub fn root_health(root: &StorageRoot) -> RootHealth {
    match root.path.try_exists() {
        Ok(false) => RootHealth::Missing,
        Err(_) => RootHealth::Unavailable,
        Ok(true) => match std::fs::metadata(&root.path) {
            Ok(metadata) if !metadata.is_dir() => RootHealth::NotDirectory,
            Ok(_) if fs4::available_space(&root.path).is_ok() => RootHealth::Ready,
            Ok(_) => RootHealth::Unavailable,
            Err(_) => RootHealth::Unavailable,
        },
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("dtrpg_storage_roots_test_{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn used_space_mb_is_none_for_a_missing_directory() {
        let dir = test_dir("missing");
        assert!(used_space_mb(&dir).is_none());
    }

    #[test]
    fn used_space_mb_sums_files_recursively() {
        let dir = test_dir("sums_recursively");
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("a.txt"), vec![0u8; 1_048_576]).unwrap(); // 1 MB
        std::fs::write(dir.join("sub/b.txt"), vec![0u8; 2_097_152]).unwrap(); // 2 MB

        let mb = used_space_mb(&dir).unwrap();

        std::fs::remove_dir_all(&dir).unwrap();
        assert!((mb - 3.0).abs() < 0.01, "expected ~3 MB, got {mb}");
    }

    #[test]
    fn root_capacity_reports_used_and_available() {
        let dir = test_dir("capacity");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), vec![0u8; 1_048_576]).unwrap();
        let root = StorageRoot { id:   1,
                                 name: "Test".to_string(),
                                 path: dir.clone(), };

        let capacity = root_capacity(&root);

        std::fs::remove_dir_all(&dir).unwrap();
        assert!((capacity.used_mb.unwrap() - 1.0).abs() < 0.01);
        assert!(capacity.available_mb.is_some());
    }

    #[test]
    fn root_health_reports_missing_and_ready_roots() {
        let missing = StorageRoot { id:   1,
                                    name: "Missing".to_string(),
                                    path: test_dir("health-missing"), };
        assert_eq!(root_health(&missing), RootHealth::Missing);

        let ready_path = test_dir("health-ready");
        std::fs::create_dir_all(&ready_path).unwrap();
        let ready = StorageRoot { id:   2,
                                  name: "Ready".to_string(),
                                  path: ready_path.clone(), };
        assert_eq!(root_health(&ready), RootHealth::Ready);
        std::fs::remove_dir_all(ready_path).unwrap();
    }
}
