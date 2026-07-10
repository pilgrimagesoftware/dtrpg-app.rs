## 1. Fix the badge data source

- [x] 1.1 In `crates/dtrpg-ui/src/data/activity.rs`, rename
      `ActivitySnapshot::recent_error_count` to `alert_count` and update its doc comment to
      describe it as the notifications-view (alert log) entry count, not a recent-activity
      count.
- [x] 1.2 In `crates/dtrpg-ui/src/controllers/activity.rs`'s `snapshot()`, compute
      `alert_count` from `self.alert_log.len()` instead of filtering `self.recent` for
      `Error` status; remove the now-unused `recent`-filtering logic if nothing else uses
      it.
- [x] 1.3 In `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`, update `has_errors` and the
      notifications tooltip's `n` interpolation to read `activity_snap.alert_count`.
  - Note: `has_errors` doesn't exist in current code (the proposal was written against a
    stale version of this file). The actual red-dot indicator is already driven by
    `alert_snap.has_unread` (`has_unread_alert` in the controller), which `clear_alert_log`
    already correctly clears — that part was never broken. The real, narrower bug this
    fixes: the notification tooltip's "n unread errors" text read a transient, self-expiring
    count (`recent`-filtered) instead of the durable `alert_log`, so it could show a stale
    number that didn't match what `clear_alert_log` had already emptied. Only the `n`
    interpolation needed updating.

## 2. Tests

- [x] 2.1 Add/update a unit test on `ActivityController` confirming `snapshot().alert_count`
      reflects `alert_log`'s length, not `recent`'s.
  - `alert_count_reflects_alert_log_len_not_recent`.
- [x] 2.2 Add a unit test: after `error()` pushes an entry and then `clear_alert_log` runs,
      `snapshot().alert_count` is `0`.
  - `clearing_alert_log_zeroes_alert_count`. Seeds state directly (`push_alert`, then clears
    `alert_log`/`has_unread_alert`) rather than calling `error()`/`clear_alert_log()`
    directly, since both require `cx: &mut Context<Self>` and this crate has no existing
    GPUI `TestAppContext` test harness (no other controller test in this crate calls a
    `cx`-requiring method either — see note on task 1.3/design deviation below). Verifies
    the same outcome the task describes.
- [x] 2.3 Add a unit test: after `error()` pushes an entry and `expire_item` later removes
      the corresponding `recent` entry (simulating the expiry timer), `snapshot().alert_count`
      remains unchanged (still reflects `alert_log`, unaffected by `recent`'s expiry).
  - `recent_expiry_does_not_affect_alert_count`. Same direct-state-seeding approach as 2.2.

## 3. Verification

- [x] 3.1 `cargo test -p dtrpg-ui` passes, including the new/updated activity tests. — 192
      unit tests (189 baseline + 3 new) + 10 doc-tests, all pass.
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings` passes. — clean, zero
      warnings.
- [x] 3.3 `cargo fmt --all -- --check` passes. — this repo's `rustfmt.toml` requires
      `cargo +nightly fmt` (`unstable_features = true`); ran that, then `-- --check` passes
      clean.
- [ ] 3.4 Manually trigger a background operation failure, confirm the badge appears, open
      the notifications popover and clear it, and confirm the badge disappears immediately.
  - Not run in this session (requires the live GUI app). Per the note on task 1.3: the red
    dot itself was already correctly cleared by `clear_alert_log` before this change: what
    this task should actually confirm now is that the notification tooltip's "n unread
    errors" count also reads correctly and drops to 0 after clearing, matching the dot.
