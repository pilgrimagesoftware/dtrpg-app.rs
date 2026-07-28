## Context

The app persists settings to `~/.config/dtrpg/app_config.toml` (macOS/Linux) via an `AppConfigFile` struct in `dtrpg-ui/src/data/file_openers.rs`. That struct currently holds `file_openers: Vec<FileOpenerEntry>` and `active_settings_tab: Option<String>`, serialized with `serde` + `toml`.

`LibraryController` in `dtrpg-ui/src/controllers/library.rs` owns `filter: SidebarFilter`, `search_query: String`, `sort: SortMethod`, `grouped: bool`, and `presentation: CatalogPresentation`. All five are initialized from hardcoded defaults in `LibraryController::new()`. Mutation methods (`set_filter`, `set_sort`, `set_grouped`, `set_presentation`) update state and emit `LibraryChanged`.

## Goals / Non-Goals

**Goals:**

- Restore sidebar filter, sort, grouped, and presentation from disk on every launch.
- Write prefs to disk on every mutation of those four values.
- Fit into the existing TOML config file rather than introducing a second file.

**Non-Goals:**

- Persisting the search query.
- Migrating or versioning old config files — missing/unknown fields silently fall back to defaults.
- Syncing preferences across devices.

## Decisions

### Extend `AppConfigFile` with an inline `[catalog_view]` table

Add a `CatalogViewPrefs` serde struct and embed it in `AppConfigFile` as `#[serde(default)] catalog_view: CatalogViewPrefs`. TOML will round-trip this as a `[catalog_view]` section. This keeps all persistence in one file and one code path without introducing a new file or crate.

**Alternative considered**: new `catalog_view_prefs.rs` module with its own file path. Rejected because having two config files with different paths is harder to document and back up.

### `CatalogViewPrefs` stores string-encoded values

All four fields are stored as `Option<String>` (or `String` with a known default). Enum variants are serialized by name (e.g., `"Title"`, `"Publisher"`, `"Grid"`). Unknown strings fall back to the `Default` impl with a `tracing::warn!`. `SidebarFilter::Publisher` stores the publisher name in a separate `Option<String>` field (`filter_publisher`); on load, if `filter = "Publisher"` and `filter_publisher` is `Some`, the controller validates the name against the loaded library before applying it.

**Alternative considered**: `#[serde(rename_all = "snake_case")]` on the enums directly. Rejected because `SidebarFilter::Publisher(Arc<str>)` can't be trivially serialized with serde's derive without a custom impl, and the manual string approach is simpler and already consistent with how `active_settings_tab` is handled.

### Save is triggered inside each `LibraryController` mutation method

After updating state, each mutation method calls `CatalogViewPrefs::from_controller(self).save()`. This is synchronous and best-effort (errors logged, not propagated). The save path is the same `config_path()` helper already in `file_openers.rs`.

**Alternative considered**: observe mutations from the root view and save there. Rejected because it couples the view layer to persistence and requires threading the save path through render.

### `config_path()` is moved to a shared `data::config` module

Currently `config_path()` is private to `file_openers.rs`. Moving it (or re-exporting it) to a shared location avoids duplication between `FileOpenerConfig` and the new `CatalogViewPrefs`. A simple `pub(crate)` re-export inside the existing `data::file_openers` module is sufficient; no new file is strictly required, but extracting to `data::config` is cleaner if there are further additions.

### Publisher filter validation at controller init

`LibraryController::new()` is called before library items are loaded (the load is async). Therefore publisher validation cannot happen synchronously in `new()`. Instead, add a `validate_publisher_filter(&mut self, known_publishers: &[Arc<str>])` method called from the `load_catalog` success path, after the item list is available. If the current filter is `Publisher(name)` and `name` is not in `known_publishers`, reset to `AllTitles` (without emitting a save — the pref file is left unchanged so a future launch where that publisher reappears would restore it).

**Alternative considered**: validate in `new()` by loading items synchronously. Rejected — library load is async and blocking startup on it would regress cold-launch time.

## Risks / Trade-offs

- **Save on every mutation is synchronous I/O on the main thread**: The TOML file is small (<1 KB) and writes are infrequent (user gesture cadence). Acceptable on desktop. → Mitigation: if profiling ever shows jank, move to `cx.background_executor().spawn`.
- **Publisher filter left stale if publisher disappears after validation**: The filter resets in-session but the pref file still holds the old publisher name. On next launch the validation runs again and resets again — consistent behavior. The user may notice the filter keeps resetting; they can manually select a different filter to overwrite. No action needed now.
- **File contention between FileOpenerConfig.save and CatalogViewPrefs.save**: Both read-modify-write the same file. If both fire in the same frame they could clobber each other. Mitigation: implement a single `AppConfigFile::load() -> AppConfigFile` / `AppConfigFile::save()` round-trip used by both callers, merging all fields atomically. This replaces the current duplicated load/save pattern.

## Open Questions

*(none)*
