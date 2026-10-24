//! A folder of symlinks to the files of a smart collection's members.

use std::io;
use std::path::{Path, PathBuf};

use crate::data::storage::sanitize_path_component;

#[cfg(test)]
mod tests;

/// The folder under `root` that mirrors the smart collection called `name`.
/// It is not the folder of a manual collection of the same name, so the two
/// never share links.
#[must_use]
pub fn smart_collection_dir(root: &Path, name: &str) -> PathBuf {
    root.join(crate::data::constants::SMART_COLLECTIONS_DIR)
        .join(sanitize_path_component(name))
}

/// Makes `dir` hold exactly one symlink for each of `files`, named like the
/// file. Links to other files are removed, and anything in `dir` that is not a
/// symlink is left alone. Returns how many links the folder holds afterwards.
///
/// # Errors
///
/// Fails if `dir` cannot be created or read. A link that cannot be made or
/// removed is skipped, so one bad file does not stop the others.
///
/// # Examples
///
/// ```no_run
/// use std::path::{Path, PathBuf};
///
/// use dtrpg_core::data::smart_collection::sync_mirror;
///
/// let files = [PathBuf::from("/library/items/Paizo/Core.pdf")];
/// let links = sync_mirror(Path::new("/library/smart-collections/Solo"), &files)?;
/// assert_eq!(links, 1);
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn sync_mirror(dir: &Path, files: &[PathBuf]) -> io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let wanted = std::fs::read_link(&path).is_ok_and(|target| files.contains(&target));
        let is_link = path.symlink_metadata()
                          .is_ok_and(|meta| meta.file_type().is_symlink());
        if is_link && !wanted {
            let _ = std::fs::remove_file(&path);
        }
    }
    let mut links = 0;
    for file in files {
        let Some(name) = file.file_name()
        else {
            continue;
        };
        let link = dir.join(name);
        if link.symlink_metadata().is_ok() || make_link(file, &link).is_ok() {
            links += 1;
        }
    }
    Ok(links)
}

/// Removes the symlinks in `dir`, and `dir` itself if nothing else is in it.
/// A folder that is not there is already clean.
pub fn clear_mirror(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir)
    else {
        return;
    };
    for path in entries.flatten().map(|entry| entry.path()) {
        if path.symlink_metadata()
               .is_ok_and(|meta| meta.file_type().is_symlink())
        {
            let _ = std::fs::remove_file(&path);
        }
    }
    let _ = std::fs::remove_dir(dir);
}

#[cfg(unix)]
fn make_link(target: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn make_link(target: &Path, link: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
