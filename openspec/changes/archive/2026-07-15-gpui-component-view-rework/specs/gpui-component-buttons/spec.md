## ADDED Requirements

### Requirement: Toolbar group toggle uses Button component
The toolbar group toggle SHALL use `gpui_component::button::Button` to render the "Group" control, with visual active state driven by the component's built-in selected/toggle styling rather than manual color logic.

#### Scenario: Grouped is inactive
- **WHEN** the catalog is in ungrouped mode (`grouped == false`)
- **THEN** the Group button renders in its default (unselected) state with no accent fill

#### Scenario: Grouped is active
- **WHEN** the catalog is in grouped mode (`grouped == true`)
- **THEN** the Group button renders in its selected/active state with accent-soft background and accent text

#### Scenario: Group toggle click
- **WHEN** the user clicks the Group button
- **THEN** `controller.set_grouped(!grouped, cx)` is called

### Requirement: Toolbar settings gear uses Button component
The settings gear button SHALL use `gpui_component::button::Button` (ghost variant) instead of a hand-crafted `div()` with manual border and cursor styling.

#### Scenario: Settings button renders
- **WHEN** the toolbar is rendered
- **THEN** a ghost-style button with the settings icon is present with an accessible tooltip "Settings"

#### Scenario: Settings button click
- **WHEN** the user clicks the settings button
- **THEN** `settings.toggle(cx)` is called

### Requirement: Detail panel action buttons use Button component
The Read, Download/Downloaded, and Show in Finder action buttons in the detail panel SHALL use `gpui_component::button::Button` with appropriate variants (primary for Read, outline for Download and Reveal).

#### Scenario: Read button
- **WHEN** an item is selected in the detail panel
- **THEN** a primary-variant Button labeled "Read" is displayed

#### Scenario: Download button (not downloaded)
- **WHEN** the selected item has `status == ItemStatus::Cloud`
- **THEN** an outline-variant Button labeled "Download" is displayed and triggers `controller.toggle_download(id, cx)` on click

#### Scenario: Downloaded button
- **WHEN** the selected item has `status == ItemStatus::Downloaded`
- **THEN** an outline-variant Button labeled "Downloaded" is displayed (disabled or de-emphasized to indicate already-downloaded state)

#### Scenario: Show in Finder button
- **WHEN** the selected item has `status == ItemStatus::Downloaded`
- **THEN** an outline-variant Button with the platform reveal label is displayed and triggers `reveal_in_file_manager` on click

### Requirement: Settings account logout button uses Button component
The logout button in the account settings section SHALL use `gpui_component::button::Button` (danger variant) instead of a hand-crafted `div()`.

#### Scenario: Logout button renders
- **WHEN** the user is authenticated and the Account settings tab is active
- **THEN** a danger-variant Button labeled "Log Out" is displayed

#### Scenario: Logout button click
- **WHEN** the user clicks Log Out
- **THEN** `settings.request_logout(cx)` is called
