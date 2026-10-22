//! The download sheet: building it for an item and carrying out its choices.

use super::*;
use crate::view_models::download_sheet::{DownloadPlan, DownloadSheet};

impl LibraryController {
    /// Builds the download sheet for `entry_id`, or `None` if the entry is not
    /// in the catalog or has no file list to choose from.
    ///
    /// Reads the storage settings and the free space of each root, so call it
    /// when the user asks for the sheet, never while rendering.
    #[must_use]
    pub fn download_sheet_for(&self, entry_id: &str) -> Option<DownloadSheet> {
        let item = self.item_by_id(entry_id)
                       .filter(|item| !item.files.is_empty())?;
        let collection_ids = collection_ids_for_item(&item, &self.library.collections());
        Some(DownloadSheet::new(&item,
                                &StorageConfig::load(),
                                &collection_ids,
                                available_space_mb_at))
    }

    /// Carries out what the user chose in the download sheet: remembers the
    /// destination as a rule for the publisher if asked to, then queues each
    /// chosen file.
    pub fn start_sheet_download(&mut self, entry_id: &str, plan: &DownloadPlan,
                                cx: &mut Context<Self>) {
        let Some(item) = self.item_by_id(entry_id)
        else {
            return;
        };
        if let Some(root_id) = plan.remember_root_id {
            StorageConfig::load().remember_publisher_root(&item.publisher, root_id);
            cx.emit(PlacementRulesChanged);
        }
        let title = item.title.to_string();
        for &index in &plan.file_indexes {
            self.enqueue_item_download(entry_id, index, title.clone(), cx);
        }
    }
}
