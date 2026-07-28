## REMOVED Requirements

### Requirement: Window menu contains a Select Tab submenu
**Reason**: The Select Tab submenu is a view-navigation control, not a window-management
action, and has been moved to the View menu (see the `view-menu` and
`catalog-tab-cmd-number-shortcuts` capabilities).
**Migration**: No user-facing migration needed — the submenu and its ten items,
including their `cmd-0`..`cmd-9` keybindings, are unchanged; only their menu location
moves from Window to View.
