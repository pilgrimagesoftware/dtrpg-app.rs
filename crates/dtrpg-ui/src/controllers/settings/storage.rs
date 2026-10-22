//! Storage configuration: download roots, placement rules, the storage
//! path, concurrency, and the regenerable on-disk caches.

use super::*;

impl SettingsController {
    /// Returns the resolved storage root path.
    pub fn storage_root_path(&self) -> PathBuf {
        self.storage.root_path()
    }

    /// Returns all configured download roots, with the default root first.
    pub fn storage_roots(&self) -> Vec<StorageRoot> {
        self.storage.roots()
    }

    /// Adds a named download root after validating that it is writable.
    pub fn add_storage_root(&mut self, name: String, path: PathBuf, cx: &mut Context<Self>)
                            -> Result<u32, StorageError> {
        let id = self.storage.add_root(name, path)?;
        self.refresh_root_volumes();
        cx.emit(SettingsChanged);
        Ok(id)
    }

    /// Removes a non-default download root. The default root is never removed.
    pub fn remove_storage_root(&mut self, id: u32, cx: &mut Context<Self>) {
        self.storage.remove_root(id);
        cx.emit(SettingsChanged);
    }

    /// Removes a placement rule by its ordered index.
    pub fn remove_storage_rule(&mut self, index: usize, cx: &mut Context<Self>) {
        self.storage.remove_rule(index);
        cx.emit(SettingsChanged);
    }

    /// Moves a placement rule earlier or later in evaluation order.
    pub fn move_storage_rule(&mut self, index: usize, down: bool, cx: &mut Context<Self>) {
        if down {
            self.storage.move_rule_down(index);
        }
        else {
            self.storage.move_rule_up(index);
        }
        cx.emit(SettingsChanged);
    }

    /// Reads the storage settings from disk again, for when something else (the
    /// download sheet's "Remember this choice") has changed them. Without it
    /// this controller would show the old rules, and the next setting changed
    /// here would save over the new ones.
    pub fn reload_storage(&mut self, cx: &mut Context<Self>) {
        self.storage = StorageConfig::load();
        self.refresh_root_volumes();
        cx.emit(SettingsChanged);
        cx.notify();
    }

    /// Validates `path` for writability and saves it as the new storage root.
    ///
    /// Emits [`SettingsChanged`] on success. Returns the validation error on
    /// failure so the caller can surface it to the user.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError`] if `path` is missing, unwritable, or on an
    /// unavailable volume.
    pub fn apply_storage_path(&mut self, path: PathBuf, cx: &mut Context<Self>)
                              -> Result<(), StorageError> {
        validate_writable(&path)?;
        self.storage.set_root_path(path.clone());
        self.storage_unavailable = false;
        self.check_storage_path_exists(path, cx);
        cx.emit(SettingsChanged);
        Ok(())
    }

    /// Saves `n` as the new shared thumbnail/download concurrency limit.
    ///
    /// Emits [`SettingsChanged`] so the library controller picks up the new
    /// limit on its next drain.
    pub fn set_max_concurrent_downloads(&mut self, n: usize, cx: &mut Context<Self>) {
        self.storage.set_max_concurrent_downloads(n);
        cx.emit(SettingsChanged);
    }

    /// Saves `enabled` as the new "Create collections" storage setting.
    ///
    /// Emits [`SettingsChanged`] so the library controller picks up the new
    /// setting on its next download completion.
    pub fn set_create_collections(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.storage.set_create_collections(enabled);
        cx.emit(SettingsChanged);
    }

    /// Saves `enabled` as the new "Check free space" storage setting.
    pub fn set_check_free_space(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.storage.set_check_free_space(enabled);
        cx.emit(SettingsChanged);
    }

