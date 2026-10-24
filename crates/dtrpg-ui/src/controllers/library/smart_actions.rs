//! What a smart collection does when items match it: downloads the ones that
//! start to match, and keeps a folder of symlinks to its members.

use dtrpg_core::data::smart_collection::{
    SmartCollection, clear_mirror, smart_collection_dir, sync_mirror,
};

use super::file_tags::files_on_disk;
use super::*;

impl LibraryController {
    /// Runs the on-match actions of every smart collection that has one on:
    /// queues the downloads of the items that started to match since the last
    /// run, and brings the symlink folders up to date.
    ///
    /// Call it when the catalog, the tags or the collections have settled, and
    /// when a file finishes downloading. It reads the whole catalog once per
    /// collection with an action on. A collection's first run only records
    /// what it matches, so turning an action on does not download what the
    /// collection already held.
    // ponytail: matches are remembered for this run of the app only; an item
    // that starts to match while the app is closed is not downloaded. Persist
    // the record if that turns out to matter.
    pub(super) fn run_smart_actions(&mut self, cx: &mut Context<Self>) {
        let collections = self.smart_collections.all().to_vec();
        let mut mirrors = Vec::new();
        for collection in &collections {
            if !collection.download_new_matches {
                self.match_tracker.forget(collection.id);
            }
            if !collection.download_new_matches && !collection.mirror_as_symlinks {
                continue;
            }
            let members = self.library.smart_collection_members(collection);
            if collection.download_new_matches {
                self.download_new_matches(collection.id, &members, cx);
            }
            if collection.mirror_as_symlinks {
                mirrors.push((collection.name.clone(), members));
            }
        }
        self.sync_mirrors(mirrors, cx);
    }

    /// Queues the downloads of the `members` that were not in the collection
    /// the last time it was looked at.
    fn download_new_matches(&mut self, id: u64, members: &[LibraryItem], cx: &mut Context<Self>) {
        let matching = members.iter().map(|item| Arc::clone(&item.id)).collect();
        for started in self.match_tracker.observe(id, matching) {
            let title = members.iter()
                               .find(|item| item.id == started)
                               .map(|item| item.title.to_string())
                               .unwrap_or_default();
            self.enqueue_download(&started, title, cx);
        }
    }

    /// Brings the symlink folder of each `(collection name, members)` up to
    /// date, in the background since it reads the disk.
    fn sync_mirrors(&self, mirrors: Vec<(String, Vec<LibraryItem>)>, cx: &mut Context<Self>) {
        if mirrors.is_empty() {
            return;
        }
        let storage = StorageConfig::load();
        let collections = self.library.collections();
        cx.background_executor()
          .spawn(async move {
              for (name, members) in mirrors {
                  let files: Vec<_> =
                      files_on_disk(&members, &collections, &storage).into_iter()
                                                                     .map(|file| file.path)
                                                                     .collect();
                  let dir = smart_collection_dir(&storage.root_path(), &name);
                  if let Err(error) = sync_mirror(&dir, &files) {
                      tracing::warn!(dir = %dir.display(), %error,
                                     "could not mirror the smart collection");
                  }
              }
          })
          .detach();
    }

    /// Takes down the symlink folder of `collection`, if it had one, in the
    /// background.
    pub(super) fn remove_mirror(&self, collection: &SmartCollection, cx: &mut Context<Self>) {
        if !collection.mirror_as_symlinks {
            return;
        }
        let dir = smart_collection_dir(&StorageConfig::load().root_path(), &collection.name);
        cx.background_executor()
          .spawn(async move { clear_mirror(&dir) })
          .detach();
    }
}
