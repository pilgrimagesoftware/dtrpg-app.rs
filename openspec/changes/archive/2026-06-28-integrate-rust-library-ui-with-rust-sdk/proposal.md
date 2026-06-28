## Why

After baseline Rust UI behavior is implemented with stubs, the Rust frontend needs a dedicated child change to replace stubs with real Rust SDK communication while preserving shared integration behavior.

## What Changes

- Replace Rust baseline stubs with Rust SDK-backed adapters.
- Preserve shared desktop backend integration behavior and Rust app boundaries.
- Define Rust-specific adapter and error/session mapping implementation concerns.

## Capabilities

### New Capabilities
- `rust-library-sdk-adapter`: Defines Rust frontend adapter behavior for integrating Rust SDK into library workflows.

## Impact

- `dtrpg-app/rust`: Rust-specific backend integration implementation.
- Depends on `dtrpg-app/openspec/changes/define-shared-desktop-library-sdk-integration`.
