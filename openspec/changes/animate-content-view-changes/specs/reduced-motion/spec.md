# Spec Delta

## Purpose

Makes the app follow the operating system's reduce-motion accessibility setting so users who turn it on see no animated transitions.

## ADDED Requirements

### Requirement: The app follows the OS reduce-motion setting
The system SHALL read the operating system's reduce-motion setting at startup and whenever it changes, and SHALL apply it to every animated transition in the app without a restart.

#### Scenario: Setting on at startup
- **WHEN** the app starts with reduce motion on
- **THEN** no transition animates

#### Scenario: Setting turned on while running
- **WHEN** the user turns reduce motion on while the app is open
- **THEN** transitions that start after that moment show their end state at once

### Requirement: With reduce motion on, a transition shows its end state
When reduce motion is on, a transition SHALL render its end state immediately and SHALL NOT schedule animation frames.

#### Scenario: Filter change with reduce motion on
- **WHEN** the visible items change with reduce motion on
- **THEN** the new content appears at once and no animation frame is requested

### Requirement: Platforms without the setting animate normally
On a platform where the setting cannot be read, the system SHALL treat reduce motion as off.

#### Scenario: Setting unavailable
- **WHEN** the app runs where the setting is not readable
- **THEN** transitions animate as designed
