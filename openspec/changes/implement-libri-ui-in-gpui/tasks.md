## 1. Data Model Preparation

- [ ] 1.1 Add `color`, `desc`, `line`, `kind`, `format`, `pages`, `size_mb`, and `year` fields to the `LibraryItem` struct in `dtrpg-core`
- [ ] 1.2 Update the stub `LibraryItem` factory in `StubLibraryService` to populate all new fields with representative values
- [ ] 1.3 Update `LibraryViewModel` and `AppShell` to pass new fields through without loss
- [ ] 1.4 Add `CatalogPresentation` enum (`List`, `Thumbs`, `Grid`) to `library_data.rs`
- [ ] 1.5 Add `catalog_presentation` field to `LibraryController` defaulting to `Grid`
- [ ] 1.6 Add `set_catalog_presentation` method to `LibraryController`
- [ ] 1.7 Add smart-section counts to `LibraryController`: `recently_added_count`, `downloaded_count`, `cloud_count` computed from the loaded items

## 2. Theme System

- [ ] 2.1 Define `LibriTheme` struct with all required semantic color token fields (`desktop_bg`, `surface`, `surface_alt`, `hover`, `text_primary`, `text_secondary`, `text_tertiary`, `border`, `border_strong`, `accent`, `accent_soft`, `accent_on`, `shadow`, `scrim`)
- [ ] 2.2 Define `Density` enum (`Comfortable`, `Compact`) with associated layout constant struct (`row_text_height`, `thumb_width`, `card_min_width`, `card_gap_x`, `card_gap_y`, `catalog_padding`)
- [ ] 2.3 Implement token values for the parchment theme
- [ ] 2.4 Implement token values for the slate theme
- [ ] 2.5 Implement token values for the sage theme
- [ ] 2.6 Implement token values for the ink theme
- [ ] 2.7 Register `LibriTheme` as a GPUI app-level global
- [ ] 2.8 Add `SetTheme(ThemeKey)` and `SetDensity(Density)` commands to `AppCommand` and wire them through `AppShell`

## 3. Cover System (Thumbnail + Generative Fallback)

- [ ] 3.1 Add a `cover_url` field to `LibraryItem`; populate with representative URLs in the stub service
- [ ] 3.2 Implement a disk-backed `CoverCache` (keyed by item id) with `get(id) -> Option<Image>` and `insert(id, image)` methods in a new `cover_cache.rs` module
- [ ] 3.3 Register `CoverCache` as a GPUI app-level global so all views can read from it
- [ ] 3.4 Implement `render_cover` GPUI element function: checks `CoverCache` first; if present, renders the cached image; otherwise renders the generative fallback and enqueues a background download
- [ ] 3.5 Implement the background download task using `cx.spawn` (or equivalent off-thread mechanism); on success write to `CoverCache` and invalidate the cover slot; on failure leave the generative fallback in place
- [ ] 3.6 Implement the `cover_style(item: &LibraryItem) -> CoverStyle` pure function: FNV hash of `item.id.to_string() + &item.title`, `% 4` motif selection, ITU-R 601 luminance check for foreground color
- [ ] 3.7 Implement the generative cover GPUI element: colored background, publisher text (top), motif element (center), title text (center), game line text (bottom)
- [ ] 3.8 Implement all four motif variants: circle, diamond, double horizontal rule, triangle
- [ ] 3.9 Write unit tests for `cover_style` asserting: same input → same motif, luminance threshold for light and dark color fields
- [ ] 3.10 Verify that catalog scrolling and interaction remain unblocked while cover downloads are in flight

## 4. Sidebar

- [ ] 4.1 Create `render_sidebar` function in a new `sidebar_view.rs` module
- [ ] 4.2 Render the wordmark header (logo mark + "Libri" text)
- [ ] 4.3 Render the Library section group with four smart nav items (All Titles, Recently Added, On This Device, In the Cloud) and their count badges
- [ ] 4.4 Wire smart nav item activation to update the active filter on `LibraryController`
- [ ] 4.5 Render the active nav item in the selected visual state using active theme tokens
- [ ] 4.6 Render the Publishers section group with one nav item per publisher (ordered by count desc, name asc) with count badges
- [ ] 4.7 Wire publisher nav item activation to set the publisher filter on `LibraryController`
- [ ] 4.8 Make the Publishers section independently scrollable within the sidebar
- [ ] 4.9 Render the footer with total title count and human-readable total file size

## 5. Toolbar

