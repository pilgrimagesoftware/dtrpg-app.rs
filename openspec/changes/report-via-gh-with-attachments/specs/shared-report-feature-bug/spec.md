## REMOVED Requirements

### Requirement: Report submission via browser handoff
**Reason**: Browser handoff is now the fallback, not the only path; "requires no local CLI tool" no longer holds.
**Migration**: Replaced by "Report submission via gh with browser fallback".

## ADDED Requirements

### Requirement: Report submission via gh with browser fallback
On submit, the app SHALL file the report through `gh issue create` against
`pilgrimagesoftware/dtrpg-app.rs` when `gh` is installed and signed in. When it
is not, the app SHALL compose a pre-filled "new issue" URL embedding the kind,
subject, description and diagnostic context, and open it in the user's default
browser. The app SHALL NOT store credentials of its own.

#### Scenario: gh is ready
- **WHEN** the user submits a valid report and `gh` is installed and signed in
- **THEN** the issue is filed through `gh` and the window closes

#### Scenario: gh fails
- **WHEN** `gh` is ready but filing fails
- **THEN** the window stays open and shows the error

#### Scenario: gh is not usable
- **WHEN** the user submits a valid report and `gh` is missing or signed out
- **THEN** the app opens a pre-filled "new issue" page in the browser

### Requirement: Optional application log attachment
The dialog SHALL offer an option to attach the application log. With `gh`, the
tail of the newest log SHALL be included in the issue body; in the browser
fallback the body SHALL name the log file instead, since a URL cannot carry it.

#### Scenario: Log attached through gh
- **WHEN** the option is on and the report is filed through `gh`
- **THEN** the issue body contains the log tail in a collapsed block

### Requirement: Optional screenshots
The dialog SHALL let the user choose image files to accompany the report. The
issue body SHALL name them by file name only, and the app SHALL reveal each file
in the file manager so the user can drag it onto the issue.

#### Scenario: Screenshot chosen
- **WHEN** the user submits a report with a screenshot
- **THEN** the body lists its file name, the issue page opens, and the file is revealed
