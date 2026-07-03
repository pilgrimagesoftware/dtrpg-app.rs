## Why

The app name ("Libri") renders twice: once in `title_bar_view.rs`'s `render_title_bar`,
positioned directly under the macOS traffic-light window controls where a title bar
belongs, and again in `sidebar_view.rs`'s `build_header` as a `SidebarHeader` wordmark at
the top of the sidebar column, directly below the title bar. The sidebar wordmark is
redundant now that the title bar (added per `add-rust-main-window-structure`) already
shows the app name in the conventional location.

## What Changes

- `SidebarHeader`'s wordmark ("Libri") is removed from `sidebar_view.rs`; the sidebar's
  top section starts directly with the smart-filter navigation menu.
- The title bar remains the single place the app name is displayed, immediately below the
  window's traffic-light controls.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-main-window-structure`: The app name is rendered once, in the title bar; the
  sidebar no longer duplicates it in a header wordmark.

## Impact

- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: `build_header` no longer renders the
  `t!("sidebar.app_name")` wordmark; `Sidebar`'s header slot is either removed or replaced
  with minimal top padding to preserve spacing above the nav menu.
