## 1. Alert: Replace Notification Banner

- [x] 1.1 Add `gpui_component::alert::{Alert, AlertVariant}` import to `notification_banner_view.rs`
- [x] 1.2 Replace the hand-rolled `div()` banner rows with one `Alert` per `Notice`, using `AlertVariant::Warning`, title from `NoticeKind`, and `on_close` wired to `AuthStateController::dismiss_notice`
- [x] 1.3 Add the action button as a child element inside the `Alert` (using `Alert`'s `ParentElement` impl or equivalent)
- [x] 1.4 Delete the hand-rolled `div()` banner construction code

## 2. Avatar: Replace Avatar Circle Helper

- [x] 2.1 Add `gpui_component::avatar::Avatar` import to `settings_account_view.rs`
- [x] 2.2 Replace `render_avatar_circle` with `Avatar` using `.image(...)` when `auth.avatar_bytes` is `Some` and `.label(initial)` as fallback
- [x] 2.3 Delete the `render_avatar_circle` function

## 3. DescriptionList: Replace Metadata Table

- [x] 3.1 Add `gpui_component::description_list::{DescriptionList, DescriptionItem}` import to `detail_panel_view.rs`
- [x] 3.2 Replace `render_metadata_table` body with a `DescriptionList` using `Axis::Horizontal`, `bordered(true)`, and `DescriptionItem::item(label, value)` for each metadata row
- [x] 3.3 Verify the date-added row (with tooltip) still renders correctly — add it as a `DescriptionItem` with a custom `AnyElement` value if the tooltip cannot be applied directly

## 4. Badge: Download Status on Catalog Cards

- [x] 4.1 Add `gpui_component::badge::Badge` import to `catalog_view.rs`
- [x] 4.2 In `render_thumb_row`: wrap the card element with `Badge::new().dot().color(green)` for `Downloaded` items; no badge for `Cloud` items
- [x] 4.3 In `render_grid_card`: apply the same `Badge` wrapping as for thumb rows

## 5. Spinner: Catalog Loading State

- [x] 5.1 Add `is_loading() -> bool` method to `LibraryController` that returns `true` when the catalog is empty and the background fetch task is still running (gate on an existing `catalog_loading` bool or introduce one)
- [x] 5.2 Add `gpui_component::spinner::Spinner` import to `catalog_view.rs`
- [x] 5.3 In the catalog render path, check `controller.read(cx).is_loading()` and render a centred `Spinner::new().with_size(Size::Large)` instead of the table/thumb/grid content when true

## 6. Pagination: Controller Model

- [x] 6.1 Add `current_page: usize` (default 1) and `page_size: usize` (default 25) fields to `LibraryController`
- [x] 6.2 Add `set_page(page: usize, cx)` method that clamps to `1..=total_pages()` and emits `LibraryChanged`
- [x] 6.3 Add `set_page_size(size: usize, cx)` method that accepts values from `[10, 25, 50, 100, 200]`, resets `current_page` to 1, and emits `LibraryChanged`
- [x] 6.4 Add `total_pages() -> usize` method: `(visible_items_count() + page_size - 1) / page_size` (returns 1 when count is 0)
- [x] 6.5 Update `visible_items_slice` (or add a `visible_page_items` method) to return only the current page window
- [x] 6.6 Reset `current_page` to 1 in `set_filter` and `set_search_query`
- [x] 6.7 Persist `page_size` to the preference file on change; restore it on startup

## 7. Pagination: View Component

- [x] 7.1 Add `gpui_component::pagination::Pagination` import to `catalog_view.rs`
- [x] 7.2 In the catalog render function, render a `Pagination::new("catalog-pagination").current_page(...).total_pages(...).on_click(...)` footer below the catalog content, hidden when `total_pages() == 1`
- [x] 7.3 Wire the `on_click` handler to `entity.update(cx, |ctrl, cx| ctrl.set_page(*page, cx))`

## 8. Popover: Page Size Picker

- [x] 8.1 Add `gpui_component::popover::Popover` and `gpui_component::button::{Button, ButtonVariants}` imports to `catalog_view.rs` (if not already present)
- [x] 8.2 In the pagination bar, add a `Popover::new("page-size-picker")` with a trigger `Button` showing the current page size (e.g. "25 per page")
- [x] 8.3 Add content to the popover: five `Button` items for each size option (10, 25, 50, 100, 200); each button's click handler calls `entity.update(cx, |ctrl, cx| ctrl.set_page_size(n, cx))`

## 9. Toast Notifications

- [x] 9.1 Add `DownloadComplete { title: Arc<str> }` and `DownloadError { title: Arc<str>, message: String }` variants to the `ActivityController` event enum (or create a new event type)
- [x] 9.2 Emit `DownloadComplete` when an activity item transitions from `InProgress` to `Complete` in `ActivityController`
- [x] 9.3 Emit `DownloadError` when an activity item transitions to `Error` in `ActivityController`
- [x] 9.4 Call `gpui_component::notification::init(cx)` in `app/mod.rs` application startup
- [x] 9.5 Add `Entity<NotificationList>` field to `LibraryRootView`; create it in `LibraryRootView::new`
- [x] 9.6 Subscribe to `ActivityController` events in `LibraryRootView::new`; on `DownloadComplete` push a `Notification::new().message(...).type_(NotificationType::Success).autohide(true)` to the list; on `DownloadError` push `NotificationType::Error`
- [x] 9.7 Render the `NotificationList` entity as a child of the root view (positioned as an overlay per the component's requirements)

## 10. Resizable Layout

- [x] 10.1 Add `gpui_component::resizable::{h_resizable, resizable_panel, ResizableState}` import to `root_view.rs`
- [x] 10.2 Add `Entity<ResizableState>` field to `LibraryRootView` for the main layout state
- [x] 10.3 Replace the outer fixed-width column `div()` in the `Render` impl with `h_resizable("main-layout")` using the stored `ResizableState` entity
- [x] 10.4 Add a `resizable_panel()` for the sidebar with `size_range(px(180.)..=px(360.))` and initial size `px(250.)`
- [x] 10.5 Add a `resizable_panel()` for the catalog content (unconstrained)
- [x] 10.6 Add a `resizable_panel()` for the detail panel with `size_range(px(240.)..=px(480.))`, initial size `px(320.)`; render only when an item is selected (size `px(0.)` otherwise)
- [x] 10.7 Subscribe to `ResizablePanelEvent::Resized` and persist sidebar and detail panel sizes to the preference file
- [x] 10.8 Restore persisted panel sizes on startup before constructing `ResizableState`

## 11. Build and Lint

- [x] 11.1 Run `cargo check --workspace` — no errors
- [x] 11.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no warnings

## 12. Manual Verification

- [ ] 12.1 Auth warning banner renders correctly with Alert styling and dismiss/action buttons work
- [ ] 12.2 Account settings avatar shows Gravatar image (or initial letter fallback)
- [ ] 12.3 Detail panel metadata renders as a DescriptionList with correct label/value pairs and date tooltip
- [ ] 12.4 Downloaded items in thumbs/grid views show a badge; cloud items do not
- [ ] 12.5 Catalog shows spinner when loading (empty catalog) and transitions to content once items arrive
- [ ] 12.6 Pagination bar appears when item count exceeds page size; clicking page numbers navigates correctly
- [ ] 12.7 Page size picker opens a popover with five options; selecting one updates the visible item count
- [ ] 12.8 Selected page size persists after app restart
- [ ] 12.9 Download complete and download error events show toast notifications that auto-dismiss
- [ ] 12.10 Main window columns are draggable; sidebar and detail panel respect their min/max bounds
- [ ] 12.11 Panel sizes persist after app restart
