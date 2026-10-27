//! Writing a summary of an item when its download finishes, if the person has
//! asked for that in Settings > Agent & MCP.

use dtrpg_core::api::{LibraryApi, SummarizeError, provider_for};
use dtrpg_core::data::paths::app_data_dir;
use dtrpg_core::data::summary::ContentSummary;
use dtrpg_core::data::summary_settings::{SummarySettings, SummarySettingsFile};
use dtrpg_core::services::sdk::{SdkCollectionsService, SdkLibraryService};

use super::*;

/// Summarizes the item `item_id`: builds the provider the settings choose,
/// reads the text of one of the item's files and saves what the provider
/// writes. Reads files and may call the provider, so it runs off the UI thread.
fn summarize(item_id: &str, settings: &SummarySettingsFile)
             -> Result<ContentSummary, SummarizeError> {
    let provider = provider_for(settings)?;
    let api = LibraryApi::new(Box::new(SdkLibraryService::unauthenticated()),
                              Box::new(SdkCollectionsService::unauthenticated()),
                              app_data_dir());
    api.summarize_item(item_id, &StorageConfig::load(), settings, &*provider)
}

impl LibraryController {
    /// Starts writing a summary of `item_id` if the person asked for summaries
    /// on download, the item is fully downloaded, and it has no summary yet.
    ///
    /// The work runs in the background. A summary that cannot be made (no API
    /// key, the monthly ceiling reached, no file with text) is logged, not
    /// shown: nothing was asked for at that moment.
    pub(super) fn summarize_after_download(&mut self, item_id: &Arc<str>, cx: &mut Context<Self>) {
        let settings = SummarySettings::load().snapshot();
        if !settings.auto_summarize_on_download {
            return;
        }
        let downloaded = self.item_by_id(item_id)
                             .is_some_and(|item| item.status == ItemStatus::Downloaded);
        if !downloaded || self.library.summary(item_id).is_some() {
            return;
        }
        let item_id = Arc::clone(item_id);
        cx.spawn(async move |this, async_cx| {
              let work_id = Arc::clone(&item_id);
              let result = async_cx.background_executor()
                                   .spawn(async move { summarize(&work_id, &settings) })
                                   .await;
              this.update(async_cx, |controller, cx| match result {
                  Ok(_) => {
                      // Show the new summary the next time the tab is opened.
                      controller.inspector_summaries.remove(&item_id);
                      cx.emit(LibraryChanged);
                  }
                  Err(error) => {
                      tracing::warn!(%error, item_id = %item_id, "automatic summary was not made");
                  }
              })
              .ok();
          })
          .detach();
    }
}
