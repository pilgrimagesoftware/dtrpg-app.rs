## ADDED Requirements

### Requirement: Clicking the avatar while authenticated shows a logout popover
When the user is authenticated, clicking the avatar button SHALL display a small popover menu containing a single "Log Out" action item.

#### Scenario: Popover appears on click
- **WHEN** the user is authenticated and clicks the avatar button
- **THEN** a popover menu appears anchored below-left of the avatar button with a "Log Out" item visible

#### Scenario: User selects Log Out
- **WHEN** the user clicks the "Log Out" item in the avatar popover
- **THEN** the auth state transitions to `LoggedOut`, the avatar reverts to the generic person icon, and the popover closes

#### Scenario: Popover closes on outside click
- **WHEN** the avatar popover is open and the user clicks anywhere outside it
- **THEN** the popover closes without changing auth state

### Requirement: Clicking the avatar while unauthenticated performs no action
When the user is not authenticated, clicking the avatar button SHALL NOT open a popover or trigger any action. The button SHALL still be visually present but inert.

#### Scenario: Click while logged out
- **WHEN** the user is not authenticated and clicks the avatar button
- **THEN** no popover appears and no state changes
