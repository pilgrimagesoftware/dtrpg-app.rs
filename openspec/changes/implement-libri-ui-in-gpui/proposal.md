## Why

The Rust desktop app has an accepted layout contract (`define-rust-main-window-library-layout`) but no specification of the concrete visual design, interaction model, or component breakdown that will drive GPUI implementation. The Libri HTML prototype establishes all of that — sidebar navigation, three catalog layouts, generative covers, slide-over detail panel, four color themes, and density variants — and must be translated into normative OpenSpec requirements before GPUI development begins.

## What Changes

- Introduce a `libri-sidebar` capability: persistent left nav with logo, four smart library sections (All Titles / Recently Added / On This Device / In the Cloud), per-publisher nav items with counts, and a footer showing total title count and storage size.
- Introduce a `libri-toolbar` capability: title/count header, search input with clear action, sort dropdown (Title / Publisher / Date Added / Page Count), Group-by-publisher toggle, and layout switcher (list | thumbs | grid segmented control).
- Introduce a `libri-catalog` capability: three catalog layouts sharing one filtered/sorted result set — text-list rows, thumbnail-list rows, and grid cards — with optional publisher grouping and an empty state.
- Introduce a `libri-cover` capability: cover images that prefer a real thumbnail fetched from the catalog item's cover URL and cached on disk; fall back to a deterministic generative cover (per-book color, luminance-aware foreground, four hash-derived motif variants) when a thumbnail cannot be acquired from cache or downloaded.
- Introduce a `libri-detail-panel` capability: slide-over detail panel triggered by item selection, showing cover, publisher, title, game line, description, Read and Download/Downloaded actions, and a metadata table; dismissible via close button or Escape key.
- Introduce a `libri-theme` capability: four named color themes (parchment, slate, sage, ink) expressed as a set of semantic color tokens; two density variants (comfortable, compact) controlling row height, thumbnail size, card dimensions, and padding.

## Capabilities

### New Capabilities

- `libri-sidebar`: Left navigation panel with logo, smart library sections, publisher list, and storage summary footer.
- `libri-toolbar`: Top control strip with title, count, search, sort, grouping toggle, and layout switcher.
- `libri-catalog`: Three catalog layouts (list, thumbs, grid) over a shared filtered/sorted result set, with optional publisher grouping and an empty state.
- `libri-cover`: Cover images sourced from cached or downloaded catalog thumbnails, with a deterministic generative cover (per-book color, hash-derived motif) as the fallback when a real image is unavailable.
- `libri-detail-panel`: Slide-over item detail panel with cover, metadata, description, and action buttons.
- `libri-theme`: Four color themes and two density variants expressed as semantic design tokens applied to the GPUI view hierarchy.

### Modified Capabilities

(none — no existing requirement specs are changing)

## Impact

- `dtrpg-app/rust`: All GPUI view and model modules for the main window. Six new capability specs drive implementation tasks.
- Depends on `define-rust-main-window-library-layout` for the accepted layout contract and controller/view boundaries.
- Depends on existing library data model fields: `title`, `publisher`, `line`, `kind`, `format`, `pages`, `size_mb`, `year`, `added`, `status`, `color`, `desc`.
- No SDK contract changes required; no new API endpoints.
