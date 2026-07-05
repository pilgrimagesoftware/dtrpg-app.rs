## Context

The Rust frontend (`dtrpg-gui`) currently renders the main window as a minimal scaffold: raw hex colors, a flat item list or basic tree, and a text-only detail pane. The `LibraryController` already owns filtering, sorting, view mode, grouping, account menu state, and sync status. What is missing is a GPUI view layer that translates the Libri visual design into that controller's state — the sidebar navigation, toolbar controls, three catalog layouts, generative covers, slide-over detail panel, and the theme/density token system.

The design prototype (in `rust/design/`) is the authoritative reference for visual intent. This design maps each prototype component to the existing GPUI view hierarchy.

## Goals / Non-Goals

**Goals:**

- Implement the Libri sidebar: logo, four smart library sections with counts, scrollable per-publisher nav, storage summary footer.
- Implement the Libri toolbar: current-section title, matched item count, search input with clear, sort dropdown, group-by toggle, and list/thumbs/grid layout switcher.
- Implement three catalog layouts that share one filtered/sorted result set from `LibraryController`: text-list rows, thumbnail-list rows, and grid cards — with optional publisher grouping sections and a search-empty state.
- Implement deterministic generative book covers: per-book `color` field drives background; luminance determines foreground (cream or ink); hash of `(id, title)` selects one of four motifs (circle, diamond, double-rule, triangle).
- Implement the slide-over detail panel: triggered by item selection, contains cover, publisher, title, game line, description, Read and Download/Downloaded action buttons, and a metadata table; dismissible via close button or Escape.
- Implement a GPUI theme system: four named themes (parchment, slate, sage, ink) as semantic color token structs; two density variants (comfortable, compact) as layout constant sets; both applied through a `ThemeContext` (or equivalent GPUI global) accessible to all views.

**Non-Goals:**

- Replacing or restructuring `LibraryController` — its state and methods are the source of truth for this design.
- Implementing real file download/open functionality — action buttons fire controller methods and update status; file I/O is out of scope here.
- Implementing the account menu or settings panel beyond the existing controller stubs.
- Adding new `LibraryItem` fields — the design uses `title`, `publisher`, `line`, `kind`, `format`, `pages`, `size_mb`, `year`, `added_order`, `status`, `color`, `desc` already present on the model. Fields not yet on the model (e.g. `color`, `desc`, `line`, `kind`, `format`, `pages`, `year`) will need to be added to `LibraryItem` as part of implementation.
- Custom typeface loading — system serif (`ui-serif` equivalent) is acceptable for GPUI; the prototype's Google Fonts are web-only.

## Decisions

### 1. Sidebar replaces the current left nav stub

The sidebar is a fixed-width GPUI `div` column rendered by a new `render_sidebar` function. It is not a platform `NSOutlineView` or similar — it is a GPUI-native panel consistent with the existing GPUI shell architecture.

Smart section items (All Titles, Recently Added, On This Device, In the Cloud) map to `FilterScope` values or dedicated filter modes on `LibraryController`. Publisher nav items reuse the existing `TreeNode` publisher grouping to derive names and counts.

**Rationale:** The existing GPUI shell is already a custom-rendered window; a native outline view would require AppKit bridging that doesn't exist in the codebase.

### 2. Three catalog layouts as a `LibraryViewMode` expansion

`LibraryViewMode` currently has `FlatList`, `TreeByPublisher`, `TreeByProductType`, `GridByPublisher`, `GridByProductType`. The Libri design uses three presentation modes: `list` (text rows), `thumbs` (thumbnail rows), `grid` (cover cards). Grouping by publisher is a separate boolean toggle.

Rather than adding more `LibraryViewMode` variants, the design maps:
- Libri list → `FlatList` or `TreeByPublisher` depending on the grouping toggle
- Libri thumbs → same modes, different render path
- Libri grid → `GridByPublisher` or `GridByProductType`

A new `CatalogPresentation` enum (`List`, `Thumbs`, `Grid`) is added to the controller alongside the existing `LibraryViewMode`. The grouping toggle drives whether the publisher tree is used. Both enums together determine the final render path.

**Rationale:** Avoids exploding the `LibraryViewMode` enum further while giving the toolbar three clean controls (presentation + grouping toggle).

### 3. Cover rendering: real thumbnail first, generative fallback

`render_cover` checks a disk-backed image cache for the item's cover thumbnail before rendering. The resolution order is:

1. **Cache hit** — render the cached image immediately, no network call.
2. **Cache miss, network available** — enqueue a background download of the cover URL; render the generative fallback while the download is in flight; replace with the real image once the download completes and the cache is populated.
3. **Cache miss, download failed or unavailable** — render the generative fallback permanently for this session.

