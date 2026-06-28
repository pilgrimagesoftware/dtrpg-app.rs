## MODIFIED Requirements

### Requirement: Rust search and filter controls MUST be disclosable
The Rust desktop app MUST provide a low-profile disclosable search/filter area with a functional text search input, view mode, grouping, and sort controls, plus a collapsed summary of active browsing state. The search input SHALL be an editable text field that filters the catalog on every keystroke. A clear button SHALL appear when the input is non-empty and SHALL reset the search and the input field.

#### Scenario: Toggling Rust filter disclosure
- **WHEN** the user expands or collapses the search/filter area
- **THEN** the Rust app preserves active search, filter, view mode, grouping, and sort state

#### Scenario: Typing in the search field filters the catalog
- **WHEN** the user types text into the search input
- **THEN** the catalog immediately narrows to items whose title or publisher matches the typed text

#### Scenario: Clearing the search field
- **WHEN** the user activates the clear button (✕) while a search query is active
- **THEN** the search input is emptied and the catalog returns to the unfiltered result set

#### Scenario: Empty search shows placeholder
- **WHEN** the search input is empty
- **THEN** the field displays its placeholder text ("Search…") and no clear button is shown
