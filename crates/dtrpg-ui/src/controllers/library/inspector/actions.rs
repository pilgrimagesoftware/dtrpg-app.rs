//! Inspector work that reads or changes files and summaries: the lazily loaded
//! summary tab, "Verify on disk", and "Materialize".

use dtrpg_core::platform::{MaterializeWait, Platform, materialize_and_wait};

use super::super::*;

impl LibraryController {
    /// The summary tab's state for `entry_id`. An entry whose summary has not
    /// been asked for yet reads as loading.
    #[must_use]
    pub fn inspector_summary_for(&self, entry_id: &str) -> InspectorSummaryState {
        self.inspector_summaries
            .get(entry_id)
            .cloned()
            .unwrap_or(InspectorSummaryState::Loading)
    }

    /// Reads the stored summary of `entry_id` off the render path.
    ///
    /// Called whenever the summary tab opens, so a summary written since (by
    /// the MCP server, for example) shows up. A summary already shown stays on
    /// screen while it is read again.
    pub fn load_inspector_summary(&mut self, entry_id: Arc<str>, cx: &mut Context<Self>) {
        self.inspector_summaries
            .entry(Arc::clone(&entry_id))
            .or_insert(InspectorSummaryState::Loading);
        cx.emit(LibraryChanged);
        let library = Arc::clone(&self.library);
        cx.spawn(async move |this, async_cx| {
              let read_id = Arc::clone(&entry_id);
              let summary = async_cx.background_executor()
                                    .spawn(async move { library.summary(&read_id) })
                                    .await;
              this.update(async_cx, |controller, cx| {
                      let state = summary.map_or(InspectorSummaryState::Missing,
                                                 InspectorSummaryState::Loaded);
                      controller.inspector_summaries.insert(entry_id, state);
                      cx.emit(LibraryChanged);
                  })
                  .ok();
          })
          .detach();
    }

    /// Checks `entry_id`'s files against disk now ("Verify on disk"), applying
    /// any difference found.
    pub fn verify_item_on_disk(&mut self, entry_id: &str, cx: &mut Context<Self>) {
        self.verify_selected_item_download(Arc::from(entry_id), cx);
    }

    /// Asks the platform to bring back the local bytes of every file of
    /// `entry_id` that it has offloaded to the cloud ("Materialize"), then
    /// checks the entry against disk so its flags follow.
    pub fn materialize_item(&mut self, entry_id: &str, cx: &mut Context<Self>) {
        let Some(item) = self.item_by_id(entry_id)
        else {
            return;
        };
        let storage = StorageConfig::load();
        let collection_ids = collection_ids_for_item(&item, &self.library.collections());
        let paths: Vec<PathBuf> =
            item.files
                .iter()
                .filter(|file| file.offloaded)
                .map(|file| {
                    resolved_file_path_in_collections(&storage, &item, file, &collection_ids)
                })
                .collect();
        if paths.is_empty() {
            return;
        }
        let id = Arc::clone(&item.id);
        self.library.begin_verifying([Arc::clone(&id)]);
        cx.emit(LibraryChanged);
        cx.spawn(async move |this, async_cx| {
              async_cx.background_executor()
                      .spawn(async move {
                          for path in &paths {
                              if let Err(error) =
                                  materialize_and_wait(&Platform, path, MaterializeWait::default())
                              {
                                  tracing::warn!(path = %path.display(), %error,
                                                 "could not materialize the file");
                              }
                          }
                      })
                      .await;
              this.update(async_cx, |controller, cx| {
                      controller.library.end_verifying([&id]);
                      controller.verify_selected_item_download(id, cx);
                  })
                  .ok();
          })
          .detach();
    }
}
