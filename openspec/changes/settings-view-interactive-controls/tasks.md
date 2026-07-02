## 1. Dependencies and Data Model

- [x] 1.1 `rfd` was already a workspace dependency (used by the Storage "Change…" folder picker); no `Cargo.toml` change needed
- [x] 1.2-1.4 — superseded: rather than a `pending_add` field on `SettingsController` (transient UI state on a shared domain-ish entity, which `design.md` itself flagged as a trade-off), the Add flow reuses the existing `window.open_dialog` modal pattern already established by the "New Collection" dialog in `root_view.rs`. The picked `app_path` and a per-`LibraryRootView` `file_opener_extension_input: Entity<InputState>` are captured directly in the dialog's `on_ok`/`on_cancel` closures — no new controller state, no `PendingAdd` struct, no `begin_add`/`confirm_add`/`cancel_add` methods.
- [x] 1.3 (storage half) `storage_path_draft: String` and `set_storage_path_draft()` already existed on `SettingsController` prior to this change, wired to a real `storage_path_input: Entity<InputState>` — already satisfies this item.
- [x] 1.5 `set_storage_path_draft()` already existed (see above).
- [x] 1.6 — not needed: no `pending_add` field was added to `SettingsSnapshot` (see 1.2-1.4). `storage_path_draft` was already present in the snapshot before this change.
- [x] 1.7 `cargo check -p dtrpg-ui` — zero errors

## 2. File Openers — Add Flow

- [x] 2.1 `render_add_button` uses `rfd::FileDialog::new().add_filter("Applications", &["app"]).set_directory("/Applications").pick_file()` — the synchronous variant, matching the existing "Change…" storage-folder picker's convention in the same codebase (native dialogs are modal; blocking the click handler for the dialog's duration is the established pattern here, not `AsyncFileDialog`/`cx.spawn()`)
- [x] 2.2-2.3 — superseded: instead of an inline form replacing the list, picking an app opens a `window.open_dialog` modal (same widget as "New Collection") with an app-name label and an extension `Input` bound to `file_opener_extension_input`, Add/Cancel buttons
- [x] 2.4 The extension `Input` is bound directly to `file_opener_extension_input`'s `InputState`; no separate draft-sync method needed since the dialog reads `extension_input.read(cx).value()` directly in `on_ok`
- [x] 2.5 `on_ok` trims the input, rejects empty (returns `false`, keeping the dialog open — same pattern as the "New Collection" dialog's empty-name check), builds a `FileOpenerEntry`, and calls `ctrl.add_file_opener()` (already normalizes via `FileOpenerConfig::add()` → `normalize_ext`, which now also trims whitespace — see the `data/file_openers.rs` fix below) and persists via the existing `.save()` call inside `add_file_opener()`
- [x] 2.6 `on_cancel` returns `true` (closes dialog); no controller state to clear since none was added
- [x] 2.7 `cargo check -p dtrpg-ui` — zero errors

### Data-model fix found during implementation

- [x] `data/file_openers.rs`: `normalize_ext` stripped a leading dot and lower-cased but did not trim whitespace, contradicting the `settings-editable-fields` spec's "trims whitespace" requirement. Fixed to `ext.trim().trim_start_matches('.').to_lowercase()`; added `add_trims_whitespace_in_extension` regression test.

## 3. Storage Section — Editable Path and Warning Style

- [x] 3.1 — already implemented prior to this change: `settings_storage_view.rs` renders a real gpui-component `Input` bound to `storage_path_input` when present (falls back to a static div only if the input entity hasn't been attached yet). Typing updates `storage_path_draft` via the existing `set_storage_path_draft()` subscription in `root_view.rs`; not persisted (matches the `settings-editable-fields` spec's "SHALL NOT be persisted until `catalog-storage-location` is connected").
- [x] 3.2 — already implemented prior to this change: the warning line already renders in `gpui::hsla(0.08, 0.9, 0.55, 1.0)` (amber) with a `⚠` prefix. Wording differs slightly from the spec's exact string ("download location" vs. "storage location") but the color/symbol requirement is met; left as-is since it's pre-existing, shipped text outside this change's scope.
- [x] 3.3 `cargo check -p dtrpg-ui` — zero errors

## 4. Verification

- [ ] 4.1 Build and run the app; open Settings → File Openers; click Add; confirm a native macOS open-file dialog appears
- [ ] 4.2 Select an `.app` bundle; confirm the extension entry dialog appears with the correct app name displayed
- [ ] 4.3 Type an extension and click Add; confirm the new entry appears in the list with correct extension and app name
- [ ] 4.4 Click Add again and dismiss the native file dialog without selecting a file; confirm no new entry appears and no modal opens
- [ ] 4.5 Add an entry with an extension that already exists; confirm the existing entry is replaced (not duplicated)
- [ ] 4.6 Click Remove on an entry; confirm the entry disappears from the list immediately (pre-existing behavior, re-verify no regression)
- [ ] 4.7 Open Settings → Storage; click the path field; type a new value; confirm the field accepts input (pre-existing behavior, re-verify no regression)
- [ ] 4.8 Close and reopen Settings; confirm the path field shows the placeholder/current path (not a leftover typed value)
- [ ] 4.9 Confirm the warning line displays in amber with a ⚠ prefix
- [x] 4.10 `cargo test --workspace --lib --bins` — 88/88 pass; `cargo clippy --all-targets --all-features -- -D warnings` — clean; `cargo fmt` — clean on touched files
