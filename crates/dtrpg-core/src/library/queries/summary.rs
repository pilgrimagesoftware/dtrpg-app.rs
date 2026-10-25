//! The stored content summary of a catalog item.

use std::sync::PoisonError;

use crate::data::summary::ContentSummary;
use crate::library::Library;

impl Library {
    /// The saved content summary of item `item_id`, or `None` if it has none.
    ///
    /// Blocking: call it off the UI thread. A failed read is logged and
    /// reported as no summary, so a broken index shows an empty summary tab
    /// instead of an error.
    #[must_use]
    pub fn summary(&self, item_id: &str) -> Option<ContentSummary> {
        self.index
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get_summary(item_id)
            .inspect_err(|error| tracing::warn!(%error, item_id, "could not read the summary"))
            .ok()
            .flatten()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::data::enums::ItemStatus;
    use crate::data::library::LibraryItem;
    use crate::data::storage::StorageConfig;
    use crate::data::summary::ContentSummary;
    use crate::library::Library;

    fn open_library() -> Library {
        let datadir = tempfile::tempdir().expect("tempdatadir");
        let cachedir = tempfile::tempdir().expect("tempcachedir");
        Library::open(datadir.keep(),
                      cachedir.keep(),
                      &StorageConfig::for_test(std::env::temp_dir())).expect("open")
                                                                     .0
    }

    #[test]
    fn an_item_without_a_summary_has_none() {
        assert_eq!(open_library().summary("missing"), None);
    }

    #[test]
    fn a_saved_summary_is_read_back() {
        let library = open_library();
        let summary = ContentSummary { item_id:     "a".to_owned(),
                                       text:        "A heist.".to_owned(),
                                       model:       "local".to_owned(),
                                       created_at:  1_700_000_000,
                                       token_count: 2, };
        let item = LibraryItem::new("a",
                                    "Title",
                                    "Publisher",
                                    "",
                                    "Core",
                                    "PDF",
                                    10,
                                    1.0,
                                    2024,
                                    0,
                                    ItemStatus::Cloud,
                                    "#1C2A44",
                                    "",
                                    None);
        let mut index = library.index.lock().expect("lock");
        index.replace_item(&item).expect("save item");
        index.save_summary(&summary).expect("save summary");
        drop(index);

        assert_eq!(library.summary("a"), Some(summary));
    }
}
