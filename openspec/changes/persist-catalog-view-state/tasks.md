## 1. Shared Config File Access

- [x] 1.1 Make `config_path()` in `dtrpg-ui/src/data/file_openers.rs` `pub(crate)` so other data modules can use it without duplication
- [x] 1.2 Add a `pub(crate) fn load_app_config() -> AppConfigFile` and `pub(crate) fn save_app_config(cfg: &AppConfigFile)` pair to replace the current duplicated read-modify-write logic in `FileOpenerConfig::load*` and `FileOpenerConfig::save`; refactor those methods to call through the shared helpers

## 2. CatalogViewPrefs Struct

- [x] 2.1 Add a `CatalogViewPrefs` struct to `dtrpg-ui/src/data/file_openers.rs` (or a new `dtrpg-ui/src/data/catalog_view_prefs.rs`) with serde-serializable fields: `sort: Option<String>`, `grouped: Option<bool>`, `presentation: Option<String>`, `filter: Option<String>`, `filter_publisher: Option<String>`; derive `Serialize`, `Deserialize`, `Default`
- [x] 2.2 Embed `CatalogViewPrefs` in `AppConfigFile` as `#[serde(default)] catalog_view: CatalogViewPrefs` so it round-trips as a `[catalog_view]` TOML section
- [x] 2.3 Implement `CatalogViewPrefs::to_sort() -> SortMethod`: parse `self.sort` string to `SortMethod` variant; return `SortMethod::default()` and emit `tracing::warn!` for unrecognized values
- [x] 2.4 Implement `CatalogViewPrefs::to_presentation() -> CatalogPresentation`: same pattern as `to_sort()`
- [x] 2.5 Implement `CatalogViewPrefs::to_filter() -> SidebarFilter`: when `filter` is `"Publisher"`, return `SidebarFilter::Publisher(Arc::from(name))` using `filter_publisher`; fall back to `AllTitles` if `filter_publisher` is `None`; return `AllTitles` for unrecognized values with a `tracing::warn!`
- [x] 2.6 Implement `CatalogViewPrefs::from_state(filter: &SidebarFilter, sort: SortMethod, grouped: bool, presentation: CatalogPresentation) -> Self`: encode each value to its string representation; set `filter_publisher` from `Publisher(name)` or `None`
- [x] 2.7 Implement `CatalogViewPrefs::load() -> Self`: call `load_app_config()` and return `cfg.catalog_view`
- [x] 2.8 Implement `CatalogViewPrefs::save(&self)`: call `load_app_config()`, replace `cfg.catalog_view = self.clone()`, call `save_app_config(&cfg)`; log `tracing::warn!` on any I/O error

## 3. LibraryController Init from Prefs

- [x] 3.1 In `LibraryController::new()`, call `CatalogViewPrefs::load()` and use `prefs.to_sort()`, `prefs.to_presentation()`, `prefs.grouped.unwrap_or(false)`, and `prefs.to_filter()` as the initial field values instead of hardcoded defaults
- [x] 3.2 Add `pub fn validate_publisher_filter(&mut self, known_publishers: &[Arc<str>])` to `LibraryController`: if `self.filter` is `Publisher(name)` and `name` is not in `known_publishers`, set `self.filter = SidebarFilter::AllTitles` (do not save; do not emit)
- [x] 3.3 Call `validate_publisher_filter` in the catalog-load success path in `LibraryController`, after the item list is populated and publishers are known

## 4. Save on Mutation

- [x] 4.1 At the end of `LibraryController::set_filter()`, call `CatalogViewPrefs::from_state(&self.filter, self.sort, self.grouped, self.presentation).save()`
- [x] 4.2 At the end of `LibraryController::set_sort()`, call the same save helper
- [x] 4.3 At the end of `LibraryController::set_grouped()`, call the same save helper
- [x] 4.4 At the end of `LibraryController::set_presentation()`, call the same save helper

## 5. Verification

- [x] 5.1 Run `cargo check -p dtrpg-ui` and confirm zero errors
- [x] 5.2 Run `cargo test -p dtrpg-ui` and confirm all existing tests pass
- [x] 5.3 Run the app; change sort to Publisher, enable grouping, switch to List view; quit and relaunch; confirm all three preferences are restored
- [x] 5.4 Select a publisher filter; quit and relaunch; confirm the publisher filter is active
- [x] 5.5 Manually edit `app_config.toml` to set `sort = "Bogus"`; relaunch; confirm the app opens with Title sort and a WARN in the log
- [x] 5.6 Confirm the search field is always empty on launch regardless of prior searches
