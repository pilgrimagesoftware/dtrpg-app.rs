## ADDED Requirements

### Requirement: Title column is labeled "Title" (not "Title / Kind")
The first column in the catalog list DataTable SHALL have the header label "Title". The kind information is moved into the cell body as a badge rather than sharing the column header name.

#### Scenario: Column header label
- **WHEN** the catalog list view is displayed
- **THEN** the first column header shows "Title" only, with no "/Kind" suffix

### Requirement: Item kind is displayed as an abbreviated text badge
In the title cell of each list row, the item kind SHALL be displayed as a compact text badge (2–3 character abbreviation) adjacent to the title text. The badge SHALL be visually distinct from the title text (smaller, lower contrast, pill-shaped background).

The abbreviation mapping SHALL cover at minimum:

| Kind string contains | Badge |
|---------------------|-------|
| "Core"              | CR    |
| "Supplement"        | SUP   |
| "Adventure"         | ADV   |
| "Map"               | MAP   |
| "Token"             | TOK   |
| "Bundle" or "PDF"   | PDF   |
| (default)           | OTH   |

#### Scenario: Core Rulebook badge
- **WHEN** an item's kind contains "Core"
- **THEN** the title cell shows a "CR" badge adjacent to the title text

#### Scenario: Unknown kind falls back to OTH
- **WHEN** an item's kind does not match any known category
- **THEN** the title cell shows an "OTH" badge

#### Scenario: Badge does not displace title text
- **WHEN** the title is long and the column is narrow
- **THEN** the title truncates before the badge, and the badge remains visible and does not wrap
