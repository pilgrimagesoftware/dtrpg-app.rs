## 1. Aggregate Progress Source

- [x] 1.1 Add or confirm an `ActivityController` accessor returning aggregate
      (completed, total) across all currently active loaders
- [x] 1.2 Determine indeterminate vs. determinate state: loaders without a known total
      (e.g. thumbnail queue) contribute to an indeterminate spin instead of a fixed value

## 2. Status Bar Indicator

- [x] 2.1 Replace the glyph `Button` label in `render_status_bar` with
      `ProgressCircle::new(...)` sized for the status bar row
- [x] 2.2 Wire `.value()` from the aggregate accessor; use `.loading(true)` for the
      indeterminate case
- [x] 2.3 Preserve `.on_click()` → `toggle_panel` and the existing tooltip text
- [x] 2.4 Idle state (no in-progress or recent items) renders an empty/inactive circle

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Trigger a catalog load and confirm the circle fills proportionally to progress
- [ ] 4.2 Trigger thumbnail loading (no known total) and confirm an indeterminate spin
- [ ] 4.3 Confirm clicking the indicator still opens the activity panel
- [ ] 4.4 Confirm the idle state (no activity) looks visually inactive
