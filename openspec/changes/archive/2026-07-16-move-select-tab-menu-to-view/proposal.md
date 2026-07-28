## Why

The "Select Tab" submenu (ten items for jumping to Catalog / open detail tabs by
position, mirroring the `cmd-0`..`cmd-9` shortcuts) currently lives in the Window menu.
Window is the wrong home for it: on macOS, "Window" is conventionally reserved for
window-management actions (Minimize, Zoom, and the OS-managed list of open windows), not
in-window navigation. Tab selection is a view-level concern — it belongs alongside the
View menu's other catalog navigation controls (Presentation, Sort, Find in Library).

## What Changes

- Move the "Select Tab" submenu (and its ten `SelectTab0`..`SelectTab9` menu items) out
  of the Window menu and into the View menu in `build_menus`.
- The Window menu keeps Minimize, Zoom, Show Activity, and Show Alert History; it loses
  the Select Tab submenu and the separator that preceded it.
- The View menu gains the Select Tab submenu, placed after its existing Presentation/Sort
  submenus and "Find in Library" item.
- No change to the submenu's contents, labels, enabled/disabled state, checkmark
  behavior, or the underlying `SelectTab0`..`SelectTab9` actions and `cmd-0`..`cmd-9`
  keybindings — this is a menu-placement change only.

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `window-menu`: Removes the "Window menu contains a Select Tab submenu" requirement —
  the Window menu no longer holds tab-selection items.
- `catalog-tab-cmd-number-shortcuts`: The requirement describing the Select Tab submenu's
  existence and behavior is retargeted from the Window menu to the View menu; the
  `cmd-<n>` shortcut behavior itself is unchanged.

## Impact

- `crates/dtrpg-ui/src/ui/app/mod.rs`: `build_menus` — relocate the Select Tab submenu
  block from the Window `Menu` to the View `Menu`.
- No changes to `crates/dtrpg-ui/src/ui/views/root_view.rs` (`tab_target_at`), the
  `SelectTab0`..`SelectTab9` action definitions, or any keybinding — only the menu bar
  layout changes.
- No i18n key changes: `menu.window_select_tab_title` and `menu.window_select_tab_empty`
  keep their existing keys/text (renaming them is out of scope for this menu-placement
  change; a future change can revisit the `window_` prefix if desired).
- Note: `openspec/changes/view-menu-selection-state` is an implemented-but-unarchived
  change that documents the View menu's existing Presentation/Sort/Find-in-Library
  behavior as a new `view-menu` capability. That capability does not yet exist under
  `openspec/specs/`. This change's `view-menu` delta spec is additive against that
  not-yet-archived baseline and should be reconciled when `view-menu-selection-state` is
  archived.
