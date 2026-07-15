## Context

`open_library_window` (`ui/app/mod.rs`) currently opens the library window with `WindowOptions::default()` — no `window_bounds` set, so GPUI falls back to its own default placement (`default_bounds`, roughly centered on the primary display at a fixed size) every launch. The settings window, by contrast, already has an `on_window_should_close` hook (used today only to sync `SettingsController`'s open/closed flag) and, as of `settings-appearance-fonts`, explicitly centers itself via `WindowBounds::centered` on every open — that window is intentionally fixed-size and re-centered, not a candidate for position/size memory.

`UiPrefs` (`data/ui_prefs.rs`) is the existing mechanism for this kind of small layout preference: a flat `UiPrefsFile` TOML struct, loaded fresh and flushed on every mutation, already storing `sidebar_width`/`detail_width`/`settings_page_ix` and (per `settings-appearance-fonts`) `theme_key`/font selections. This change follows that exact pattern rather than introducing new persistence machinery.

GPUI represents a window's placement as `WindowBounds::Windowed(Bounds<Pixels>)`, where `Bounds { origin: Point<Pixels>, size: Size<Pixels> }`. `Window::bounds()` returns the window's current bounds; `Window::on_window_should_close` fires with `&mut Window` access immediately before the window closes — the same hook the settings window already uses to sync `SettingsController::close`.

## Goals / Non-Goals

**Goals:**

- The library window reopens at the position and size it was last closed at, instead of always resetting to the default placement.
- If the saved bounds don't intersect any currently connected display (external monitor unplugged, laptop moved to a different desk setup), fall back to the default placement rather than opening off-screen or spanning a monitor that no longer exists.

**Non-Goals:**

- The settings window is out of scope — it's fixed-size and always centers on open by design (see `settings-appearance-fonts`), with nothing meaningful to remember.
- Maximized/fullscreen state is not tracked — only `WindowBounds::Windowed` bounds are persisted; if the user quits while maximized, this change persists whatever `Window::bounds()` reports at that point (the OS-restored windowed bounds, not the maximized state), which is an acceptable simplification for a first pass.
- No live "save on every drag" — bounds are captured once, at window-close time, not continuously while resizing/dragging (see Decisions).

## Decisions

### Capture bounds at `on_window_should_close`, not continuously

GPUI has no dedicated "window moved" or "window resized" event — a resize does trigger a relayout/render pass (so bounds *could* be sampled from `Render::render`), but a pure move (no size change) doesn't necessarily trigger any render at all, making "sample bounds every render" an unreliable way to catch a plain drag. `on_window_should_close` is the one point guaranteed to fire before the window goes away, already used by `open_settings_window` for exactly this "do something right before close" shape, and it hands back `&mut Window` so `window.bounds()` can be read directly.

_Alternative considered:_ sampling `window.bounds()` on every `Render::render` pass (mirroring this codebase's existing `on_prepaint`-based bounds-capture pattern for `entry_bounds`/`ItemListDelegate::table_width`). Rejected as the primary mechanism — a plain window move without any content change may never trigger a render pass, so this would miss most drags. Capture-on-close covers the actual goal (remember where the user left it) with one hook instead of a per-frame write.

### Persist through `UiPrefs`, as a nested `WindowBoundsPref` struct

Adds one field to `UiPrefsFile`:

```rust
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct WindowBoundsPref {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
```

with `library_window_bounds: Option<WindowBoundsPref>` on `UiPrefsFile`, and `UiPrefs::library_window_bounds() -> Option<WindowBoundsPref>` / `UiPrefs::save_library_window_bounds(bounds: WindowBoundsPref)` following the existing getter/setter shape used for every other `UiPrefs` field.

_Alternative considered:_ a raw `(f32, f32, f32, f32)` tuple field. Rejected — a named struct documents which number is which in the TOML file and in code, at negligible extra cost.

### Validate against connected displays before restoring

At startup, `open_library_window` resolves the persisted bounds like this:

1. Load `UiPrefs`; if no `library_window_bounds` was ever saved, use the default (`WindowOptions::default()`'s behavior — no explicit `window_bounds`, same as today).
2. If saved bounds exist, build a `Bounds<Pixels>` from them and check `bounds.intersects(&display.bounds())` for at least one display in `cx.displays()`.
3. If it intersects some display, restore it (`WindowBounds::Windowed(bounds)`). If it doesn't (the display it was on is no longer connected), fall back to the default placement — same as the "never saved" case.

This mirrors `Bounds::centered`'s existing fallback-to-origin behavior when no display is found, just applied to a stored preference instead of a freshly computed centered position.

### Save on close, not on every `on_app_quit`

`App::on_app_quit` exists as an app-wide "about to quit" hook, and could theoretically read bounds from the already-tracked `LibraryWindowHandle` global. It's not used here: `on_window_should_close` already fires for the normal window-close path (including Cmd+Q on macOS, which closes open windows as part of quitting), so adding a second save path would just be redundant bookkeeping for the same value. A hard kill (`kill -9`, crash) won't persist the final bounds either way — an accepted limitation shared with every other `UiPrefs` field, none of which are flushed on unclean termination.

## Risks / Trade-offs

- **[Risk]** A window resized to be larger than a subsequently-connected smaller display could restore partially off-screen if it still technically "intersects" that display. → Accepted: `intersects` (any overlap) rather than "fully contained" is the deliberately looser check — it means "was this generally usable" rather than "pixel-perfect placement guaranteed," matching how most desktop apps handle this (they don't hard-clamp either).
- **[Risk]** Maximized-state isn't tracked, so a user who always quits maximized never sees that restored, only the windowed bounds underneath. → Accepted per Non-Goals; can be a fast follow if requested.
