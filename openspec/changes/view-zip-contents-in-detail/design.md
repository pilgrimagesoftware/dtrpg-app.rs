## Context

The expanded detail tab (`render_detail_tab_content` in `detail_panel_view.rs`) currently
renders a single item's metadata and a single download/open/reveal action row. It has no
per-item file list — `LibraryItem` models one catalog entry as if it were one file, even
though DriveThruRPG products are frequently delivered as a single Zip bundling several
files. This was called out as a known data-model gap when the detail tab was first built
(`add-rust-main-window-structure`), blocked on a future "multi-item catalog entry" change.
This change is that follow-up, scoped specifically to make Zip archive contents visible.

The app's backend is currently fully stubbed (`rust-library-ui-implementation`:
"Rust baseline implementation MUST use stubbed backend adapters") — `toggle_download` only
flips an enum, no real file is fetched. This design has to work today against stub data and
continue to work unchanged once real downloads land.

Two existing patterns in the codebase are directly reused:
- The anchored/deferred popover pattern in `item_popover_view.rs`
  (`deferred(anchored().position(...).child(...))`), already the established way to render
  a floating panel tied to controller state rather than gpui-component's click-toggled
  `Popover`.
- `TabsController`'s `active: TabTarget` as the single source of truth for which tab (and
  therefore which detail item) is currently visible.

## Goals / Non-Goals

**Goals:**
- Render a per-file row list in the detail tab for items with more than one bundled file
  (or a Zip that itself contains multiple entries).
- On hover over a Zip file row, show a scrollable popover listing that Zip's internal
  entries (name + size), read from the file's central directory without extracting it.
- On click, pin the popover open so it survives mouse-out; a second click, its own close
  control, or leaving the detail tab dismisses it.
- Guarantee the popover is never visible unless its owning detail tab is the active tab.

**Non-Goals:**
- Extracting or opening individual files from within the Zip (preview is read-only listing
  of names/sizes; opening an entry is a future capability).
- Reading non-Zip archive formats (rar, 7z, tar).
- Wiring real downloads/SDK file transfer — this change reads whatever Zip file already
  exists at the item's on-disk path (real once downloads are implemented, absent under the
  current stub).

## Decisions

### 1. `ItemFile` model on `LibraryItem`

Add:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFile {
    pub name: Arc<str>,
    pub size_mb: f64,
    pub is_zip: bool,
}
```

and `pub files: Vec<ItemFile>` on `LibraryItem` (`#[serde(default)]` so existing cached
catalog JSON deserializes without migration). Stub catalog generation populates `files`
alongside the existing `size_mb`/`format` stub fields — for a "PDF + EPUB" format item this
is two `ItemFile` rows; a Zip-bundled item is one `ItemFile` with `is_zip: true`.

Alternative considered: model files as a separate lookup table keyed by item id in the
controller (mirrors `titles: HashMap<Arc<str>, String>` in `TabsController`). Rejected —
files are intrinsic, serialized catalog data, not transient UI state; keeping them on
`LibraryItem` matches how every other item attribute is modeled and avoids a second
id-keyed map to keep in sync with catalog mutations.

### 2. Reading Zip contents: on demand, not cached in the model

`ItemFile` does not store the archive's internal entries. When a Zip row is hovered, the
detail view calls a small adapter (`crate::util::zip_preview::list_entries(path) ->
Result<Vec<ZipEntry>, ZipPreviewError>`) that opens the file at
`storage_root_path/items/<id>/<file.name>` and reads only the central directory via the
`zip` crate (`zip::ZipArchive::new` + `by_index_raw` / `.file_names()` — no decompression).

Alternative considered: precompute and cache entry lists when an item is marked
downloaded. Rejected for this change — adds cache invalidation surface (re-download,
partial download, file moved) for a preview that is cheap to read on hover (central
directory reads are O(entry count), not O(file size)). Revisit if profiling shows hover
latency is an issue.

