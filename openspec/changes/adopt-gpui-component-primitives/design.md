## Context

The app has accumulated hand-rolled implementations of UI patterns that gpui-component covers: a warning banner, an avatar circle, a key-value metadata table, catalog loading state, item status indicators, and fixed-width layout columns. This design covers eight targeted substitutions plus two new features (pagination, toast notifications).

Components explicitly excluded from this design: `Sidebar` (`use-gpui-sidebar-components`), `Settings` (`use-gpui-settings-component`), `List` (no current use case).

## Goals / Non-Goals

**Goals**
- Drop hand-rolled code wherever a gpui-component primitive matches the use case.
- Add catalog pagination with user-configurable page size.
- Add toast notifications for download completion and error.
- Make the three-column layout user-resizable.

**Non-Goals**
- Changing what information is displayed (only how it is rendered).
- File Openers CRUD or any settings panel internals (covered by `use-gpui-settings-component`).
- Sidebar navigation internals (covered by `use-gpui-sidebar-components`).

## Decisions

### Alert replaces the notification banner

`render_notification_banner` builds a hand-rolled yellow strip with icon, message, action button, and dismiss. `Alert::new(id, message).variant(AlertVariant::Warning).title(...).on_close(...)` provides the same layout with theme-consistent colours. The action button slot uses `Alert`'s `Styled` impl to add a child button element.

Each `Notice` maps to one `Alert`. The existing `AuthStateController::dismiss_notice` and `open_settings` wiring is unchanged; only the rendering function changes.

### Avatar replaces the avatar circle helper

`render_avatar_circle` reproduces the gpui-component `Avatar` pattern: image with rounded-full clip if bytes are present, fallback initial letter otherwise. Replacing it with `Avatar::new().image(...)` or `Avatar::new().label(initial)` removes ~30 lines of custom layout code and picks up future upstream improvements.

### Badge wraps thumbs/grid cards

`Badge::new().dot()` with a green colour for `Downloaded` items is placed as the `Badge`'s child around the card element. `Badge` uses `position: relative` + an absolute overlay so no layout changes are needed in the card structure.

### DescriptionList replaces the metadata table

`render_metadata_table` iterates a `Vec<(&str, String)>` and builds a hand-rolled key-value table. `DescriptionList` takes `DescriptionItem::item(label, value)` entries with the same data. The `Axis::Horizontal` layout matches the existing two-column appearance. `bordered(true)` reproduces the divider lines.

### Pagination model lives in LibraryController

Adding `current_page: usize` (1-based) and `page_size: usize` to `LibraryController` keeps all filtered-result logic in one place. `visible_items_slice` is updated to return `items[page_start..page_end]` derived from the current page. `set_page` and `set_page_size` reset to page 1 when the filter changes.

Page size is one of the five preset values (10, 25, 50, 100, 200). The selected value is persisted via the existing settings/preferences mechanism (a new `catalog_page_size` key in the preference file).

### Popover provides the page-size picker

A `Popover` in the pagination bar wraps a list of five `Button` items for the page size choices. `Popover::new("page-size-picker").trigger(Button::new(...))` and a content builder that renders the option buttons covers this without a custom dropdown widget.

### Resizable layout via h_resizable

`root_view.rs` currently nests the sidebar, catalog, and detail panel as adjacent `div()` children. Replacing the outer container with `h_resizable("main-layout")` and three `resizable_panel()` children gives drag-to-resize behaviour automatically. Panel sizes are stored in `ResizableState`, which can be serialized and restored via the existing settings infrastructure (a new `panel_sizes` key).

- Sidebar panel: `size_range(px(180.)..=px(360.))`, default `px(250.)`
- Catalog panel: unconstrained (fills remaining space)
- Detail panel: `size_range(px(240.)..=px(480.))`, default `px(320.)`; only rendered when an item is selected. When hidden, the panel's initial size is set to `px(0.)` so it does not occupy space.

### Notification toasts from ActivityController events

`gpui_component::notification::NotificationList` is a gpui `Entity<NotificationList>` that accumulates `Notification` items and renders them as overlaid toasts. The root view subscribes to a new `ActivityController` event (`DownloadComplete(title)` and `DownloadError(title, message)`) emitted when the activity status for an item transitions from `InProgress` to `Complete` or `Error`.

The root view holds an `Entity<NotificationList>` and calls `notification_list.update(cx, |n, cx| n.push(Notification::new()...))` in the subscription handler.

_Tradeoff_: emitting new events from `ActivityController` is a minor expansion of its public API surface. The alternative (polling in the root view) would require duplicating state.

### Spinner for empty loading state

`LibraryController` already has sufficient state to detect the loading condition: `catalog.is_empty()` while the background task has not yet completed. A new `is_loading()` method returns `true` during this window. The catalog render function checks `is_loading()` and renders a centred `Spinner::new().with_size(Size::Large)` instead of the table/thumb/grid content.

## Risks / Trade-offs

- `ResizableState` serialization format must be stable; using a simple `Vec<f32>` keyed by panel name avoids versioning issues.
- `NotificationList` needs `gpui_component::notification` to be initialized (an `init(cx)` call, like other stateful gpui-component elements). This must be added to `app/mod.rs` app startup.
- `Badge` applies `position: relative` to its children container, which may interact with the existing `absolute`-positioned overlay in the detail panel close button. Verify no layout regressions after Badge adoption.
- Page size persistence ties `LibraryController` to a settings/preferences file. If the prefs file schema changes this could be a migration concern — use `Option<usize>` with a fallback default of 25 to make the field optional.

## Migration Plan

Changes can be implemented incrementally in any order since they are independent subsystems:
1. Alert + Avatar + DescriptionList + Badge + Spinner — pure rendering substitutions, no data model changes.
2. Pagination — requires `LibraryController` changes; update controller first, then view.
3. Toast notifications — requires new `ActivityController` events; add events, then subscribe in root view.
4. Resizable layout — requires `root_view.rs` structural change; do last as it touches the most code.

## Open Questions

- Should page size persist globally (all users of the same binary) or per-window? Current assumption: global via the preference file.
- Should `ResizableState` sizes be stored in the preference file or in a separate layout state file?
