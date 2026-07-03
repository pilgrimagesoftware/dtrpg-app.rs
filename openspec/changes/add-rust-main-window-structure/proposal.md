## Why

`dtrpg-app` now defines `shared-main-window-structure`, replacing the disclosable search/filter
strip and content-area account menu (`rust-main-window-library-layout`) with a title bar, a
persistent sidebar, tabbed content, and a status bar. The Rust GPUI app needs a child change
mapping that structure to gpui-components, reusing the sidebar, activity panel, and account menu
work already in progress rather than restarting it.

## What Changes

- Add a title bar view above the content area with a horizontal separator, the window title, and
  an account button (existing avatar/account-menu work from `avatar-menu-user-info` and
  `user-avatar-with-logout`) that adds a sign-out action.
- Extend the existing sidebar (`sidebar-collections-and-collapsible`, `sidebar-section-counts`,
  `use-gpui-sidebar-components`) with default navigation section counts; Collections and
  Publishers sections carry forward unchanged.
- Replace the catalog's disclosable filter strip with a tab strip: a non-closable catalog tab
  first, using `gpui-component`'s `Tab`/`TabBar` primitives with an overflow "more" menu, and
  closable expanded detail tabs.
- Move the catalog's search, sort, and view mode controls (`catalog-menu`,
  `sort-menu-group-toggle`, existing view-mode controls) into the catalog tab's own header.
- Add single-click popover detail (using `gpui-component`'s `Popover`) and double-click expanded
  detail tab behavior for catalog items, reusing `detail-panel-resizable-and-wrapping` and
  `multi-item-catalog-entry-detail` content for the expanded tab's thumbnail, attributes, and file
  list.
- Add a status bar view consolidating: total item count/size, active-tab summary, a theme picker
  (extending `add-theme-selector`), an activity indicator (extending `activity-panel-improvements`),
  and a notification indicator (extending `alert-history-view`).
- Reference the gpui-components gallery demo (`gpui-component` crate's `gallery` example) as the
  interaction model for tab strip, status bar, and popover primitives.

## Capabilities

### New Capabilities

- `rust-main-window-structure`: Defines the GPUI-specific title bar, sidebar, tabbed content
  area, and status bar for the Rust desktop app.

### Modified Capabilities

- `rust-main-window-library-layout`: Retires the disclosable search/filter strip and content-area
  account menu requirements in favor of `rust-main-window-structure`; browsing-state and
  presentation requirements are unaffected.

## Impact

- `dtrpg-app/rust/openspec`: Adds `rust-main-window-structure`, retires superseded parts of
  `rust-main-window-library-layout`.
- Affected code: `finder`/`rust` GPUI shell modules for the main window, sidebar, catalog view,
  and status bar (exact module paths determined during implementation planning).
- Depends on `dtrpg-app/openspec/changes/add-shared-main-window-structure`.
- Builds on in-progress work: `sidebar-collections-and-collapsible`, `use-gpui-sidebar-components`,
  `adopt-gpui-component-primitives`, `activity-panel-improvements`, `alert-history-view`,
  `avatar-menu-user-info`, `add-theme-selector`, `sort-menu-group-toggle`.
