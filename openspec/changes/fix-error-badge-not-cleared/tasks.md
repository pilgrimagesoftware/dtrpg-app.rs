## 1. Fix the badge data source

- [ ] 1.1 In `crates/dtrpg-ui/src/data/activity.rs`, rename
      `ActivitySnapshot::recent_error_count` to `alert_count` and update its doc comment to
      describe it as the notifications-view (alert log) entry count, not a recent-activity
      count.
- [ ] 1.2 In `crates/dtrpg-ui/src/controllers/activity.rs`'s `snapshot()`, compute
      `alert_count` from `self.alert_log.len()` instead of filtering `self.recent` for
      `Error` status; remove the now-unused `recent`-filtering logic if nothing else uses
      it.
- [ ] 1.3 In `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`, update `has_errors` and the
      notifications tooltip's `n` interpolation to read `activity_snap.alert_count`.

## 2. Tests

- [ ] 2.1 Add/update a unit test on `ActivityController` confirming `snapshot().alert_count`
      reflects `alert_log`'s length, not `recent`'s.
- [ ] 2.2 Add a unit test: after `error()` pushes an entry and then `clear_alert_log` runs,
      `snapshot().alert_count` is `0`.
- [ ] 2.3 Add a unit test: after `error()` pushes an entry and `expire_item` later removes
      the corresponding `recent` entry (simulating the expiry timer), `snapshot().alert_count`
      remains unchanged (still reflects `alert_log`, unaffected by `recent`'s expiry).

## 3. Verification

- [ ] 3.1 `cargo test -p dtrpg-ui` passes, including the new/updated activity tests.
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [ ] 3.3 `cargo fmt --all -- --check` passes.
- [ ] 3.4 Manually trigger a background operation failure, confirm the badge appears, open
      the notifications popover and clear it, and confirm the badge disappears immediately.
