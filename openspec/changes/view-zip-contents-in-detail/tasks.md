## 1. Data model

- [x] 1.1 ~~Add `ItemFile { name: Arc<str>, size_mb: f64, is_zip: bool }`~~ — superseded:
      `LibraryItemFile { id, index, name, format, size_mb, downloaded }` already exists in
      `crates/dtrpg-ui/src/data/library.rs` (added by the `catalog-entry-detail-view`
      capability). `is_zip` isn't stored — `format` is already derived from the file
      extension (`file_extension_label` in `dtrpg-core`), so a Zip file's `format` is
      exactly `"ZIP"`; checked via `file.format.eq_ignore_ascii_case("zip")` at render/use
      time instead of a duplicated stored field.
- [x] 1.2 ~~Add `files: Vec<ItemFile>`~~ — superseded: `LibraryItem::files: Vec<LibraryItemFile>`
      already exists with `#[serde(default)]`.
- [x] 1.3 ~~Update stub catalog generation~~ — not applicable: `util::stubs::stub_catalog`
      and `services::stub::StubLibraryService` are both `#[cfg(test)]`-gated (unit-test
      fixtures only, exercised by `view_models::library::tests`) — there is no longer a
      runnable "stub backend" app mode to manually verify against (that assumption in the
      design predates the real `RustSdkLibraryService`/`HttpSdkLibraryGateway` backend now
      in place). No stub catalog changes needed for this change's scope.
- [x] 1.4 ~~Add a fixture Zip file under the stub storage root~~ — not applicable for the
      same reason as 1.3. Automated coverage of `list_entries` against a real fixture Zip is
      in `util::zip_preview`'s own tests (2.3); manual verification (5.1) needs a real
      downloaded Zip item via the live backend.

## 2. Zip reading

- [x] 2.1 Added `zip = "8.6.0"` (latest stable) to `crates/dtrpg-ui/Cargo.toml` with
      `default-features = false, features = ["deflate"]`.
- [x] 2.2 Added `crates/dtrpg-ui/src/util/zip_preview.rs` with `ZipPreviewError`
      (`thiserror::Error`: `NotFound`, `Io`, `InvalidArchive`) and
      `list_entries(path: &Path) -> Result<Vec<ZipEntry>, ZipPreviewError>` reading only the
      central directory via `by_index_raw`.
- [x] 2.3 Unit tests cover a valid fixture Zip, a missing file, an empty file, and a non-Zip
      file — all pass, none panic.

## 3. Detail tab file list rendering

- [x] 3.1 The per-file item tier table (`render_item_tier`/`ItemListDelegate`, added by
      `catalog-entry-detail-view`) already rendered one row per `LibraryItemFile` for
      multi-item entries; changed its gate in `render_detail_tab_content` from
      `item.is_multi_item()` to `!item.files.is_empty()` so a **single**-file entry (e.g. one
      whose one file is itself a Zip) also gets a row — this is the scenario the proposal's
      "Why" section actually describes. The existing entry-level download/open/reveal row is
      unchanged and still renders above it.
- [x] 3.2 Zip-typed rows (`file.format.eq_ignore_ascii_case("zip")`) get a `FolderOpen` icon
      next to the name and `cursor_pointer()`; non-Zip rows are unaffected.
- [x] 3.3 No changes needed — the existing entry-level download/open/reveal row
      (`reveal_in_file_manager`) was not removed, only the item tier's visibility gate
      changed (3.1).

## 4. Hover/pin state and popover

- [x] 4.1 Added a single `zip_preview: Option<ZipPreviewState>` field to `LibraryController`
      (struct of `entry_id`, `row_ix`, `anchor_pos`, `pinned`) instead of three separate
      `Option` fields — one struct keeps hovered/pinned/anchor as one atomic unit instead of
      three fields callers must keep in sync. Added `zip_preview_for`, `hover_zip_preview`,
      `clear_hover_zip_preview`, `toggle_zip_preview_pin`, `close_zip_preview`, all emitting
      `LibraryChanged`.
- [x] 4.2 Wired `on_mouse_move` (hover open/move, no debounce — see note below),
      `on_hover(false)` (hover end), and `on_click` (pin toggle) on the Zip-typed name cell in
      `ItemListDelegate::render_td`, calling the methods above. The popover itself calls
      `zip_preview::list_entries` at render time (4.3), keyed off the active preview's file
      path.
- [x] 4.3 Added `crates/dtrpg-ui/src/ui/views/zip_preview_popover.rs`: anchored/deferred
      popover mirroring `item_popover_view.rs`, a `max_h` + `overflow_y_scrollbar()` entry
      list, an inline "preview unavailable" state on any `ZipPreviewError`, and a close
      button wired to `close_zip_preview`.
- [x] 4.4 No extra gating needed — confirmed `root_view.rs` only calls
      `render_detail_tab_content`/`render_item_tier` for the currently-active `TabTarget::Detail`
      (a `match` on `tabs_snap.active`, not a hide/show toggle over an always-mounted tree),
      so a non-active detail tab's popover is never rendered in the first place.
- [x] 4.5 Reused the existing `clear_item_selection` call sites (already invoked wherever a
      detail tab is (re)opened, in `catalog_view.rs` and `item_popover_view.rs`) rather than
      adding a new `TabsController` subscription — `clear_item_selection` now also clears
      `zip_preview` for that entry id, matching `selected_item_file`'s established convention
      of clearing at open-time rather than at close-time.

**Deviation note:** no debounce was added on hover (design's "Risks" section flagged
re-reading the Zip's central directory on rapid hover on/off). `list_entries` only runs at
render time for whichever file is the *current* active preview, and `hover_zip_preview`
already short-circuits without notifying when the target is unchanged, which limits redundant
reads to actual row changes, not raw mouse-move ticks. A time-based debounce can be added
later if profiling shows it's needed.

## 5. Verification

- [x] 5.1 Manual pass — **not done by the implementing agent** (UI verification is left to
      the user per this project's workflow); deferred to the user to verify post-merge
      against a real downloaded Zip item via the live SDK backend (no runnable stub mode —
      see 1.3/1.4): hover a Zip row shows the popover and scrolls for a many-entry archive;
      moving off without clicking closes it; clicking pins it; a second click or close
      control unpins it; switching tabs hides it; closing the detail tab and reopening does
      not show stale state.
- [x] 5.2 `cargo clippy -p dtrpg-ui --all-targets --all-features -- -D warnings` passes.
- [x] 5.3 `cargo +nightly fmt --all -- --check` passes.
- [x] 5.4 `cargo test --workspace` passes (293 tests across `dtrpg-core`/`dtrpg-ui`, plus
      doctests), including the new `zip_preview` unit tests.