Failure handling: file missing, not a valid Zip, or a read error all return
`ZipPreviewError` variants; the popover renders an inline error line ("preview
unavailable") instead of failing to open. Nothing panics; no `.unwrap()`/`.expect()` on the
read path.

### 3. Hover/pin state lives on `LibraryController`, gated by tab visibility

Add to `LibraryController`:

```rust
zip_preview_hovered: Option<Arc<str>>, // file name, only meaningful for the active detail item
zip_preview_pinned: Option<Arc<str>>,
zip_preview_anchor_pos: Option<Point<Pixels>>,
```

`render_detail_tab_content` takes the owning item's id and the current `TabTarget` from
`TabsSnapshot`; it only reads/renders the popover state when
`active == TabTarget::Detail(item.id)`. `LibraryController` additionally clears all three
fields whenever `TabsController` emits a close or switch away from that detail tab (wired
via the existing `LibraryChanged`/`TabsChanged` event subscription already used to
coordinate the two controllers) — this is belt-and-suspenders so state does not persist
stale into a future re-open of the same or a different detail tab.

Alternative considered: per-tab state stored in a `HashMap<Arc<str>, ZipPreviewState>` to
support multiple detail tabs independently retaining pinned state while inactive. Rejected
by explicit requirement — the popover must not be visible except while its detail view is
being viewed, so there is no need to persist it for inactive tabs; a single-slot field is
simpler and enough.

### 4. Popover rendering reuses the anchored/deferred pattern, not `gpui-component::Popover`

New `zip_preview_popover.rs` module mirrors `item_popover_view.rs`: a plain
`div()...deferred(anchored().position(anchor_pos).child(content))`, anchored at the file
row's bounds (captured on hover/click, same technique as
`detail-popover-stable-anchor`'s frozen anchor point — captured once, not re-read from a
continuously updating mouse position). Content is a scrollable column
(`gpui_component::scroll::ScrollableElement`, already imported in `detail_panel_view.rs`)
listing entry name + size per row, capped at a fixed max height so very large archives
scroll instead of overflowing the window.

Alternative considered: `gpui_component::Popover` (click-toggled, built-in open/close).
Rejected — it does not support hover-to-preview / click-to-pin as two distinct trigger
mouse behaviors out of the box, and the codebase already has a working anchored-popover
convention for exactly this "info panel tied to controller state" shape.

### 5. New dependency: `zip` crate, read-only, `default-features = false`

Added to `dtrpg-ui`'s `Cargo.toml` with only the `deflate` feature (the common compression
method for DriveThruRPG Zips); no write/compression features enabled.

## Risks / Trade-offs

- [Risk] Large Zip archives (thousands of entries) could make the popover's entry list
  slow to render or scroll → Mitigation: virtualize is out of scope for v1 given expected
  entry counts (tens, not thousands) for RPG PDF/EPUB bundles; cap rendered rows with a
  "+N more" line if a future archive exceeds a fixed threshold (documented in tasks, not a
  blocking requirement now).
- [Risk] Reading the Zip's central directory on every hover re-reads the file from disk
  repeatedly if a user hovers on/off rapidly → Mitigation: debounce the hover-triggered read
  (only read after a short hover delay, matching typical tooltip-delay UX) and short-circuit
  if the same file is already the pinned/loaded one.
- [Risk] Current stub backend means no real Zip file exists on disk for any item yet, so the
  feature cannot be manually verified against real downloads → Mitigation: stub catalog
  generation ships a small fixture Zip file under the stub storage root for at least one
  sample item, and unit tests exercise `list_entries` against fixture Zips in `tests/`.

## Migration Plan

- `files: Vec<ItemFile>` defaults to empty via `#[serde(default)]`; existing cached catalog
  JSON on disk continues to deserialize, just with no file rows shown until the cache is
  next regenerated by the stub catalog generator.
- No rollback complexity beyond reverting the change — no server-side or persisted-schema
  migration involved.

## Open Questions

- Should the popover offer a "reveal in Finder/Explorer" affordance for a specific entry, or
  only the whole archive (as today)? Deferred — not required by this change's scope.
