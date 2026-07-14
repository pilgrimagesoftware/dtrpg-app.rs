# catalog-loading-state Specification

## Purpose
TBD - created by archiving change adopt-gpui-component-primitives. Update Purpose after archive.

## Requirements

### Requirement: Catalog shows a spinner while loading

The catalog content area SHALL display a centred `Spinner` component when the initial catalog fetch is in progress and no items have yet been delivered to the controller. Once the first page of items arrives the spinner is replaced by the catalog.

#### Scenario: Empty catalog during initial load shows spinner
- **WHEN** `LibraryController` is loading and the local catalog is empty
- **THEN** the catalog content area shows a centred `Spinner` and no table, thumb, or grid rows

#### Scenario: Spinner is hidden once items are available
- **WHEN** at least one catalog item has been delivered
- **THEN** the spinner is not shown, regardless of whether loading is still in progress
