## ADDED Requirements

### Requirement: App name renders exactly once, in the title bar

The system SHALL render the app name ("Libri") exactly once, in the title bar positioned
below the window's traffic-light controls. The sidebar SHALL NOT render a duplicate app
name header.

#### Scenario: Title bar shows the app name

- **WHEN** the main window renders
- **THEN** the title bar shows "Libri" below the traffic-light controls

#### Scenario: Sidebar does not duplicate the app name

- **WHEN** the sidebar renders
- **THEN** it does not show an app name wordmark; its top section begins with the
  smart-filter navigation menu
