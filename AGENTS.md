# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code), Codex (openai.com/codex/), GitHub Copilot (copilot.github.com) when working with code in this repository.

## About This Project

This is the repository for the DriveThruRPG frontend application implemented in Rust.

## Frameworks

The UI is implemented using the `gpui` framework from the [Zed](https://zed.dev) project, along with the `gpui-component` crate from [Longbridge](https://github.com/longbridge/gpui-component).

A short term goal is to move from `gpui` to `gpui-ce` (https://github.com/gpui-ce/gpui-ce).

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

## gpui-component Usage

Always prefer components from the `gpui-component` crate over rolling custom implementations:

- **Tabular data**: Use `DataTable` (virtualized, delegate-based, handles header/row alignment) or `Table`/`TableHeader`/`TableRow`/`TableHead`/`TableCell` (simple, stateless). Never build custom flex-row column layouts for table-like views — they cannot reliably align headers with rows when virtualization is involved.
- Check the `gpui-component` crate for any other UI primitive before implementing from scratch.

## Credential Storage

All DriveThruRPG account credentials (API key, access token, refresh token) MUST be stored
exclusively in the platform-native secure store via `dtrpg_ui::credentials::KeyringCredentialStore`.

- Service namespace: `com.pilgrimagesoftware.dtrpg` (constant: `credentials::keys::SERVICE`)
- Account keys: `api-key`, `access-token`, `refresh-token` (constants in `credentials::keys`)
- Never write credentials to config files, environment variables, or unencrypted databases.
- On Linux, the app requires `libsecret` (GNOME Keyring or KWallet). Add `libsecret-1-dev`
  to the Linux CI build environment.
- The SDK (`dtrpg-sdk`) does NOT persist credentials — that is the app's responsibility.
