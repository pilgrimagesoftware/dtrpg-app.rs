# Spec Delta

## Purpose

Defines which changes in the catalog's content views are animated, how long they last, and what must never animate, so the views stop flickering and jumping without costing frame time.

## ADDED Requirements

### Requirement: A change to the visible items fades the content area
When the set or order of visible items changes because of a filter, sort, search, grouping, or catalog data change, the system SHALL fade the content area from partly transparent to opaque over no more than 150 ms instead of replacing it in one frame.

#### Scenario: Filter changes the visible items
- **WHEN** the user checks a facet and the visible items change
- **THEN** the content area fades in to the new items within 150 ms and no row shows a half-updated state

#### Scenario: A change that leaves the items as they were
- **WHEN** the library changes but the visible items and their order are identical
- **THEN** no transition starts

### Requirement: Switching presentation fades the new presentation in
When the user switches between List, Thumbs, and Grid, or turns grouping on or off, the system SHALL fade the new presentation in over no more than 150 ms.

#### Scenario: List to Grid
- **WHEN** the user switches from List to Grid
- **THEN** the Grid fades in and the List is not visible during the transition

### Requirement: A cover fades in when it arrives
When a cover image becomes available for an item that is on screen and was showing its placeholder, the system SHALL fade the image in over the placeholder over no more than 200 ms. A cover that was already available when the item was drawn SHALL be shown at once.

#### Scenario: Cover arrives while on screen
- **WHEN** a cover download finishes for an item on screen
- **THEN** the image fades in over its placeholder

#### Scenario: Scrolling back to a cached cover
- **WHEN** an item with a cached cover scrolls into view
- **THEN** the cover is shown at once with no fade

### Requirement: Content fades in after a loading or empty state
When the content replaces the loading skeleton or the empty or no-match state, the content SHALL fade in over no more than 150 ms.

#### Scenario: Skeleton to content
- **WHEN** the first page of the catalog arrives while the skeleton is shown
- **THEN** the content fades in without a one-frame cut

### Requirement: Scroll position survives a change that keeps the items
A change to the catalog that keeps the same items in the same order SHALL NOT move the scroll position or restart the content transition.

#### Scenario: Thumbnail progress during a load
- **WHEN** covers keep arriving and no item is added, removed, or reordered
- **THEN** the scroll position does not change

### Requirement: Nothing animates per frame without a running transition
Transitions SHALL animate only values already computed for the layout, SHALL run only while a transition is in progress, and SHALL NOT add computation to a frame in which no transition runs. Scrolling, hover, selection, and progress updates SHALL NOT start a transition.

#### Scenario: Idle window
- **WHEN** no transition is in progress
- **THEN** no animation frames are requested

#### Scenario: Scrolling
- **WHEN** the user scrolls a list or grid
- **THEN** no transition starts for rows entering or leaving the viewport
