//! `Library` methods that record the page count of each downloaded file.

use std::sync::PoisonError;

use crate::data::storage::StorageConfig;
use crate::library::Library;
use crate::util::file_presence::{collection_ids_for_item, resolved_file_path_in_collections};
use crate::util::pdf_pages::pdf_page_count;

impl Library {
    /// Reads the page count of each downloaded file of `item_id` from the file,
    /// and records it, replacing what was recorded. A file that is not a PDF,
    /// or cannot be read, is recorded with 0 pages so it is not read again.
    ///
    /// Opens every file, so it belongs on a background thread. A failure to
    /// write is logged, not returned: the record is rebuilt by
    /// [`Self::record_missing_page_counts`] on a later run.
    pub fn record_page_counts(&self, storage: &StorageConfig, item_id: &str) {
        let Some(item) = self.item_by_id(item_id)
        else {
            return;
        };
        let collection_ids = collection_ids_for_item(&item, &self.collections());
        let pages: Vec<(String, u32)> =
            item.files
                .iter()
                .filter(|file| file.downloaded)
                .map(|file| {
                    let count = if file.format.eq_ignore_ascii_case("pdf") {
                        let path = resolved_file_path_in_collections(storage,
                                                                     &item,
                                                                     file,
                                                                     &collection_ids);
                        pdf_page_count(&path).unwrap_or(0)
                    }
                    else {
                        0
                    };
                    (file.name.to_string(), count)
                })
                .collect();
        let written = self.index
                          .lock()
                          .unwrap_or_else(PoisonError::into_inner)
                          .set_file_pages(item_id, &pages);
        if let Err(error) = written {
            tracing::error!(%error, "failed to record page counts in the library index");
        }
    }

    /// Records page counts for downloaded items that have none, which are those
    /// downloaded before page counts were recorded. Returns whether any were
    /// recorded, so the caller knows the totals changed.
    ///
    /// Opens the files, so it belongs on a background thread.
    #[track_caller]
    pub fn record_missing_page_counts(&self, storage: &StorageConfig) -> bool {
        let missing = self.index
                          .lock()
                          .unwrap_or_else(PoisonError::into_inner)
                          .downloaded_items_without_pages()
                          .map_err(|error| tracing::warn!(%error, "library index query failed"))
                          .unwrap_or_default();
        for item_id in &missing {
            self.record_page_counts(storage, item_id);
        }
        !missing.is_empty()
    }
}

#[cfg(test)]
mod tests;
