## ADDED Requirements

### Requirement: Toolbar layout switcher uses TabBar (segmented)
The toolbar layout switcher SHALL use `gpui_component::tab::TabBar` in `segmented()` mode to render the List / Thumbs / Grid control, with active-tab tracking driven by `selected_index` rather than per-segment manual color logic.

#### Scenario: Active segment matches current presentation
- **WHEN** the catalog is in List presentation
- **THEN** the TabBar renders index 0 (List) as the active/selected segment

#### Scenario: Segment click changes presentation
- **WHEN** the user clicks the "Grid" segment (index 2)
- **THEN** `controller.set_presentation(CatalogPresentation::Grid, cx)` is called

#### Scenario: Tooltip on each segment
- **WHEN** the user hovers over a layout segment
- **THEN** a tooltip is shown ("List view", "Thumbnail view", or "Grid view" respectively)

### Requirement: Settings tab strip uses TabBar (pill)
The settings panel tab strip SHALL use `gpui_component::tab::TabBar` in `pill()` mode to render the Account / Storage / File Openers tabs, replacing the hand-crafted `render_tab_strip` function.

#### Scenario: Active tab highlighted
- **WHEN** the Account settings tab is active
- **THEN** the TabBar renders the Account pill as selected

#### Scenario: Tab click changes active section
- **WHEN** the user clicks "Storage"
- **THEN** `settings.set_tab(SettingsTab::Storage, cx)` is called and the Storage section content is displayed