- [ ] 5.1 Create `render_toolbar` function in a new `toolbar_view.rs` module
- [ ] 5.2 Render the title/count header (section name and matched item count badge)
- [ ] 5.3 Render the search input; wire text changes to `LibraryController::set_filter_query`
- [ ] 5.4 Render the clear button conditionally when search is non-empty; wire to `LibraryController::clear_filter_query`
- [ ] 5.5 Render the sort dropdown with options: Title, Publisher, Date Added, Page Count; wire to `set_flat_sort` / `set_outer_sort`
- [ ] 5.6 Render the Group by publisher toggle; wire to `set_catalog_presentation` (grouped vs. flat) and view mode selection
- [ ] 5.7 Render the layout switcher segmented control (List / Thumbs / Grid); wire to `set_catalog_presentation`
- [ ] 5.8 Apply active theme tokens to all toolbar elements

## 6. Catalog — List and Thumbs Layouts

- [ ] 6.1 Implement text-list column header row in `render_library_pane`
- [ ] 6.2 Implement text-list item rows with all required columns: title+kind tag, publisher, system, pages, size, date added, status glyph
- [ ] 6.3 Implement thumbnail-list item rows with cover thumbnail, title, publisher+line, kind tag, dimensions, date, status glyph
- [ ] 6.4 Render the dot status glyph for downloaded items
- [ ] 6.5 Render the cloud icon status glyph for cloud-only items
- [ ] 6.6 Implement publisher section headers (name + count badge) for grouped mode in both List and Thumbs layouts
- [ ] 6.7 Implement the empty state view (search icon + "No titles match.") shown when the filtered result set is empty

## 7. Catalog — Grid Layout

- [ ] 7.1 Implement grid card layout using `render_cover` for the card cover area
- [ ] 7.2 Render card footer with title, publisher name, and status glyph
- [ ] 7.3 Implement publisher section headers for grouped grid mode
- [ ] 7.4 Apply density constants (card min-width, column gap, row gap) from the active `Density` variant

## 8. Detail Panel

- [ ] 8.1 Create `render_detail_panel` function in a new `detail_panel_view.rs` module
- [ ] 8.2 Render the panel conditionally based on `LibraryController::selection` containing a `Selection::Item`
- [ ] 8.3 Render the close button; wire to `LibraryController::shell.dispatch(AppCommand::ClearSelection)`
- [ ] 8.4 Extend `LibraryController::handle_global_key` to dismiss the detail panel on Escape when an item is selected
- [ ] 8.5 Render the cover using `render_cover` at the top of the panel
- [ ] 8.6 Render the publisher name, title heading, and game line below the cover
- [ ] 8.7 Render the description paragraph
- [ ] 8.8 Render the primary "Read" button
- [ ] 8.9 Render the Download/Downloaded secondary button; wire to a toggle-download action on the controller
- [ ] 8.10 Render the metadata table with rows: System, Category, Format, Pages, File size, Released, Added, Status
- [ ] 8.11 Apply active theme tokens to all panel elements

## 9. Root View Integration

- [ ] 9.1 Compose sidebar, toolbar, catalog, and detail panel into `render_root_view` in `root_view.rs`
- [ ] 9.2 Replace all hardcoded hex color values in existing view files with reads from the active `LibriTheme` global
- [ ] 9.3 Apply density constants from the active `Density` variant across all catalog layouts
- [ ] 9.4 Verify that switching layout between List, Thumbs, and Grid preserves filtered result set and sort order

## 10. Verification

- [ ] 10.1 Smoke test all four themes: parchment, slate, sage, ink — confirm token coverage with no hardcoded colors remaining
- [ ] 10.2 Smoke test comfortable and compact density variants across all three catalog layouts
- [ ] 10.3 Verify search filters title, publisher, and game line fields case-insensitively
- [ ] 10.4 Verify empty state appears when no items match the active query
- [ ] 10.5 Verify group-by-publisher sections appear and disappear correctly across all three layouts
- [ ] 10.6 Verify detail panel opens and closes via click and Escape; confirm metadata table values are correct
- [ ] 10.7 Verify Download/Downloaded button toggles item status and the status glyph updates in the catalog
- [ ] 10.8 Verify generative covers are deterministic: same item always produces the same motif and foreground color
- [ ] 10.9 Verify that real thumbnail images replace the generative fallback after a successful background download
- [ ] 10.10 Verify that duplicate cover download tasks are not enqueued when multiple cards for the same item are visible simultaneously
