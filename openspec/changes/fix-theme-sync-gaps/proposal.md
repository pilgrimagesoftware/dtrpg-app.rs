## Why

Switching the active Libri theme only visibly changes some of the app. `apply_table_colors` (`data/theme.rs`) — the one place that pushes `LibriTheme`'s colors onto `gpui_component::Theme`, the separate global most `gpui-component` widgets actually read their colors from — only syncs the `table*` fields, since it was written specifically to fix the catalog `DataTable`. Every other `gpui-component` widget (buttons without a custom variant, inputs, popovers/dropdown menus, tooltips, scrollbars, and the entire sidebar) keeps whichever light/dark default `gpui_component::Theme::apply_config` set at startup, regardless of which Libri theme is active. Two settings-page warning labels also hardcode a color literal instead of using the `warning_text` token already in scope.

## What Changes

- Extend the table-only color sync into a full sync covering `gpui_component::Theme`'s `button*`, `input`, `popover`/`popover_foreground`, `scrollbar*`, and `sidebar*` fields (both the `.colors` and `.tokens` sub-structs, matching the existing table-field pattern) — called at the same two points `apply_table_colors` already is (app startup, and every `set_theme`), so no new call sites are needed. This is why the fix is a color-mapping change, not a per-view rewrite: `sidebar_view.rs`, `drag.rs`, and every unstyled `Button`/`Input`/`Popover`/tooltip already read `cx.theme()` — they just need that global to actually carry the active Libri palette.
- Fix two hardcoded amber-warning color literals (`gpui::hsla(0.08, 0.9, 0.55, 1.0)`) in `settings_file_openers_view.rs` and `settings_storage_view.rs` to use the `colors.warning_text` token already in scope at both call sites (already used one call earlier in `settings_storage_view.rs`).
- Remove the unreferenced `AppWindow` scaffold (`ui/windows/app.rs`), which hardcodes its own `rgb(...)` literals and isn't constructed anywhere in the app — dead code, not a real theme gap.

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `libri-theme`: theme changes now visibly apply to the sidebar, all buttons/inputs/popovers/tooltips/scrollbars, and the two settings-page warning labels, not just table-backed views.

## Impact

- `dtrpg-ui/src/data/theme.rs`: `apply_table_colors` becomes (or gains a sibling covering) the `button*`/`input`/`popover*`/`scrollbar*`/`sidebar*` field mapping from `ColorTokens`.
- `dtrpg-ui/src/ui/views/settings_file_openers_view.rs`, `settings_storage_view.rs`: one-line literal-to-token fixes.
- `dtrpg-ui/src/ui/windows/app.rs` (and its `mod` declaration): removed.
- No changes to `sidebar_view.rs`, `drag.rs`, or any individual `Button`/`Input`/`Popover` call site — they already read `cx.theme()` and pick up the fix automatically once that global carries the right colors.
- No changes to `ui/library/cover.rs`'s generative cover-art colors — those are intentionally derived from per-item metadata, not the active theme (documented in that file already), and are out of scope here.