The generative fallback is a stateless `fn cover_style(item: &LibraryItem) -> CoverStyle` returning a `CoverStyle` struct (background color, foreground color, motif variant, display strings). The motif is `fnv_hash(item.id.to_string() + &item.title) % 4`. Luminance check uses the ITU-R 601 formula: `(r*299 + g*587 + b*114) / 1000 > 150`.

The background download MUST NOT block the GPUI main thread. Use a `cx.spawn` task or equivalent off-thread mechanism. The GPUI view invalidates and re-renders the cover slot when the cached image becomes available.

**Rationale:** Real cover art is strongly preferred for product quality, but the library must remain fully usable offline or before images are fetched. The generative fallback is always available and requires no network or disk I/O, making it safe to render synchronously during the first paint.

### 4. Slide-over detail panel via animated GPUI `div`

The detail panel is a fixed-position right-edge `div` that is conditionally rendered when `controller.selection` contains a `Selection::Item`. It slides in using a GPUI `transition` or opacity animation when the selection changes. Escape key dismissal is handled by extending `handle_global_key` in `LibraryController` to detect `"escape"` while detail is open and call `ClearSelection`.

**Rationale:** GPUI does not have a native drawer primitive; a positioned overlay div is consistent with how the existing overlay panes are built.

### 5. Theme and density as a `LibriTheme` global on the GPUI `App`

A `LibriTheme` struct holds all semantic color tokens as `gpui::Rgba` values and a `Density` variant (`Comfortable` / `Compact`) that carries layout constants (row height, thumb size, card min width, card gap, catalog padding). It is stored as a GPUI app-level global and read by all view render functions via `cx.global::<LibriTheme>()`.

Initial theme is `Parchment` / `Comfortable`. Theme switching is a new `AppCommand::SetTheme(ThemeKey)` dispatched through the shell.

**Rationale:** GPUI's global mechanism is the idiomatic way to share ambient state (fonts, colors, sizes) without threading it through every view parameter.

## Risks / Trade-offs

- **Cover download concurrency may cause flickering or redundant requests.** Multiple grid cards for the same publisher might enqueue duplicate downloads before the first completes. Mitigation: key the in-flight download set by item id and skip re-enqueuing if a download is already pending.
- **`LibraryItem` is missing several fields the design requires** (`color`, `desc`, `line`, `kind`, `format`, `pages`, `size_mb`, `year`). These must be added to the SDK `LibraryItem` struct and stub data before the cover and detail panel can render correctly. Mitigation: add placeholder values in stubs; wire real SDK fields as a follow-on task.
- **GPUI does not have a native CSS-style transition system.** Slide-over animation may need to be approximated with opacity toggling or a position offset driven by a frame timer. Mitigation: treat panel as non-animated initially; add animation once the layout is stable.
- **GPUI text layout differs from web.** Truncation, line clamping, and multi-line cover titles require explicit GPUI text element configuration. Mitigation: test cover rendering with long titles early; add `.truncate()` or explicit max-width constraints.
- **Theme global adds app-level state.** If GPUI's global mechanism requires `Model<LibriTheme>` rather than a plain struct, theme switching will require a `cx.update_global` call pattern. Mitigation: prototype the global pattern in a small spike before full integration.

## Migration Plan

1. Add missing `LibraryItem` fields with stubs; update stub service seeded data.
2. Add `LibriTheme` global and `CatalogPresentation` enum to the controller.
3. Implement sidebar render function and wire smart sections + publisher nav.
4. Implement toolbar render function with layout switcher and grouping toggle.
5. Implement `render_cover` and `cover_style` pure function with all four motifs.
6. Implement text-list and thumbnail-list catalog layouts.
7. Implement grid catalog layout (reuses cover render).
8. Implement detail panel slide-over with metadata table and action buttons.
9. Implement all four themes; validate token coverage against each view.
10. Implement compact density variant; verify layout constants apply correctly.
11. Smoke test with seeded stub data across all themes, densities, and layouts.

## Open Questions

- Should `CatalogPresentation` live on `LibraryController` or be pure UI state owned by the root view? (Leaning toward controller, since it affects `flat_items()` vs `tree_items()` dispatch.)
- What GPUI primitive best approximates a segmented control for the layout switcher — three adjacent buttons with a shared border? Or a row of toggle buttons?
- Should the "Recently Added" smart section use `added_order` already on `LibraryItem`, or does it need a timestamp field?
- Should theme persistence use `UserDefaults` (macOS), a local config file, or remain in-memory for now?
