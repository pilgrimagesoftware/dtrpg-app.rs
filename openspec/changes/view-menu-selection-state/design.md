## Context

The View menu's Presentation (List/Thumbs/Grid), Sort (Title/Publisher/Date
Added/Pages, Ascending/Descending), and Group-by-Publisher items were added to
`ui/app/mod.rs` and wired to `LibraryController` in a previous pass, but the menu bar was
only ever built once at startup with `cx.set_menus([...])` hardcoded inline. There was no
mechanism to update it, and no menu item ever showed a checkmark.

## Goals / Non-Goals

**Goals:**
- Make the native menu bar an accurate reflection of `LibraryController`'s current view
  state at all times.
- Keep the fix centered on the existing `LibraryChanged` event, which the controller
  already emits on every state change relevant here (presentation, sort, grouping).

**Non-Goals:**
- Fixing the separate, still-open issue where some View menu actions read as disabled in
  certain focus states (root-caused but not fixed; see `catalog-menu`/`window-menu`
  history and the app's own notes for the mechanism).
- Any other menu (Catalog, Window, Help) — those are unaffected by this change.

## Decisions

**Rebuild the whole bar on every relevant change, rather than patching in place**

`gpui::App::set_menus` replaces the entire menu bar; there is no partial-update API. Since
`LibraryChanged` already fires on every state change this menu depends on, rebuilding is
cheap (menu construction is a handful of allocations) and keeps a single source of truth
(`build_menus`) instead of two divergent code paths for "initial" vs. "updated" menus.

**`ViewMenuState` as a plain snapshot struct, not a reference to the controller**

`build_menus` is a free function taking `&ViewMenuState` rather than
`&LibraryController`, so it has no dependency on the controller's internal representation
and can be called from `setup` (before any controller exists) with `Default::default()`.

**Column-driven `SortMethod::Custom { col_key }` normalized back to named variants**

Clicking a `DataTable` column header produces `SortMethod::Custom { col_key: "publisher" }`
etc. rather than the named `SortMethod::Publisher` the menu's four sort items represent.
`build_menus` normalizes these back so the checkmark still tracks column-driven sorts,
rather than going blank whenever the user sorts via a column header instead of the menu.

## Risks / Trade-offs

[Menu rebuilt on every `LibraryChanged`] could become a cost center if that event starts
firing at high frequency for unrelated reasons → Mitigation: menu construction is cheap
(no I/O, no allocation beyond `Vec<Menu>`); revisit only if profiling shows otherwise.

## Migration Plan

No migration needed — this is additive UI behavior with no data model or API surface
involvement.
