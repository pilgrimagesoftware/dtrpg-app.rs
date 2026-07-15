## 1. Data model — library.rs

- [x] 1.1 Add `pub date_added: Option<i64>` field to `LibraryItem` (Unix seconds; `None` = unknown)
- [x] 1.2 Add `date_added: Option<i64>` parameter to `LibraryItem::new()`; set `self.date_added = date_added`

## 2. Datetime utility — util/datetime.rs

- [x] 2.1 Create `crates/dtrpg-ui/src/util/datetime.rs`; declare `pub mod datetime;` in `util/mod.rs`
- [x] 2.2 Implement `pub fn format_relative(ts: i64) -> String` using `std::time::SystemTime` and `UNIX_EPOCH`; cover all label buckets: "just now", "N minutes ago", "N hours ago", "yesterday", "N days ago", "N weeks ago", "Mon D", "Mon D, YYYY"
- [x] 2.3 Implement helper `fn epoch_to_ymd(ts: i64) -> (i32, u32, u32)` using Gregorian proleptic calendar arithmetic (no crate)
- [x] 2.4 Implement `pub fn format_absolute(ts: i64) -> String` returning "Month D, YYYY at H:MM AM/PM" (e.g., "January 5, 2024 at 3:42 PM")
- [x] 2.5 Add `#[cfg(test)] mod tests` in `datetime.rs` with unit tests for `format_relative` at each bucket boundary (59s, 60s, 59m, 1h, 23h, 24h, 47h, 48h, 6d, 7d, 29d, 30d) and at least two `format_absolute` cases

## 3. Stub data — util/stubs.rs

- [x] 3.1 Add a helper in `stubs.rs`: `fn stub_date_added(added_order: u32) -> Option<i64>` — computes `SystemTime::now()` as Unix seconds and subtracts `added_order as i64 * 43200` (12 hours per rank unit)
- [x] 3.2 Update all 46 `LibraryItem::new(...)` calls in `stubs.rs` to pass `stub_date_added(added_order)` as the new final argument

## 4. Detail panel — detail_panel_view.rs

- [x] 4.1 In `render_metadata_table`, import `Tooltip` from `gpui_component::tooltip` and `SharedString` from `gpui`
- [x] 4.2 Import `crate::util::datetime::{format_relative, format_absolute}`
- [x] 4.3 After the existing `rows` loop, conditionally append an "Added" child when `item.date_added.is_some()`: render a `div().id(SharedString::from(format!("detail-added-{}", item.id))).tooltip(...)` wrapping the relative label text, where the tooltip calls `Tooltip::new(absolute_text).build(window, cx)`
- [x] 4.4 Used closure-based `.tooltip()` pattern — no signature change needed; call site unchanged

## 5. Verification

- [x] 5.1 Run `cargo test --all-features -- datetime` — all datetime unit tests pass (13 unit + 2 doc)
- [x] 5.2 Run `cargo check --all-targets` — no compile errors
- [x] 5.3 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings (pre-existing expect lint at library.rs:102 is unrelated)
- [x] 5.4 Open the detail panel for an item; confirm the "Added" row shows a relative label
- [x] 5.5 Hover the "Added" value; confirm a tooltip appears with the full date and time
- [x] 5.6 Confirm items with `added_order` values in each time bucket show the correct label format

## 6. Fix: real SDK data leaked raw RFC 3339 timestamps into `desc`

The Rust SDK adapter (`services::sdk::map_order_product`) built `LibraryItem.desc` by
interpolating the API's raw `datePurchased`/`fileLastModified` RFC 3339 strings directly
(e.g. `"Purchased 2024-07-16T10:45:52-05:00"`), and always set `date_added: None` — so real
(non-stub) items never used the relative/tooltip formatting built in section 4, and the
description text shown in the detail panel leaked the machine-readable format.

- [x] 6.1 Add `pub fn parse_rfc3339_to_epoch(input: &str) -> Option<i64>` to `util/datetime.rs`
      (own `civil_to_days` inverse of `epoch_to_ymd`; handles `Z` and `±HH:MM` offsets; no new
      crate dependency), with unit tests for offset math, `Z` suffix, malformed input, and a
      round trip against `epoch_to_ymd`
- [x] 6.2 In `map_order_product`, parse `date_purchased`/`file_last_modified` with
      `parse_rfc3339_to_epoch` and set `LibraryItem.date_added` / `LibraryItem.date_updated`
      from the results; stop building `desc` from the raw date strings
- [x] 6.3 Add `pub date_updated: Option<i64>` to `LibraryItem` (mirrors `date_added`); `new()`
      sets it to `None` (stub data has no "last modified by publisher" concept — only real SDK
      data populates it)
- [x] 6.4 Add unit tests in `services::sdk` asserting `date_added`/`date_updated` are parsed
      correctly and that `desc` never contains a raw timestamp

## 7. Fix: `format_relative` had no month/year buckets

`format_relative` jumped straight from "N weeks ago" (max 29 days) to calendar-day formatting
("Jan 5" / "Jan 5, 2023") for anything 30+ days old — so an item updated 5 months ago showed a
bare date instead of "5 months ago".

- [x] 7.1 Add `months_between(ts, now) -> i64` using calendar year/month/day arithmetic (not a
      fixed day-count division, which drifts against short months)
- [x] 7.2 Replace the "Mon D" / "Mon D, YYYY" buckets in `format_relative` with "N months ago"
      (1-11 months) and "N years ago" (12+ months)
- [x] 7.3 Add unit tests for 1, 5, 11, 12, and 25 month boundaries using a
      calendar-anchored helper (`months_ago_anchored`) so tests don't depend on which day of the
      month they happen to run

## 8. Detail panel — "Updated" row

- [x] 8.1 Extract the "Added" row's div-plus-tooltip construction into a shared
      `render_relative_date_value(item_id, slot, ts) -> AnyElement` helper
- [x] 8.2 Add an "Updated" row using `item.date_updated`, mirroring "Added" (omitted when `None`)
- [x] 8.3 Add `detail.field_updated` = "Updated" to `i18n/en.yaml` (and mirror in `de.yaml` /
      `fr.yaml` if those are kept in sync as part of this change)

## 9. Re-verification

- [x] 9.1 `cargo test --all-features --workspace` — all unit + doc tests pass
- [x] 9.2 `cargo check --all-targets` — no compile errors
- [x] 9.3 `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings
- [x] 9.4 `cargo +nightly fmt --all -- --check` — no formatting diffs
