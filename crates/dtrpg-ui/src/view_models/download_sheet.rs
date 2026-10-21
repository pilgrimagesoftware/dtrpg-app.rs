//! State of the download sheet: which files of an item to fetch and which
//! storage root to put them in.
//!
//! Pure data and rules, no I/O: the caller reads free space and passes it in,
//! so the sheet's behavior can be tested without a disk or a window.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use dtrpg_core::data::library::LibraryItem;
use dtrpg_core::data::storage::StorageConfig;

/// One file of the item, as the sheet lists it.
#[derive(Debug, Clone, PartialEq)]
pub struct SheetFile {
    /// The file's position in the item's file list.
    pub index:      u32,
    /// The file's name.
    pub name:       Arc<str>,
    /// The file's format, for example `PDF`.
    pub format:     Arc<str>,
    /// The file's size in megabytes.
    pub size_mb:    f64,
    /// Whether the file is already on this machine.
    pub downloaded: bool,
}

/// One storage root the files can go to.
#[derive(Debug, Clone, PartialEq)]
pub struct SheetRoot {
    /// The root's id.
    pub id:      u32,
    /// The root's display name.
    pub name:    String,
    /// The root's directory, for display.
    pub path:    String,
    /// Free space on the root's volume in megabytes, if it could be read.
    pub free_mb: Option<f64>,
}

/// What the user chose, for the controller to carry out.
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadPlan {
    /// The files to download, by position in the item's file list.
    pub file_indexes:     Vec<u32>,
    /// The root to send all of the publisher's titles to from now on, if the
    /// choice is to be remembered.
    pub remember_root_id: Option<u32>,
}

/// Not enough room on the chosen root for the chosen files.
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceShortfall {
    /// The root's display name.
    pub root_name: String,
    /// Megabytes the chosen files need.
    pub needed_mb: f64,
    /// Megabytes free on the root.
    pub free_mb:   f64,
}

/// The download sheet for one item.
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadSheet {
    /// The item's id.
    pub item_id:    Arc<str>,
    /// The item's title.
    pub title:      Arc<str>,
    /// The item's publisher, for the "remember" choice.
    pub publisher:  Arc<str>,
    /// Every file of the item.
    pub files:      Vec<SheetFile>,
    /// The roots the files can go to.
    pub roots:      Vec<SheetRoot>,
    selected:       BTreeSet<u32>,
    /// The root the placement rules choose for this item.
    rule_root_id:   u32,
    chosen_root_id: u32,
    remember:       bool,
}

impl DownloadSheet {
    /// Builds the sheet for `item`: every file not yet downloaded starts
    /// selected, and the root the placement rules choose starts chosen.
    ///
    /// `free_mb` reads the free space on a root's directory.
    #[must_use]
    pub fn new(item: &LibraryItem, storage: &StorageConfig, collection_ids: &[u64],
               free_mb: impl Fn(&Path) -> Option<f64>)
               -> Self {
        let files: Vec<SheetFile> =
            item.files
                .iter()
                .enumerate()
                .map(|(position, file)| SheetFile { index:
                                                        u32::try_from(position).unwrap_or(u32::MAX),
                                                    name:       Arc::clone(&file.name),
                                                    format:     Arc::clone(&file.format),
                                                    size_mb:    file.size_mb,
                                                    downloaded: file.downloaded, })
                .collect();
        let selected = files.iter()
                            .filter(|file| !file.downloaded)
                            .map(|file| file.index)
                            .collect();
        let roots = storage.roots()
                           .into_iter()
                           .map(|root| SheetRoot { id:      root.id,
                                                   name:    root.name,
                                                   free_mb: free_mb(&root.path),
                                                   path:    root.path.display().to_string(), })
                           .collect();
        let rule_root_id = storage.resolve_root_for(&item.publisher, collection_ids, item.size_mb)
                                  .id;
        Self { item_id: Arc::clone(&item.id),
               title: Arc::clone(&item.title),
               publisher: Arc::clone(&item.publisher),
               files,
               roots,
               selected,
               rule_root_id,
               chosen_root_id: rule_root_id,
               remember: false }
    }

