## 1. Data model

- [x] 1.1 Add `AlertEntry { id: u64, label: Arc<str>, message: String, occurred_at: SystemTime }` to `data/activity.rs`
- [x] 1.2 Add `AlertHistorySnapshot { open: bool, entries: Vec<AlertEntry> }` to `data/activity.rs`
- [x] 1.3 Add `ALERT_LOG_CAP: usize` to `data/constants.rs`

## 2. Controller

- [x] 2.1 Add `alert_log: VecDeque<AlertEntry>` and `alert_panel_open: bool` fields to `ActivityController`
- [x] 2.2 In `error(...)`, push a new `AlertEntry` to `alert_log`, evicting the oldest entry once `ALERT_LOG_CAP` is reached
- [x] 2.3 Add `toggle_alert_panel(&mut self, cx)` following the `toggle_panel` pattern
- [x] 2.4 Add `clear_alert_log(&mut self, cx)` that empties `alert_log` and emits `ActivityChanged`
- [x] 2.5 Add `alert_snapshot(&self) -> AlertHistorySnapshot`
- [x] 2.6 `clear(&mut self, cx)` (used on service replacement / sign-out) leaves `alert_log` intact — alert history is a durable session log, not tied to the active service

## 3. Panel view

- [x] 3.1 Create `ui/views/alert_history_view.rs` with `render_alert_history_panel(snap: &AlertHistorySnapshot, entity: Entity<ActivityController>, colors: &ColorTokens) -> AnyElement`
- [x] 3.2 Header: title, "Clear" action (calls `clear_alert_log`), close button (calls `toggle_alert_panel`)
- [x] 3.3 Row: label, error message, relative timestamp via `util::datetime::format_relative`, tooltip with absolute timestamp via `format_absolute`
- [x] 3.4 Empty state matching the visual pattern of `activity_panel_view::render_empty`
- [x] 3.5 Register the new module in `ui/views/mod.rs`

## 4. Wiring

- [x] 4.1 In `root_view.rs`, replace the `ShowAlertHistory` stub handler with `activity.update(cx, |a, cx| a.toggle_alert_panel(cx))`
- [x] 4.2 Read `alert_snap = self.activity.read(cx).alert_snapshot()` in `render`
- [x] 4.3 Compose the alert history overlay as a root-level sibling (same pattern as `activity_overlay`), positioned so it does not fully overlap the activity panel when both are open — offset `left(360.0)` places it beside rather than under the activity panel (`w(340.0)`)

## 5. Localization

- [x] 5.1 Add `alert_history.title`, `alert_history.empty`, `alert_history.empty_hint`, `alert_history.clear` keys to `en.yaml`, `de.yaml`, `fr.yaml`

## 6. Verify

- [x] 6.1 Run `cargo check --workspace --all-targets` and confirm no compile errors or warnings
- [x] 6.2 Run `cargo test --workspace` and confirm all tests and doctests pass (88 unit tests + 4 doctests, 0 failures) — `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo fmt --all -- --check` also pass
- [ ] 6.3 Trigger an error activity (e.g. a failed collection delete) and confirm it appears in both the activity panel (until it expires) and the alert history panel (persists) — requires a manual run against a live or stubbed error path; not exercised by an automated test in this codebase (no existing precedent for GPUI `Context`-based controller tests here — see `ActivityController`, `LibraryController`)
