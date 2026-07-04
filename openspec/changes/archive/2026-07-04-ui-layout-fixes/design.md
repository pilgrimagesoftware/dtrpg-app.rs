## Context

Several unrelated bugs accumulated since the initial catalog-collections-improvements change landed. The bugs span layout, state persistence, service timing, and event handling. Each fix is small and localized.

## Goals / Non-Goals

**Goals:**
- Fix all eight bugs listed in the proposal with minimal blast radius per fix
- Persist new UI state (section collapse) through the existing `UiPrefs` mechanism
- Keep the layout change to a root_view restructure only

**Non-Goals:**
- Redesign the detail panel or settings account view beyond what's needed to show existing data
- Add any new data fields to the API response or SDK

## Decisions

### Collections load timing
`start_load` always runs against the unauthenticated service at construction time. The authenticated service arrives via `replace_service` after `startup_auth` succeeds. `load_collections` inside `start_load` was already removed to fix the "Not signed in" warning. The remaining issue ("collections still don't load") points to `replace_service` not being called, or `load_collections` in `replace_service` failing silently. Investigate: add a debug log in `replace_service` → `load_collections` to confirm the call reaches the authenticated service.

### Reload doubles catalog content
`reload_catalog` calls `start_load_inner(true)`. Inside, `append_catalog_page` from the disk cache runs first - it appends to the existing `self.catalog` (which still has the previous full load). Then the live fetch replaces via `set_catalog`. Fix: before the cache pre-population, clear `self.catalog` when `force_reload` is true. A `catalog.clear()` inside the spawn closure (on the controller) before `append_catalog_page` is the right location.

### File opener remove button
`render_file_openers_section` constructs a remove button with `ctrl.remove_file_opener(...)` on click. Need to trace whether the button's click handler is wired (check the actual button element for `.on_click`), and whether `remove_file_opener` is correctly implemented on the controller. This is a two-step debug: verify the handler is attached, then verify the controller method works.

### Account view / Avatar menu user info
`startup_auth` calls `set_logged_in(email, ...)` where `email` comes from `self.email_draft`, which is loaded from `ProfileConfig` at construction. If the user never saved an email (common case), `email` is `None`. The avatar menu then shows blank. Fix: after a successful API authentication, the API response may contain account info. Alternatively, the account view in settings should show the API key hint as the "account" identifier. Check what the auth API returns - if it returns a username/email, store it. If not, the settings Account tab should at minimum show the masked API key hint to confirm which account is logged in.

### Detail view close button
The close button is `.absolute()` inside a `.overflow_hidden()` container. Absolute children are clipped by `overflow_hidden` in gpui. Fix: change the outer container to `overflow_visible`, or move the close button outside the scroll container and use a z-index / stacking approach. Best: wrap the panel in a `relative` div, put the close button there (absolute within the outer wrapper), and let the inner scroll area use `overflow_hidden` separately.

### Catalog layout - remove right resizable panel
The `h_resizable` group in `root_view.rs` currently has three panels: sidebar, catalog, and (conditionally) detail. Remove the third `resizable_panel` for the detail. Instead, position the detail panel as a fixed-width absolute/overlay element within the catalog area, or use a flex row where the detail has `flex_none` and a fixed width. The simplest approach: switch from `h_resizable` with a conditional third panel to a flex row where the catalog gets `flex_1` and the detail (when present) is `flex_none` at a fixed width, no resize handle between them.

### Sidebar section collapse state
`SidebarMenuItem` accepts `.default_open(bool)` at construction. gpui-component's sidebar does not expose a collapse callback. The workaround: wrap the section header in an `on_click` handler that toggles and persists state, or use the `click_to_toggle` + initial `default_open` from a loaded preference. On each render, `default_open` is read from `UiPrefs`, and `UiPrefs` stores two new booleans: `collections_collapsed` and `publishers_collapsed`. Since `default_open` only sets initial state, a state machine in the controller or a listener on sidebar toggle events is needed for persistence. Check if `SidebarMenuItem` has a collapse callback; if not, intercept the section header click to toggle + save.

### Pagination First/Last buttons
`Pagination` from gpui-component has no first/last API. Add two `Button` elements flanking the `Pagination` widget. "First" dispatches `ctrl.set_page(1, cx)`, "Last" dispatches `ctrl.set_page(total_pages, cx)`. Both are disabled (`Button::disabled(true)`) when already on the target page.

## Risks / Trade-offs

- **Sidebar collapse persistence**: If `SidebarMenuItem` doesn't fire a callback on toggle, we can't intercept it without forking the component. Alternative: replace `click_to_toggle` with a manual open/close toggle driven entirely by controller state. This is more code but gives full control. → Investigate the `SidebarMenuItem` API first; if no callback, use the manual approach.
- **Detail panel layout change**: Removing the resize handle may surprise users who relied on it. Since the proposal explicitly requests this, it's intentional. The detail panel defaults to 320 px fixed width.
- **Catalog clear on force_reload**: Briefly shows an empty state while the live fetch is in flight. Accept this - it's honest feedback that a reload is happening. The `catalog_loading` flag ensures the loading spinner shows.

## Open Questions

- Does the DTRPG auth API return a username or email in its response that could populate the account view? If yes, capture it in `LoginTokens` and thread it through `set_logged_in`.
- Does `SidebarMenuItem` have a collapse/expand event we can subscribe to, or is `default_open` the only hook?
