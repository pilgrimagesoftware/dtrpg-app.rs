//! The Needs Attention section: the items whose last download failed.

use super::*;

impl LibraryController {
    /// Records how a download of `item_id` ended: a failure puts the item in
    /// Needs Attention, a success takes it out. The section on show is
    /// brought up to date.
    pub(super) fn note_download_result(&mut self, item_id: &Arc<str>, succeeded: bool) {
        if succeeded {
            self.failed_downloads.remove(item_id);
        }
        else {
            self.failed_downloads.insert(Arc::clone(item_id));
        }
        self.refresh_attention_members();
    }

    /// Recomputes the product ids held by Needs Attention, if it is on show.
    pub(super) fn refresh_attention_members(&mut self) {
        if self.filter != SidebarFilter::NeedsAttention {
            return;
        }
        self.collection_members = self.failed_downloads
                                      .iter()
                                      .filter_map(|id| self.library.item_by_id(id))
                                      .flat_map(|item| [item.order_product_id, item.product_id])
                                      .filter(|id| *id > 0)
                                      .collect();
    }
}

impl LibraryController {
    /// Recounts each collection's titles that are only in the cloud. Reads the
    /// whole catalog, so it runs when the catalog, the collections or the
    /// downloads change, not while drawing.
    pub(super) fn refresh_collection_cloud(&mut self) {
        self.collection_cloud = Arc::new(self.library.collection_cloud_counts());
    }
}
