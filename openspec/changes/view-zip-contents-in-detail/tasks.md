## 1. Data model

- [ ] 1.1 Add `ItemFile { name: Arc<str>, size_mb: f64, is_zip: bool }` to
      `crates/dtrpg-ui/src/data/library.rs`, deriving `Debug, Clone, Serialize, Deserialize`.
- [ ] 1.2 Add `#[serde(default)] pub files: Vec<ItemFile>` to `LibraryItem`.
- [ ] 1.3 Update stub catalog generation to populate `files` per item (single entry for
      single-format items, multiple entries for multi-format items, at least one Zip-typed
      entry for at least one stub item).
- [ ] 1.4 Add a small fixture Zip file under the stub storage root so the Zip-typed stub
      item has a real file to read during manual verification and tests.

## 2. Zip reading

- [ ] 2.1 Add the `zip` crate to `crates/dtrpg-ui/Cargo.toml` with
      `default-features = false` and only the compression feature(s) DriveThruRPG archives
      use.
- [ ] 2.2 Add `crates/dtrpg-ui/src/util/zip_preview.rs` with a typed `ZipPreviewError`
      (`thiserror::Error`) covering not-found, IO, and invalid-archive cases, and
      `list_entries(path: &Path) -> Result<Vec<ZipEntry>, ZipPreviewError>` that reads only
      the central directory (no extraction).
- [ ] 2.3 Unit tests for `list_entries`: valid fixture Zip returns expected entries; missing
      file, empty file, and non-Zip file each return the expected error variant, none panic.

## 3. Detail tab file list rendering

- [ ] 3.1 Replace the single download/open/reveal action row in
      `render_detail_tab_content` with one row per `ItemFile` in `item.files`.
- [ ] 3.2 Each row shows file name and size; Zip-typed rows are visually marked (icon or
      label) as eligible for content preview.
- [ ] 3.3 Preserve existing download/open/reveal actions per-row where they still apply
      (e.g. reveal-in-file-manager for the specific file), reusing
      `crate::util::reveal::reveal_in_file_manager`.

## 4. Hover/pin state and popover

- [ ] 4.1 Add `zip_preview_hovered`, `zip_preview_pinned`, `zip_preview_anchor_pos` fields
      to `LibraryController`, plus accessor/mutator methods that emit `LibraryChanged`.
- [ ] 4.2 Wire hover (with a short debounce) and click handlers on Zip file rows to update
      the above state and call `zip_preview::list_entries` for the hovered/pinned file's
      path.
- [ ] 4.3 Add `crates/dtrpg-ui/src/ui/views/zip_preview_popover.rs`: anchored/deferred
      popover (mirroring `item_popover_view.rs`) rendering a scrollable entry list (name +
      size), a fixed max height with scroll, an inline "preview unavailable" state for
      `ZipPreviewError`, and a close control.
- [ ] 4.4 Gate the popover's render call in `render_detail_tab_content` on
      `active == TabTarget::Detail(item.id)` from `TabsSnapshot`, so it never renders for a
      non-active detail tab.
- [ ] 4.5 Clear `zip_preview_hovered` / `zip_preview_pinned` / `zip_preview_anchor_pos` when
      `TabsController` closes or switches away from the owning detail tab.

## 5. Verification

- [ ] 5.1 Manual pass: hover a Zip row shows the popover and scrolls for a many-entry
      fixture; moving off without clicking closes it; clicking pins it; a second click or
      close control unpins it; switching tabs hides it; closing the detail tab and
      reopening does not show stale state.
- [ ] 5.2 `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [ ] 5.3 `cargo fmt --all -- --check` passes.
- [ ] 5.4 `cargo test --workspace` passes, including new `zip_preview` unit tests.
