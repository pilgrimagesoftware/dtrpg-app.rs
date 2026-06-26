## Context

The settings panel views were scaffolded with static placeholder elements: a non-editable extension badge, a hardcoded stub in the Add button, and a read-only storage path div. gpui's `div()` builder produces non-interactive text by default. Making these controls interactive requires either gpui's built-in `TextInput` (from `gpui-component`) or a native OS file dialog for the app picker.

The `FileOpenerConfig` data model already supports `add()`, `remove()`, and `update_app_path()` — no model changes are needed. `SettingsController` methods (`add_file_opener`, `remove_file_opener`) are already connected. The gap is entirely in the view layer.

## Goals / Non-Goals

**Goals:**
- The Add button in File Openers opens a real native application-picker dialog (`rfd::AsyncFileDialog`).
- After picking an app the user is prompted for an extension via an inline input field rendered inside the settings modal (not a second OS dialog).
- The extension field normalizes input (strip leading dot, trim, lower-case) before creating the `FileOpenerEntry`.
- The storage path field becomes an editable text input holding transient state.
- The storage warning line applies amber color and `⚠` prefix.

**Non-Goals:**
- Persisting the storage path (blocked on `catalog-storage-location`).
- In-place extension editing on existing entries (tracked separately; requires a rename in `FileOpenerConfig`).
- Windows or Linux file dialog variants (rfd handles these automatically via the same async API).

## Decisions

### Decision 1: Use `rfd::AsyncFileDialog` for the app picker

`rfd` (Rusty File Dialog) is an established cross-platform crate for native OS open/save dialogs. Its `AsyncFileDialog` variant does not block the UI thread. On macOS it produces a native NSOpenPanel. We add `rfd = { version = "0.15", features = ["async-std"] }` to `dtrpg-ui/Cargo.toml`.

**Alternative considered**: `nfd2` (older) — smaller maintenance community; rejected. Spawning `osascript` — fragile and non-portable; rejected.

The add flow becomes: click Add → spawn `AsyncFileDialog` on a background task → `cx.spawn()` / `cx.foreground_executor()` collects the result → update `SettingsController` with chosen path.

### Decision 2: Inline extension input rendered inside the modal (not a second OS dialog)

After the app picker resolves, we flip a boolean in `SettingsController` (or in the view's local state) to show an inline text field + Confirm/Cancel buttons replacing the normal File Openers list. This keeps the interaction inside the settings modal and avoids a second native dialog.

In gpui there is no persistent component-local state separate from the `Entity` model. We add `pending_add: Option<PendingAdd>` to `SettingsController`:

```rust
struct PendingAdd {
    app_path: PathBuf,
    extension_draft: String, // typed by user
}
```

`render_file_openers_section` checks `snapshot.pending_add` and renders either the normal list or the "enter extension" form.

**Alternative considered**: Prompt for the extension first (before the dialog). This inverts the natural flow (pick app → name the type it opens) and requires a custom text dialog before showing the file picker. Rejected.

### Decision 3: Storage path uses a gpui `div` with an `on_click` + local state held in `SettingsController`

gpui-component's `TextInput` is a full-featured input widget. For the storage path we need only a simple editable single-line field. We add `storage_path_draft: String` to `SettingsController` (initialized from the placeholder) and expose `set_storage_path_draft()`. The view renders the draft string via `TextInput` or a styled `div` with key-capture; `TextInput` from `gpui-component` is preferred if it is already in the dependency tree.

Given that gpui-component's input widget surface is already used elsewhere in the project (or is available), we use it. If the widget is not available, we implement a minimal single-line input using gpui's `on_key_down` on a focused `div` — but prefer `TextInput` to avoid reimplementing cursor/selection behavior.

### Decision 4: Warning text — inline style change only

The warning line in `render_storage_section` changes from `.text_color(text_tertiary)` to `.text_color(gpui::hsla(0.08, 0.9, 0.55, 1.0))` (the project-standard amber) and the string literal gains a `⚠ ` prefix. No new color token is introduced; the amber value is already established in `settings_file_openers_view.rs` for stale-app warnings.

## Risks / Trade-offs

**[Risk] `rfd` async integration with gpui's executor** — `rfd::AsyncFileDialog` returns a `Future`. gpui's `cx.spawn()` runs on gpui's async executor. We need `rfd` compiled with a compatible async runtime. `rfd = { version = "0.15", features = ["async-std"] }` uses async-std under the hood but the future itself is compatible with any executor via `poll`. If this causes runtime conflicts, use `rfd = { version = "0.15" }` (blocking variant) on a `cx.background_executor().spawn()` detached thread.

**[Risk] `pending_add` state in `SettingsController` adds model complexity** — `SettingsController` is a shared gpui `Entity`. Adding transient UI state (draft strings, pending add flow) mixes domain and UI concerns. The alternative is view-local state via `cx.view_state()`, but gpui does not natively support this pattern outside `Render::render`. Accepted trade-off: keep draft state in `SettingsController` for now, document it as UI-transient.

**[Risk] gpui-component `TextInput` availability** — If the crate does not expose a public `TextInput` element, fall back to a `div`-based implementation using `on_key_down` and explicit string mutation via `cx.update_view`. Investigate during implementation.

## Migration Plan

1. Add `rfd` to `dtrpg-ui/Cargo.toml`.
2. Add `pending_add: Option<PendingAdd>` and `storage_path_draft: String` to `SettingsController`; expose methods.
3. Update `SettingsSnapshot` to include `pending_add` and `storage_path_draft`.
4. Implement the "enter extension" inline form in `settings_file_openers_view.rs`.
5. Wire Add button to `rfd::AsyncFileDialog`; on pick, call `ctrl.begin_add(app_path, cx)`.
6. Update `settings_storage_view.rs`: path field → editable input; warning line → amber + `⚠`.
7. `cargo check -p dtrpg-ui` passes with zero errors.
