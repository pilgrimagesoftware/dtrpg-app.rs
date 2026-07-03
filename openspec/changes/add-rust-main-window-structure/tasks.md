## 1. Structure Spec

- [x] 1.1 Add `rust-main-window-structure` capability delta spec
- [x] 1.2 Identify which in-progress Rust changes to build on (sidebar, activity panel,
      notifications, account menu, theme selector, catalog controls)

## 2. Title Bar

- [x] 2.1 Implement the title bar view with separator, title, and account button
- [x] 2.2 Extend the account menu with a sign-out action

## 3. Sidebar

- [x] 3.1 Add default navigation section item counts to the existing sidebar (already satisfied by
      `nav_item`'s count suffix — All Titles, Recently Added, On This Device, In the Cloud all show
      counts via `SectionCounts`; no code change needed)

## 4. Tabs

- [x] 4.1 Implement the `TabBar`-based tab strip with overflow "more" menu (`tab_strip_view.rs`,
      `TabsController`)
- [x] 4.2 Implement the non-closable catalog tab (`TabTarget::Catalog`, always first, no close
      suffix)
- [x] 4.3 Relocate search, sort, and view mode controls into the catalog tab header (toolbar now
      renders only inside the `TabTarget::Catalog` branch of `root_view.rs`'s active-tab match, not
      for detail tabs)
- [x] 4.4 Implement single-click popover detail (`item_popover_view.rs`, anchored via tracked
      cursor position; click-count escalation mirrors `gpui-component`'s own
      `TableState::on_row_left_click` pattern)
- [x] 4.5 Implement double-click expanded detail tab with thumbnail and attributes
      (`render_detail_tab_content` in `detail_panel_view.rs`). File list for multi-item entries is
      NOT implemented — `LibraryItem` has no per-item file list field yet; this is a data-model gap,
      not a UI gap, and blocks on a future `multi-item-catalog-entry-detail` data change.

## 5. Status Bar

- [x] 5.1 Implement the status bar view with library totals and active-tab summary
      (`status_bar_view.rs`, using `gpui-component`'s `StatusBar`). Retired the sidebar footer,
      which previously duplicated the library total and activity indicator. Selection count is NOT
      shown — this app has no bulk-selection state yet (`catalog-bulk-selection` change is
      unstarted); the active-tab summary shows title and item count only.
- [x] 5.2 Relocate the theme picker into the status bar (new dropdown over the four `ThemeKey`
      variants, calling `LibraryController::set_theme`)
- [x] 5.3 Relocate the activity indicator into the status bar (moved from the sidebar footer,
      same glyph/tooltip logic, click still toggles the activity panel)
- [x] 5.4 Relocate the notification indicator into the status bar. Maps onto the existing alert
      history panel (bell icon, red dot badge from `activity_recent_error_count`) — this app has no
      separate notification-inbox capability; reuses the existing entry point rather than inventing
      a new one.

## 6. Verification

- [ ] 6.1 Verify browsing-state requirements from `rust-main-window-library-layout` still hold
- [ ] 6.2 Verify tab overflow, popover/tab distinction, and status bar sync with sidebar/catalog
      state
