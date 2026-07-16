## Context

`build_menus` (`crates/dtrpg-ui/src/ui/app/mod.rs`) constructs the entire native menu bar
from scratch on every call — `cx.set_menus` replaces the whole bar, not individual items —
so relocating the Select Tab submenu is purely a matter of moving its `MenuItem::submenu(...)`
block from the Window `Menu`'s `.items([...])` array to the View `Menu`'s. The submenu's
own construction (the `tab_label` closure and the ten `SelectTab0`..`SelectTab9` entries)
is untouched.

## Goals / Non-Goals

**Goals:**
- Select Tab submenu appears under View instead of Window, with identical contents,
  labels, enabled/disabled state, and checkmark behavior.
- Window menu's remaining items (Minimize, Zoom, Show Activity, Show Alert History) are
  unaffected other than losing the now-unnecessary trailing separator.

**Non-Goals:**
- No change to the `SelectTab0`..`SelectTab9` actions, their `cmd-0`..`cmd-9`
  keybindings, or `tab_target_at`'s position-resolution logic.
- No i18n key renames (`menu.window_select_tab_title` / `menu.window_select_tab_empty`
  keep their current keys despite the `window_` prefix now being placed under View) —
  renaming is a separate, purely-cosmetic follow-up not required for this change to be
  correct.
- No change to where in the View menu's item list the submenu is inserted beyond "after
  the existing Presentation/Sort/Find-in-Library items" — there's no functional reason to
  interleave it earlier.

## Decisions

### Move the whole `MenuItem::submenu(...)` block verbatim

The submenu's construction already lives inline inside `build_menus`, built from the same
`tab_label` closure and `tabs: &TabsSnapshot` parameter available at every call site
within the function. Moving the block to the View `Menu`'s `.items([...])` array requires
no new parameters, no new closures, and no change to `tab_label` — the closure is defined
once at the top of `build_menus` and is in scope for both menus.

_Alternative considered_: extract the submenu construction into a standalone helper
function returning `MenuItem`, called from wherever it's needed. Rejected as unnecessary
indirection for a single call site — `build_menus` already inlines every other submenu
(Presentation, Sort) the same way.

### Leave i18n keys as-is

Renaming `menu.window_select_tab_title` to something like `menu.view_select_tab_title`
would touch three locale files (`en.yaml`, `de.yaml`, `fr.yaml`) for a label the user
never sees (`window_`/`view_` is purely the constant's namespace prefix, not
user-facing text) and adds no behavioral value. Deferred to a separate cleanup change if
desired.

## Risks / Trade-offs

- [Risk] Losing track of the separator that currently precedes the Select Tab submenu in
  the Window menu, leaving a dangling trailing separator after Show Alert History. ->
  Mitigation: remove that separator along with the submenu move, per the proposal's
  "Impact" section.
- [Risk] None identified for the View-menu side — the submenu is simply appended after
  existing items, no interaction with the Presentation/Sort checkmark logic.
