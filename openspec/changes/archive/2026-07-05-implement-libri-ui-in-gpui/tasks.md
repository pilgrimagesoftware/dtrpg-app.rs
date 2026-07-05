## 1. Data Model Preparation

- [x] 1.1 Add `color`, `desc`, `line`, `kind`, `format`, `pages`, `size_mb`, and `year` fields to the `LibraryItem` struct in `dtrpg-core`
- [x] 1.2 Update the stub `LibraryItem` factory in `StubLibraryService` to populate all new fields with representative values
- [x] 1.3 Update `LibraryViewModel` and `AppShell` to pass new fields through without loss
- [x] 1.4 Add `CatalogPresentation` enum (`List`, `Thumbs`, `Grid`) to `library_data.rs`
- [x] 1.5 Add `catalog_presentation` field to `LibraryController` defaulting to `Grid`
- [x] 1.6 Add `set_catalog_presentation` method to `LibraryController`
- [x] 1.7 Add smart-section counts to `LibraryController`: `recently_added_count`, `downloaded_count`, `cloud_count` computed from the loaded items

## 2. Theme System

- [x] 2.1 Define `LibriTheme` struct with all required semantic color token fields (`desktop_bg`, `surface`, `surface_alt`, `hover`, `text_primary`, `text_secondary`, `text_tertiary`, `border`, `border_strong`, `accent`, `accent_soft`, `accent_on`, `shadow`, `scrim`)
- [x] 2.2 Define `Density` enum (`Comfortable`, `Compact`) with associated layout constant struct (`row_text_height`, `thumb_width`, `card_min_width`, `card_gap_x`, `card_gap_y`, `catalog_padding`)
- [x] 2.3 Implement token values for the parchment theme
- [x] 2.4 Implement token values for the slate theme
- [x] 2.5 Implement token values for the sage theme
- [x] 2.6 Implement token values for the ink theme
- [x] 2.7 Register `LibriTheme` as a GPUI app-level global
- [x] 2.8 Add `SetTheme(ThemeKey)` and `SetDensity(Density)` commands to `AppCommand` and wire them through `AppShell`

## 3. Cover System (Thumbnail + Generative Fallback)

- [x] 3.1 Add a `cover_url` field to `LibraryItem`; populate with representative URLs in the stub service
- [x] 3.2 Implement a disk-backed `CoverCache` (keyed by item id) with `get(id) -> Option<Image>` and `insert(id, image)` methods in a new `cover_cache.rs` module
- [x] 3.3 Register `CoverCache` as a GPUI app-level global so all views can read from it
- [x] 3.4 Implement `render_cover` GPUI element function: checks `CoverCache` first; if present, renders the cached image; otherwise renders the generative fallback and enqueues a background download
- [x] 3.5 Implement the background download task using `cx.spawn` (or equivalent off-thread mechanism); on success write to `CoverCache` and invalidate the cover slot; on failure leave the generative fallback in place
- [x] 3.6 Implement the `cover_style(item: &LibraryItem) -> CoverStyle` pure function: FNV hash of `item.id.to_string() + &item.title`, `% 4` motif selection, ITU-R 601 luminance check for foreground color
- [x] 3.7 Implement the generative cover GPUI element: colored background, publisher text (top), motif element (center), title text (center), game line text (bottom)
- [x] 3.8 Implement all four motif variants: circle, diamond, double horizontal rule, triangle
- [x] 3.9 Write unit tests for `cover_style` asserting: same input → same motif, luminance threshold for light and dark color fields
- [x] 3.10 Verify that catalog scrolling and interaction remain unblocked while cover downloads are in flight

## 4. Sidebar

- [x] 4.1 Create `render_sidebar` function in a new `sidebar_view.rs` module
- [x] 4.2 Render the wordmark header (logo mark + "Libri" text)
- [x] 4.3 Render the Library section group with four smart nav items (All Titles, Recently Added, On This Device, In the Cloud) and their count badges
- [x] 4.4 Wire smart nav item activation to update the active filter on `LibraryController`
- [x] 4.5 Render the active nav item in the selected visual state using active theme tokens
- [x] 4.6 Render the Publishers section group with one nav item per publisher (ordered by count desc, name asc) with count badges
- [x] 4.7 Wire publisher nav item activation to set the publisher filter on `LibraryController`
- [x] 4.8 Make the Publishers section independently scrollable within the sidebar
- [x] 4.9 Render the footer with total title count and human-readable total file size

