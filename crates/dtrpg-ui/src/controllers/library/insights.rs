//! The Insights page: working out the library's aggregates off the main thread
//! and exporting them.

use std::path::PathBuf;

use dtrpg_core::data::cache_counts::CacheCounts;
use dtrpg_core::data::storage::StorageConfig;
use dtrpg_core::library::insights::{Insights, RootUsage, root_usages};

use super::*;

/// What the Insights page shows besides the catalog's aggregates: titles in the
/// middle of a download, and titles whose last download failed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LiveAvailability {
    /// Titles being downloaded now.
    pub downloading: usize,
    /// Titles whose last download failed.
    pub failed:      usize,
}

impl LibraryController {
    /// The aggregates, or `None` until they have first been worked out.
    #[must_use]
    pub fn insights(&self) -> Option<Arc<Insights>> {
        self.insights.clone()
    }

    /// What the caches hold, as of the last time the aggregates were worked
    /// out.
    #[must_use]
    pub const fn cache_counts(&self) -> CacheCounts {
        self.cache_counts
    }

    /// True while the aggregates are being worked out.
    #[must_use]
    pub const fn is_insights_computing(&self) -> bool {
        self.insights_computing
    }

    /// Titles being downloaded and titles that failed, as they are now.
    #[must_use]
    pub fn live_availability(&self) -> LiveAvailability {
        LiveAvailability { downloading: self.library.downloading_item_ids().len(),
                           failed:      self.failed_downloads.len(), }
    }

    /// Works the aggregates out again from the catalog and the storage
    /// locations. The work runs on a background thread, so the catalog stays
    /// usable, and the page keeps showing the old figures until the new ones
    /// arrive.
    pub fn recompute_insights(&mut self, cx: &mut Context<Self>) {
        if self.insights_computing {
            return;
        }
        self.insights_computing = true;
        let library = Arc::clone(&self.library);
        let roots = StorageConfig::load().roots();
        cx.emit(LibraryChanged);
        cx.spawn(async move |this, async_cx| {
              let (insights, cache_counts) =
                  async_cx.background_executor()
                          .spawn(async move {
                              let insights = Insights::compute(&library.all_items_scan(),
                                                               now_secs()).with_roots(&roots);
                              (insights, CacheCounts::read())
                          })
                          .await;
              this.update(async_cx, |ctrl, cx| {
                      ctrl.insights = Some(Arc::new(insights));
                      ctrl.cache_counts = cache_counts;
                      ctrl.insights_computing = false;
                      cx.emit(LibraryChanged);
                  })
                  .ok();
          })
          .detach();
    }

    /// Asks where to save the aggregates and writes them there as CSV. The
    /// export shows in the activity list.
    pub fn export_insights_csv(&mut self, cx: &mut Context<Self>) {
        let Some(insights) = self.insights()
        else {
            return;
        };
        let label = t!("activity.exporting_insights").to_string();
        let activity_id = self.activity.update(cx, |a, cx| a.start(&label, None, cx));
        cx.spawn(async move |this, async_cx| {
              let Some(handle) = rfd::AsyncFileDialog::new().set_file_name("libri-insights.csv")
                                                            .save_file()
                                                            .await
              else {
                  this.update(async_cx, |ctrl, cx| {
                          ctrl.activity
                              .update(cx, |a, cx| a.complete(activity_id, cx));
                      })
                      .ok();
                  return;
              };
              let path: PathBuf = handle.path().to_path_buf();
              let written = async_cx.background_executor()
                                    .spawn(async move { std::fs::write(path, insights.to_csv()) })
                                    .await;
              this.update(async_cx, |ctrl, cx| {
                      ctrl.activity.update(cx, |a, cx| match written {
                                       Ok(()) => a.complete(activity_id, cx),
                                       Err(error) => a.error(activity_id, error.to_string(), cx),
                                   });
                  })
                  .ok();
          })
          .detach();
    }
}

impl LibraryController {
    /// What Libri uses on each storage location, as of the last refresh.
    #[must_use]
    pub fn root_usage(&self) -> Arc<Vec<RootUsage>> {
        Arc::clone(&self.root_usage)
    }

    /// Measures each storage location off the main thread, for the status bar.
    /// Walking the files takes a while on a large library, so it runs at
    /// startup, when the locations change, and when the downloads finish, not
    /// while drawing.
    pub fn refresh_root_usage(&mut self, cx: &mut Context<Self>) {
        let roots = StorageConfig::load().roots();
        cx.spawn(async move |this, async_cx| {
              let usage = async_cx.background_executor()
                                  .spawn(async move { root_usages(&roots) })
                                  .await;
              this.update(async_cx, |ctrl, cx| {
                      ctrl.root_usage = Arc::new(usage);
                      cx.emit(LibraryChanged);
                  })
                  .ok();
          })
          .detach();
    }
}
