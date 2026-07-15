## ADDED Requirements

### Requirement: Detail panel Read button MUST apply the download guard
The Rust main-window detail panel SHALL apply the `detail-read-button-download-guard` behavior: the Read button is disabled with a tooltip when the selected item is not downloaded.

#### Scenario: Detail panel reflects download guard in all item states
- **WHEN** the detail panel renders for any selected item
- **THEN** the Read button is enabled only when `item.status == Downloaded`, and shows the download-prerequisite tooltip otherwise
