# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code), Codex (openai.com/codex/), GitHub Copilot (copilot.github.com) when working with code in this repository.

## About This Project

This is the repository for the DriveThruRPG frontend application implemented in Rust.

## Frameworks

The UI is implemented using the `gpui` framework from the [Zed](https://zed.dev) project, along with the `gpui-component` crate from [Longbridge](https://github.com/longbridge/gpui-component).

## Architecture and Code Organization

Use a modular architecture. Do not place substantial UI, state, and data logic in a single file.

- `src/main.rs` should stay minimal and only bootstrap application startup.
- `src/ui/` owns GPUI view composition and rendering concerns.
- `src/ui/<feature>/state.rs` owns feature interaction/state controller logic.
- `src/ui/<feature>/data.rs` owns sorting/filtering/grouping helpers and presentation transforms.
- `src/view_models/` owns domain-facing view models that mediate service data into UI state.

When adding features, prefer new focused modules over expanding existing files into monoliths.

## UI Policy

- Preserve the separation between UI rendering, interaction state, and service adapters.
- Keep code structured so stubs can be replaced by SDK adapters without rewriting views.
