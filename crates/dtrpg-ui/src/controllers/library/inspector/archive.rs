//! Finding the archive file the inspector's Contents tab is showing.

use std::path::PathBuf;

use dtrpg_core::util::file_presence::{collection_ids_for_item, resolved_file_path_in_collections};

use super::super::*;

/// Whether a file format is one the Contents tab can list.
fn is_archive_format(format: &str) -> bool {
    matches!(format.to_ascii_uppercase().as_str(),
             "ZIP" | "TAR" | "TAR.GZ" | "TGZ" | "TAR.BZ2" | "TBZ2" | "TAR.XZ" | "TXZ")
}

impl LibraryController {
    /// The path on disk of the archive the Contents tab shows for `entry_id`:
    /// the selected one of its archive files.
    ///
    /// Resolved the way the download put it there, by the file's own size and
    /// the item's collections, so a placement rule that sends it to another
    /// location is followed. `None` if the item has no archive, or none at the
    /// selected position.
    #[must_use]
    pub(crate) fn selected_archive_path(&self, entry_id: &str) -> Option<PathBuf> {
        let item = self.item_by_id(entry_id)?;
        let file = item.files
                       .iter()
                       .filter(|file| is_archive_format(&file.format))
                       .nth(self.inspector_selected_archive_for(entry_id))?;
        let collection_ids = collection_ids_for_item(&item, &self.library.collections());
        Some(resolved_file_path_in_collections(&StorageConfig::load(),
                                               &item,
                                               file,
                                               &collection_ids))
    }
}
