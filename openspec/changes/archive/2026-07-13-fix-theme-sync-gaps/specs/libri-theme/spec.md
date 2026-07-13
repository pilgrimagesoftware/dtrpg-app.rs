## ADDED Requirements

### Requirement: Theme changes MUST propagate to all gpui-component-backed widgets
Switching the active theme SHALL update colors on every `gpui-component` widget the app uses that reads from `gpui_component::Theme` rather than this app's own `ColorTokens` — including default-styled buttons, inputs, popovers/dropdown menus, tooltips, scrollbars, and the sidebar — not only the catalog `DataTable`/`Table`.

#### Scenario: Sidebar reflects the active theme
- **WHEN** the user switches from one theme to another
- **THEN** the sidebar's background, borders, and active-item highlight update to match the newly active theme

#### Scenario: Default-styled buttons reflect the active theme
- **WHEN** the user switches from one theme to another
- **THEN** buttons without a custom color variant (e.g. ghost/ordinary buttons throughout Settings and the toolbar) update to match the newly active theme

#### Scenario: Inputs, popovers, and scrollbars reflect the active theme
- **WHEN** the user switches from one theme to another
- **THEN** text input fields, popover/dropdown menu backgrounds, and scrollbar colors update to match the newly active theme

### Requirement: Settings warning labels MUST use theme tokens, not hardcoded colors
Settings-page warning labels (e.g. missing-download-folder, missing-file-opener-app warnings) SHALL use the active theme's `warning_text`/`warning_bg` tokens rather than a hardcoded color literal.

#### Scenario: Warning label color follows the active theme
- **WHEN** a settings-page warning label is shown and the user switches themes
- **THEN** the warning label's text color updates to match the newly active theme's warning color, the same as every other themed element on that page
