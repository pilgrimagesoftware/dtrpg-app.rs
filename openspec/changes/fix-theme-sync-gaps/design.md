## Context

`gpui_component::Theme` (a separate `gpui::Global` from this app's own `LibriTheme`) is what most `gpui-component` widgets actually read colors from — `Button` (unless given a `ButtonCustomVariant`), `Input`, `Popover`/`PopupMenu`/dropdown menus, tooltips, scrollbars, and `Sidebar`/`SidebarMenu` all read `cx.theme()` internally, not this app's `ColorTokens`. `gpui_component::Theme::colors` has on the order of 100 semantic fields (`button*` ×24 across 6 variants, `input`, `popover`/`popover_foreground`, `scrollbar`/`scrollbar_thumb`/`scrollbar_thumb_hover`, `sidebar`/`sidebar_accent`/`sidebar_accent_foreground`/`sidebar_border`/`sidebar_foreground`/`sidebar_primary`/`sidebar_primary_foreground`, plus base semantic fields like `background`/`foreground`/`border`/`muted`/`muted_foreground`/`primary`/`secondary`/`danger`/`warning`/`info`/`success`/`ring`/`selection`/`caret`/`drag_border`/`drop_target`/`description_list_label`/`description_list_label_foreground`, and a mirrored `.tokens` sub-struct used by newer widgets like `DataTable`).

Today, `apply_table_colors` (`data/theme.rs`) sets only 10 `table*` field pairs (`.colors` and `.tokens`), called from `app::setup` at startup and from `LibraryController::set_theme` on every theme change. Everything else on `gpui_component::Theme` keeps whatever `Theme::apply_config` computed for the ambient light/dark mode at construction — unrelated to whichever of the six Libri themes (soon eight, per `settings-appearance-fonts`) is actually active. This is why the sidebar, every default-styled button, every input field, every popover/dropdown/tooltip, and scrollbars visibly ignore theme changes while the catalog table (already fixed) and anything reading `ColorTokens` directly (most custom-styled views) do not.

Confirmed via codebase audit (not guessed): `sidebar_view.rs` and `ui/library/drag.rs` import no `ColorTokens`/`LibriTheme` at all — 100% `cx.theme()`. 26 of 31 `Button::new(...)` call sites across the app have no `.custom(...)` override. Both hardcoded `gpui::hsla(0.08, 0.9, 0.55, 1.0)` amber literals (`settings_file_openers_view.rs:206`, `settings_storage_view.rs:168`) sit in functions that already have `colors.warning_text` in scope and use it elsewhere in the same function — these are copy-paste regressions from before `ColorTokens.warning_text` existed, not intentional. `ui/windows/app.rs`'s `AppWindow` (hardcodes its own `rgb(...)` literals) is never constructed anywhere in the crate — confirmed dead code, not a real theme gap.

## Goals / Non-Goals

**Goals:**

- A single, general theme-sync function covering `gpui_component::Theme`'s base semantic fields plus `button*`/`input`/`popover*`/`scrollbar*`/`sidebar*`, called from the same two existing call sites (`app::setup`, `LibraryController::set_theme`) — no new call sites, no per-view rewrites.
- Fix the two hardcoded warning-color literal regressions.
- Remove the dead `AppWindow` scaffold rather than theme-wiring unreferenced code.

**Non-Goals:**

- Rewriting `sidebar_view.rs`, `drag.rs`, or any individual `Button`/`Input`/`Popover` call site to read `ColorTokens` directly — the whole point of fixing the sync function is that these don't need to change; they already read `cx.theme()` correctly.
- Changing `ui/library/cover.rs`'s generative cover-art palette — intentionally independent of the active theme (per that file's own doc comment), not a bug.
- A perfect 1:1 semantic mapping for every gpui-component field — `ColorTokens` has far fewer distinct roles (no separate info/success/primary/secondary hues) than `gpui_component::Theme` does (see Decisions).
- Persisting or exposing any new user-facing setting — this is a bugfix to existing theme-switching behavior, not a new capability.

## Decisions

### One general sync function, replacing/absorbing `apply_table_colors`

Rename `apply_table_colors` to `apply_theme_colors` (or keep the name and broaden its body — either way, one function, not several narrowly-scoped ones per widget family) since it's called from exactly the same two places regardless of scope. Structure the body as a flat list of `theme.colors.<field> = colors.<token>;` / `theme.tokens.<field> = colors.<token>.into();` pairs, grouped by widget family with a comment header per group (buttons, input, popover, scrollbar, sidebar, base semantic), mirroring the existing table-field block's style.

### Mapping `ColorTokens`'s smaller token set onto `gpui_component::Theme`'s larger one