## 5. Toolbar

- [x] 5.1 Create `render_toolbar` function in a new `toolbar_view.rs` module
- [x] 5.2 Render the title/count header (section name and matched item count badge)
- [x] 5.3 Render the search input; wire text changes to `LibraryController::set_filter_query`
- [x] 5.4 Render the clear button conditionally when search is non-empty; wire to `LibraryController::clear_filter_query`
- [x] 5.5 Render the sort dropdown with options: Title, Publisher, Date Added, Page Count; wire to `set_flat_sort` / `set_outer_sort`
- [x] 5.6 Render the Group by publisher toggle; wire to `set_catalog_presentation` (grouped vs. flat) and view mode selection
- [x] 5.7 Render the layout switcher segmented control (List / Thumbs / Grid); wire to `set_catalog_presentation`
- [x] 5.8 Apply active theme tokens to all toolbar elements

## 6. Catalog — List and Thumbs Layouts

- [x] 6.1 Implement text-list column header row in `render_library_pane`
- [x] 6.2 Implement text-list item rows with all required columns: title+kind tag, publisher, system, pages, size, date added, status glyph
- [x] 6.3 Implement thumbnail-list item rows with cover thumbnail, title, publisher+line, kind tag, dimensions, date, status glyph
- [x] 6.4 Render the dot status glyph for downloaded items
- [x] 6.5 Render the cloud icon status glyph for cloud-only items
- [x] 6.6 Implement publisher section headers (name + count badge) for grouped mode in both List and Thumbs layouts
- [x] 6.7 Implement the empty state view (search icon + "No titles match.") shown when the filtered result set is empty

## 7. Catalog — Grid Layout

- [x] 7.1 Implement grid card layout using `render_cover` for the card cover area
- [x] 7.2 Render card footer with title, publisher name, and status glyph
- [x] 7.3 Implement publisher section headers for grouped grid mode
- [x] 7.4 Apply density constants (card min-width, column gap, row gap) from the active `Density` variant

## 8. Detail Panel

- [x] 8.1 Create `render_detail_panel` function in a new `detail_panel_view.rs` module
- [x] 8.2 Render the panel conditionally based on `LibraryController::selection` containing a `Selection::Item`
- [x] 8.3 Render the close button; wire to `LibraryController::shell.dispatch(AppCommand::ClearSelection)`
- [x] 8.4 Extend `LibraryController::handle_global_key` to dismiss the detail panel on Escape when an item is selected
- [x] 8.5 Render the cover using `render_cover` at the top of the panel
- [x] 8.6 Render the publisher name, title heading, and game line below the cover
- [x] 8.7 Render the description paragraph
- [x] 8.8 Render the primary "Read" button
- [x] 8.9 Render the Download/Downloaded secondary button; wire to a toggle-download action on the controller
- [x] 8.10 Render the metadata table with rows: System, Category, Format, Pages, File size, Released, Added, Status
- [x] 8.11 Apply active theme tokens to all panel elements

## 9. Root View Integration

- [x] 9.1 Compose sidebar, toolbar, catalog, and detail panel into `render_root_view` in `root_view.rs`
- [x] 9.2 Replace all hardcoded hex color values in existing view files with reads from the active `LibriTheme` global
- [x] 9.3 Apply density constants from the active `Density` variant across all catalog layouts
- [x] 9.4 Verify that switching layout between List, Thumbs, and Grid preserves filtered result set and sort order

## 10. Verification

- [x] 10.1 Smoke test all four themes: parchment, slate, sage, ink — confirm token coverage with no hardcoded colors remaining
- [x] 10.2 Smoke test comfortable and compact density variants across all three catalog layouts
- [x] 10.3 Verify search filters title, publisher, and game line fields case-insensitively
- [x] 10.4 Verify empty state appears when no items match the active query
- [x] 10.5 Verify group-by-publisher sections appear and disappear correctly across all three layouts
- [x] 10.6 Verify detail panel opens and closes via click and Escape; confirm metadata table values are correct
- [x] 10.7 Verify Download/Downloaded button toggles item status and the status glyph updates in the catalog
- [x] 10.8 Verify generative covers are deterministic: same item always produces the same motif and foreground color
- [x] 10.9 Verify that real thumbnail images replace the generative fallback after a successful background download
- [x] 10.10 Verify that duplicate cover download tasks are not enqueued when multiple cards for the same item are visible simultaneously