    /// Whether file `index` is selected.
    #[must_use]
    pub fn is_selected(&self, index: u32) -> bool {
        self.selected.contains(&index)
    }

    /// Flips file `index`. A file that is already downloaded cannot be
    /// selected.
    pub fn toggle_file(&mut self, index: u32) {
        if self.files
               .iter()
               .any(|file| file.index == index && !file.downloaded)
           && !self.selected.remove(&index)
        {
            self.selected.insert(index);
        }
    }

    /// Selects every file that is not downloaded yet.
    pub fn select_all(&mut self) {
        self.selected = self.files
                            .iter()
                            .filter(|file| !file.downloaded)
                            .map(|file| file.index)
                            .collect();
    }

    /// Selects no file.
    pub fn select_none(&mut self) {
        self.selected.clear();
    }

    /// How many files are selected.
    #[must_use]
    pub fn selected_count(&self) -> usize {
        self.selected.len()
    }

    /// The combined size of the selected files in megabytes.
    #[must_use]
    pub fn selected_size_mb(&self) -> f64 {
        self.files
            .iter()
            .filter(|file| self.selected.contains(&file.index))
            .map(|file| file.size_mb)
            .sum()
    }

    /// The id of the root the files will go to.
    #[must_use]
    pub fn chosen_root_id(&self) -> u32 {
        self.chosen_root_id
    }

    /// Chooses the root for the files.
    ///
    /// Everywhere else in the app finds a downloaded file through the
    /// placement rules, so a root other than the one the rules choose only
    /// works if the choice becomes a rule. Choosing one therefore turns
    /// "remember" on and locks it; choosing the rules' own root unlocks it.
    pub fn choose_root(&mut self, root_id: u32) {
        if self.roots.iter().all(|root| root.id != root_id) {
            return;
        }
        self.chosen_root_id = root_id;
        if self.remember_is_locked() {
            self.remember = true;
        }
    }

    /// Whether the choice will be remembered as a rule for the publisher.
    #[must_use]
    pub fn remember(&self) -> bool {
        self.remember
    }

    /// Whether "remember" is on and cannot be turned off, because the chosen
    /// root is not the one the placement rules already choose.
    #[must_use]
    pub fn remember_is_locked(&self) -> bool {
        self.chosen_root_id != self.rule_root_id
    }

    /// Turns "remember" on or off. Ignored while it is locked on.
    pub fn set_remember(&mut self, remember: bool) {
        if !self.remember_is_locked() {
            self.remember = remember;
        }
    }

    /// Whether there is anything to download.
    #[must_use]
    pub fn can_download(&self) -> bool {
        !self.selected.is_empty()
    }

    /// The chosen root's free space is less than the chosen files need.
    #[must_use]
    pub fn space_shortfall(&self) -> Option<SpaceShortfall> {
        let root = self.roots
                       .iter()
                       .find(|root| root.id == self.chosen_root_id)?;
        let free_mb = root.free_mb?;
        let needed_mb = self.selected_size_mb();
        (needed_mb > free_mb).then(|| SpaceShortfall { root_name: root.name.clone(),
                                                       needed_mb,
                                                       free_mb })
    }