`ColorTokens` has one accent hue and one error/warning pair — no separate primary/secondary/info/success hues the way `gpui_component::Theme` does. Where there's no dedicated Libri token, reuse the semantically closest one rather than leaving the field on its light/dark default:

| `gpui_component::Theme` field(s) | `ColorTokens` source |
|---|---|
| `background`, `popover` | `surface` |
| `foreground`, `popover_foreground` | `text_primary` |
| `border`, `sidebar_border`, `input` | `border` |
| `muted` | `surface_alt` |
| `muted_foreground` | `text_tertiary` |
| `button`, `sidebar` | `surface_alt` |
| `button_foreground`, `sidebar_foreground` | `text_primary` |
| `button_hover` | `hover` |
| `button_active`, `sidebar_accent`, `list_active` | `accent_soft` |
| `primary`, `button_primary`, `sidebar_primary`, `info`, `button_info` | `accent` |
| `primary_foreground`, `button_primary_foreground`, `sidebar_primary_foreground`, `info_foreground`, `button_info_foreground` | `accent_on` |
| `primary_hover`, `button_primary_hover`, `info_hover`, `button_info_hover` | `accent` (same as base — `ColorTokens` has no separate hover shade for accent) |
| `secondary`, `button_secondary` | `surface_alt` |
| `secondary_foreground`, `button_secondary_foreground`, `sidebar_accent_foreground` | `text_secondary` |
| `danger`, `button_danger` | `error` |
| `danger_foreground`, `button_danger_foreground` | `accent_on` (the app's existing "text on a saturated background" token — no separate `error_on`) |
| `warning`, `button_warning` | `warning_bg` |
| `warning_foreground` (if present), `button_warning_foreground` | `warning_text` |
| `success`, `button_success` | `accent` (no dedicated success green in `ColorTokens`; accent is the closest "positive/active" hue) |
| `success_foreground`, `button_success_foreground` | `accent_on` |
| `scrollbar` | `surface_alt` |
| `scrollbar_thumb` | `border_strong` |
| `scrollbar_thumb_hover` | `text_tertiary` |
| `ring` | `accent` |
| `selection` | `accent_soft` |
| `drag_border` | `accent` |
| `drop_target` | `accent_soft` |
| `description_list_label` | `surface_alt` |
| `description_list_label_foreground` | `text_secondary` |

Active/hover shade fields that `ColorTokens` has no distinct value for (e.g. `button_danger_active`, `button_danger_hover`) reuse the base field's source rather than being left unsynced — a flat color on hover/active is a smaller visual defect than falling back to a mismatched-theme default, and matches how `accent_soft`/`accent` already double up for both "active" and "hover" states in the button mapping above.

_Alternative considered:_ add new dedicated tokens to `ColorTokens` (e.g. `info`, `success`) so the mapping could be 1:1. Rejected for this change — expanding `ColorTokens` touches all eight (six existing + two from `settings-appearance-fonts`) palette definitions and is a larger, separate concern from "theme changes don't reach these widgets"; the reuse mapping above is a complete, if imperfect, fix for the actual bug report.

### Fix the two warning-color literals directly, no abstraction needed

Both are one-line diffs — replace `gpui::hsla(0.08, 0.9, 0.55, 1.0)` with the `colors.warning_text` (or `warning_text` local binding, matching whichever is already in scope at each call site) variable already used elsewhere in the same function. No design decision beyond "use the token that's already there."

### Delete `AppWindow` rather than theme-wire it

It hardcodes its own colors and is never constructed anywhere in the crate (confirmed via grep for any `AppWindow::`/`AppWindow {` construction). Wiring dead code into the theme system would be effort spent on something no user ever sees; deleting it is strictly simpler and removes a maintenance trap (a future contributor might otherwise "fix" it under the impression it's live).

## Risks / Trade-offs

- **[Risk]** The reuse-mapping table means some semantically distinct gpui-component states (e.g. "info" vs "primary" buttons) render identically in this app, since `ColorTokens` doesn't distinguish them. → Acceptable: this app doesn't currently use `button_info`/`button_success` variants anywhere (confirmed no `.info()`/`.success()` `Button` calls in the codebase), so the mapping exists for completeness/future-proofing, not because it's user-visible today.
- **[Risk]** A ~35-field mapping function is easy to leave subtly incomplete (miss a field gpui-component adds in a future version bump). → Mitigation: grouped by widget family with comments, matching the existing table block's structure, so a future addition has an obvious place to slot in; not solvable generically without also owning `gpui-component`'s theme struct.
- **[Trade-off]** This fixes existing widget usage but doesn't prevent a future view from introducing a new hardcoded color literal (like the two being fixed here). → Out of scope for a bugfix change; a lint/review-checklist concern, not a code-level one.
