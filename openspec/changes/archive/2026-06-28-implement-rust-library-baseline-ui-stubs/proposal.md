## Why

The app meta-repository now defines shared language-agnostic library baseline behavior. The Rust frontend needs its own child implementation change that applies those requirements using Rust-specific UI architecture and stubbed backend adapters.

## What Changes

- Implement the shared baseline library layout and states in the Rust frontend.
- Define Rust-specific UI composition, state wiring, and stub service adapter boundaries.
- Keep backend communication stubbed in this phase.

## Capabilities

### New Capabilities
- `rust-library-ui-implementation`: Defines Rust-specific implementation of shared desktop library baseline behavior.

## Impact

- `dtrpg-app/rust`: Baseline Rust UI implementation details.
- Depends on `dtrpg-app/openspec/changes/define-shared-desktop-library-baseline`.