    /// What to carry out when the user presses Download.
    #[must_use]
    pub fn plan(&self) -> DownloadPlan {
        DownloadPlan { file_indexes:     self.selected.iter().copied().collect(),
                       remember_root_id: self.remember.then_some(self.chosen_root_id), }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dtrpg_core::data::enums::ItemStatus;
    use dtrpg_core::data::library::LibraryItemFile;
    use dtrpg_core::data::storage::roots::StorageRoot;

    use super::*;

    fn file(name: &str, size_mb: f64, downloaded: bool) -> LibraryItemFile {
        LibraryItemFile { id: name.into(),
                          index: 0,
                          name: name.into(),
                          format: "PDF".into(),
                          size_mb,
                          downloaded,
                          offloaded: false }
    }

    fn item(files: Vec<LibraryItemFile>) -> LibraryItem {
        let mut item = LibraryItem::new("a",
                                        "Curse of Strahd",
                                        "Wizards of the Coast",
                                        "",
                                        "Adventure",
                                        "PDF",
                                        256,
                                        1.0,
                                        2016,
                                        0,
                                        ItemStatus::Cloud,
                                        "#1C2A44",
                                        "",
                                        None);
        item.files = files;
        item
    }

    fn storage() -> StorageConfig {
        StorageConfig::for_test(PathBuf::from("/tmp/local"))
            .with_extra_roots(vec![StorageRoot { id:   1,
                                                 name: "Vault".to_owned(),
                                                 path: PathBuf::from("/tmp/vault"), }])
    }

    fn free(path: &Path) -> Option<f64> {
        if path.ends_with("vault") {
            Some(100.0)
        }
        else {
            Some(1000.0)
        }
    }

    fn sheet() -> DownloadSheet {
        DownloadSheet::new(&item(vec![file("book.pdf", 150.0, false),
                                      file("maps.zip", 20.0, false),
                                      file("done.pdf", 5.0, true),]),
                           &storage(),
                           &[],
                           free)
    }

    #[test]
    fn files_not_yet_downloaded_start_selected() {
        let sheet = sheet();

        assert!(sheet.is_selected(0));
        assert!(sheet.is_selected(1));
        assert!(!sheet.is_selected(2));
        assert_eq!(sheet.selected_count(), 2);
        assert!((sheet.selected_size_mb() - 170.0).abs() < f64::EPSILON);
    }

    #[test]
    fn a_downloaded_file_cannot_be_selected() {
        let mut sheet = sheet();

        sheet.toggle_file(2);

        assert!(!sheet.is_selected(2));
    }

    #[test]
    fn toggling_flips_a_file() {
        let mut sheet = sheet();

        sheet.toggle_file(0);
        assert!(!sheet.is_selected(0));
        sheet.toggle_file(0);
        assert!(sheet.is_selected(0));
    }

    #[test]
    fn none_clears_and_all_selects_what_can_be_downloaded() {
        let mut sheet = sheet();

        sheet.select_none();
        assert!(!sheet.can_download());
        sheet.select_all();
        assert_eq!(sheet.plan().file_indexes, vec![0, 1]);
    }

    #[test]
    fn the_rules_root_starts_chosen_and_nothing_is_remembered() {
        let sheet = sheet();

        assert_eq!(sheet.chosen_root_id(), 0);
        assert!(!sheet.remember());
        assert!(!sheet.remember_is_locked());
        assert_eq!(sheet.plan().remember_root_id, None);
    }

    #[test]
    fn choosing_another_root_turns_remember_on_and_locks_it() {
        let mut sheet = sheet();

        sheet.choose_root(1);
        sheet.set_remember(false);

        assert!(sheet.remember());
        assert!(sheet.remember_is_locked());
        assert_eq!(sheet.plan().remember_root_id, Some(1));
    }

    #[test]
    fn going_back_to_the_rules_root_unlocks_remember() {
        let mut sheet = sheet();
        sheet.choose_root(1);

        sheet.choose_root(0);
        sheet.set_remember(false);

        assert!(!sheet.remember_is_locked());
        assert!(!sheet.remember());
    }

    #[test]
    fn remember_can_be_turned_on_for_the_rules_root() {
        let mut sheet = sheet();

        sheet.set_remember(true);

        assert_eq!(sheet.plan().remember_root_id, Some(0));
    }

    #[test]
    fn an_unknown_root_is_ignored() {
        let mut sheet = sheet();

        sheet.choose_root(99);

        assert_eq!(sheet.chosen_root_id(), 0);
    }

    #[test]
    fn a_shortfall_names_the_chosen_root() {
        let mut sheet = sheet();
        assert_eq!(sheet.space_shortfall(), None);

        sheet.choose_root(1);

        assert_eq!(sheet.space_shortfall(),
                   Some(SpaceShortfall { root_name: "Vault".to_owned(),
                                         needed_mb: 170.0,
                                         free_mb:   100.0, }));
    }

    #[test]
    fn unknown_free_space_is_not_a_shortfall() {
        let sheet = DownloadSheet::new(&item(vec![file("book.pdf", 150.0, false)]),
                                       &storage(),
                                       &[],
                                       |_| None);

        assert_eq!(sheet.space_shortfall(), None);
    }
}