    /// Saves `days` (clamped to `[RECENTLY_UPDATED_WINDOW_MIN_DAYS,
    /// RECENTLY_UPDATED_WINDOW_MAX_DAYS]`) as the new "Recently Updated
    /// window", returning the clamped value actually stored.
    ///
    /// Emits [`SettingsChanged`] so the library controller picks up the new
    /// window immediately. Callers driving an editable number field (see
    /// `root_view.rs`) use the returned value to write the clamped result
    /// back into the field's own displayed text, since the field's typed
    /// text isn't otherwise corrected when a value outside the bounds is
    /// committed before the field loses focus.
    pub fn set_recently_updated_window_days(&mut self, days: u32, cx: &mut Context<Self>) -> u32 {
        self.storage.set_recently_updated_window_days(days);
        cx.emit(SettingsChanged);
        self.storage.recently_updated_window_days()
    }

    /// Spawns a background task to check whether `path` exists on disk.
    ///
    /// Writes the result back to `storage_path_exists` and notifies the UI.
    pub(super) fn check_storage_path_exists(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        cx.spawn(async move |this, async_cx| {
              let exists = async_cx.background_executor()
                                   .spawn(async move { path.exists() })
                                   .await;
              this.update(async_cx, |ctrl, cx| {
                      ctrl.storage_path_exists = exists;
                      cx.notify();
                  })
                  .ok();
          })
          .detach();
    }

    /// Opens the OS file manager at the configured storage root, creating the
    /// directory first if it does not exist.
    pub fn reveal_storage_location(&self) {
        let path = self.storage.root_path();
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        if let Err(e) = crate::util::reveal::reveal_in_file_manager(&path) {
            tracing::warn!("reveal_in_file_manager failed: {e}");
        }
    }

    /// Deletes all regenerable app cache data (catalog/collections metadata
    /// cache and the cached avatar image) from disk, then emits
    /// [`CacheCleared`] so the library view drops its in-memory catalog and
    /// re-fetches live.
    ///
    /// Does not touch downloaded content, credentials, or preferences.
    pub fn clear_cache(&mut self, cx: &mut Context<Self>) {
        let dir = app_cache_dir();
        if let Err(e) = std::fs::remove_dir_all(&dir)
           && e.kind() != std::io::ErrorKind::NotFound
        {
            tracing::warn!("clear cache: failed to remove {}: {e}", dir.display());
        }
        self.refresh_page_data();
        cx.emit(CacheCleared);
    }

    /// Refreshes what the pages read from disk: the free space of each storage
    /// root's volume. Runs when the panel opens, the page changes, and the
    /// cache is cleared, never while drawing.
    pub fn refresh_page_data(&mut self) {
        self.refresh_root_volumes();
    }

    /// Reads the free and total space of each root's volume.
    pub fn refresh_root_volumes(&mut self) {
        self.root_volumes =
            self.storage
                .roots()
                .iter()
                .map(|root| (root.id, dtrpg_core::data::storage::roots::volume_space_mb(root)))
                .collect();
    }

    /// The free and total space (in megabytes) of the volume of each root, by
    /// root id, as of the last refresh.
    #[must_use]
    pub fn root_volumes(&self) -> &HashMap<u32, (Option<f64>, Option<f64>)> {
        &self.root_volumes
    }

    /// The id of the root downloads go to when no placement rule matches.
    #[must_use]
    pub fn default_root_id(&self) -> u32 {
        self.storage.default_root().id
    }

    /// Chooses the root downloads go to when no placement rule matches.
    pub fn set_default_root(&mut self, id: u32, cx: &mut Context<Self>) {
        self.storage.set_default_root(id);
        cx.emit(SettingsChanged);
    }

    /// Opens the OS file manager at the app cache directory, creating it
    /// first if it does not exist (a fresh install may have no cache yet).
    pub fn open_cache_folder(&self) {
        let dir = app_cache_dir();
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        if let Err(e) = crate::util::reveal::reveal_in_file_manager(&dir) {
            tracing::warn!("reveal_in_file_manager failed: {e}");
        }
    }
}
