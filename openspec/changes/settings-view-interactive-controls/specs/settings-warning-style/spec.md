## ADDED Requirements

### Requirement: Storage location warning text uses a warning color and symbol
The "Changing the storage location will not move existing downloaded files." line in the Storage settings section SHALL render in an amber warning color (`hsla(0.08, 0.9, 0.55, 1.0)`) and SHALL be prefixed with a `⚠` warning symbol, making it visually distinct from standard helper text.

#### Scenario: Warning text is visible in the Storage section
- **WHEN** the user opens the Storage settings section
- **THEN** the warning line reads "⚠ Changing the storage location will not move existing downloaded files." and is rendered in amber rather than the default tertiary text color
