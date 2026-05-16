## Why

The Rust desktop app already contains GPUI library controls and SDK-backed library loading, but it needs a child OpenSpec change that maps the new shared main-window layout contract to Rust-specific implementation work. This keeps the Rust app aligned with the Swift frontend while allowing the existing GPUI architecture to carry the details.

## What Changes

- Define GPUI-specific main-window layout behavior for DriveThruRPG library browsing.
- Map the shared disclosable search/filter area to the existing Rust library controller and GPUI view modules.
- Define Rust account-menu state and actions for identity, token set/reset, and settings navigation.
- Define Rust browsing state for list/tree and grid presentations using one filtered/sorted result set.
- Define low-profile background sync status and asynchronous thumbnail behavior for the Rust app.

## Capabilities

### New Capabilities

- `rust-main-window-library-layout`: Defines GPUI-specific main-window layout, browsing state, account menu, and sync-status behavior for the Rust desktop app.

## Impact

- `dtrpg-app/rust`: Adds Rust implementation planning for the shared main-window library layout.
- Depends on `dtrpg-app/openspec/changes/define-shared-main-window-library-layout`.
- Can reuse existing Rust library controller, search, sort, grouping, and SDK adapter work where it satisfies the shared layout contract.
