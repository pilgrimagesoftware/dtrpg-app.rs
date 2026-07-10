# alert-history-copy-to-clipboard Specification

## Purpose
Lets a user copy an alert history entry's error message to the system clipboard, so it can be
pasted into a bug report, support request, or search.

## Requirements
### Requirement: Alert history entries can be copied to the clipboard
Each entry row in the alert history panel SHALL show a button, next to the entry's full
error message text, that copies that message to the system clipboard.

#### Scenario: Copying an error message
- **WHEN** the user clicks the copy button on an alert history entry row
- **THEN** the entry's full error message text is written to the system clipboard

#### Scenario: Message text is fully visible, not truncated
- **WHEN** an alert history entry row is rendered
- **THEN** the entry's error message is shown in full, word-wrapped within the panel rather
  than truncated with an ellipsis, so what is displayed matches what the copy button copies

#### Scenario: Copy button is always visible
- **WHEN** an alert history entry row is rendered, regardless of hover state
- **THEN** its copy button is visible alongside the message text

