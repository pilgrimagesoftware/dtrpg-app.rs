# rust-main-window-structure Specification

## Purpose
TBD - created by archiving change add-rust-main-window-structure. Update Purpose after archive.
## Requirements
### Requirement: Rust main window MUST provide a GPUI title bar
The Rust desktop app MUST implement a title bar view above the content area, separated by a
horizontal rule, showing the window title and an account button that opens a menu with user info,
a settings action, and a sign-out action.

#### Scenario: Rendering the Rust title bar
- **WHEN** the Rust app displays the main window
- **THEN** it shows the title bar with the window title, a horizontal separator, and the account
  button

#### Scenario: Signing out from the Rust title bar
- **WHEN** the user selects sign-out from the title bar account menu
- **THEN** the Rust app ends the current session and returns to the signed-out state

### Requirement: Rust sidebar MUST show default section counts
The Rust desktop app sidebar MUST display a numeric item count on each default navigation section,
in addition to the existing Collections and Publishers section counts.

#### Scenario: Viewing default section counts in the Rust sidebar
- **WHEN** the sidebar is expanded
- **THEN** each default navigation section shows its current numeric item count alongside the
  existing Collections and Publishers counts

### Requirement: Rust main content area MUST use a GPUI tab strip
The Rust desktop app MUST present a `gpui-component` `TabBar`-based segmented tab strip with an
overflow "more" menu, a non-closable catalog tab first, and closable expanded detail tabs.

#### Scenario: Opening the Rust tab overflow menu
- **WHEN** more tabs are open than fit the tab strip width
- **THEN** the Rust app shows a "more" menu listing the remaining tabs, and selecting one activates
  it

#### Scenario: Catalog tab cannot be closed
- **WHEN** the user views the catalog tab
- **THEN** the Rust app does not render a close control for it

### Requirement: Rust catalog tab header MUST host search, sort, and view mode controls
The Rust desktop app catalog tab MUST relocate its existing search, sort, and view mode controls
into the tab's own header, replacing the disclosable filter strip.

#### Scenario: Filtering from the Rust catalog tab header
- **WHEN** the user changes search text, sort order, or view mode in the catalog tab header
- **THEN** the Rust app updates the catalog tab's content using the existing shared browsing state

### Requirement: Rust catalog items MUST distinguish popover and tab detail
The Rust desktop app MUST open a `gpui-component` `Popover` on single-click and a new closable
expanded detail tab on double-click, with the expanded tab showing a large thumbnail, item
attributes, and a file list for multi-item entries.

#### Scenario: Opening a Rust popover detail
- **WHEN** the user single-clicks a catalog item
- **THEN** the Rust app shows an anchored popover with item detail and does not modify the tab
  strip

#### Scenario: Opening a Rust expanded detail tab
- **WHEN** the user double-clicks a catalog item
- **THEN** the Rust app opens a new closable tab with a large thumbnail, item attributes, and a
  file list when the item bundles multiple files

### Requirement: Rust status bar MUST consolidate library, theme, activity, and notification indicators
The Rust desktop app MUST provide a status bar showing total library item count and size, the
active tab's summary, a theme picker, an activity indicator, and a notification indicator, reusing
existing theme, activity, and notification components.

#### Scenario: Viewing the Rust status bar
- **WHEN** the Rust app displays the main window
- **THEN** the status bar shows library totals, the active tab summary, and the theme, activity,
  and notification indicators

