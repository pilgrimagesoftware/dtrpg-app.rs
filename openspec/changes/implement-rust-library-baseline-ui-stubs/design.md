## Context

Shared app-level specs define baseline desktop behavior across frontends. Rust implementation must realize that behavior with Rust-native architecture while staying backend-stubbed in this phase.

## Goals / Non-Goals

**Goals:**
- Implement shared baseline layout/state behavior in Rust frontend.
- Define Rust-specific view composition and state transitions.
- Use stub adapters for library data operations.

**Non-Goals:**
- Integrate real Rust SDK backend communication.
- Redefine shared UX behavior owned by app meta specs.

## Decisions

Own Rust-specific implementation details in a child capability.
Rationale: shared behavior and Rust implementation concerns should stay separate.

Use stub adapters behind service boundaries.
Rationale: keeps a clear seam for follow-up Rust SDK integration.
