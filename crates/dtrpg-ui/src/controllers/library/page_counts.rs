//! Counting the pages of the downloaded files, for the footer's total and the
//! Pages column.

use super::*;

impl LibraryController {
    /// Reads the page count of every downloaded file that has none recorded,
    /// on a background thread, then refreshes the totals and redraws if any
    /// were recorded. Reading a PDF takes long enough to stall a frame, so
    /// nothing here runs on the UI thread but the refresh when it lands.
    pub(super) fn count_pages(&self, cx: &mut Context<Self>) {
        let library = Arc::clone(&self.library);
        cx.spawn(async move |this, cx| {
              let recorded = cx.background_executor()
                               .spawn(async move {
                                   library.record_missing_page_counts(&StorageConfig::load())
                               })
                               .await;
              if recorded {
                  this.update(cx, |ctrl, cx| {
                          ctrl.refresh_section_counts();
                          ctrl.invalidate_cache();
                          cx.emit(LibraryChanged);
                      })
                      .ok();
              }
          })
          .detach();
    }
}
