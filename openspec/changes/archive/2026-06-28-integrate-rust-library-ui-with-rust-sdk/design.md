## Context

Rust frontend baseline behavior is defined and initially stubbed. Integration phase replaces stubs with Rust SDK-backed communication while preserving shared request/recovery behavior from app-level specs.

## Goals / Non-Goals

**Goals:**
- Integrate Rust SDK-backed adapters for library workflows.
- Preserve shared backend request/recovery behavior.
- Keep Rust UI and adapter boundaries explicit and testable.

**Non-Goals:**
- Redefine shared behavior in Rust child spec.
- Redefine Rust SDK contracts.

## Decisions

Use a Rust-specific adapter capability for integration details.
Rationale: SDK wiring and error mapping details are Rust-specific implementation concerns.

Replace adapters, not shared behavior contracts.
Rationale: preserves cross-frontend product consistency.
