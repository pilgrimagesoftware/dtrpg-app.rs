# gpui-component-alert Specification

## Purpose
TBD - created by archiving change gpui-component-view-rework. Update Purpose after archive.
## Requirements
### Requirement: Notification banner uses Alert component
Each notice in the notification banner SHALL be rendered using `gpui_component::Alert` in `Warning` variant and `banner(true)` mode, replacing the hand-crafted `div()` banner rows.

#### Scenario: Warning notice renders as Alert banner
- **WHEN** one or more notices are present (e.g., `NoticeKind::SessionExpired`)
- **THEN** each notice renders as a full-width `Alert` banner with warning styling and the appropriate message text

#### Scenario: Dismiss removes the notice
- **WHEN** the user activates the Alert's close/dismiss action
- **THEN** `auth_entity.dismiss_notice(kind, cx)` is called and the banner for that notice disappears

#### Scenario: Action button opens settings
- **WHEN** the user clicks the action button within the notice (e.g., "Set Up Account" or "Sign In Again")
- **THEN** `settings.set_tab(tab, cx)` and `settings.open(cx)` are called and the notice is dismissed

#### Scenario: Empty when no notices
- **WHEN** the notices list is empty
- **THEN** no Alert elements are rendered (the banner area is invisible)

