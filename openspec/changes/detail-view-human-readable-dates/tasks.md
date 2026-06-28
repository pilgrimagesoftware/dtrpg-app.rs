## 1. Data model — library.rs

- [ ] 1.1 Add `pub date_added: Option<i64>` field to `LibraryItem` (Unix seconds; `None` = unknown)
- [ ] 1.2 Add `date_added: Option<i64>` parameter to `LibraryItem::new()`; set `self.date_added = date_added`

## 2. Datetime utility — util/datetime.rs

- [ ] 2.1 Create `crates/dtrpg-ui/src/util/datetime.rs`; declare `pub mod datetime;` in `util/mod.rs`
- [ ] 2.2 Implement `pub fn format_relative(ts: i64) -> String` using `std::time::SystemTime` and `UNIX_EPOCH`; cover all label buckets: "just now", "N minutes ago", "N hours ago", "yesterday", "N days ago", "N weeks ago", "Mon D", "Mon D, YYYY"
- [ ] 2.3 Implement helper `fn epoch_to_ymd(ts: i64) -> (i32, u32, u32)` using Gregorian proleptic calendar arithmetic (no crate)
- [ ] 2.4 Implement `pub fn format_absolute(ts: i64) -> String` returning "Month D, YYYY at H:MM AM/PM" (e.g., "January 5, 2024 at 3:42 PM")
- [ ] 2.5 Add `#[cfg(test)] mod tests` in `datetime.rs` with unit tests for `format_relative` at each bucket boundary (59s, 60s, 59m, 1h, 23h, 24h, 47h, 48h, 6d, 7d, 29d, 30d) and at least two `format_absolute` cases

## 3. Stub data — util/stubs.rs

- [ ] 3.1 Add a helper in `stubs.rs`: `fn stub_date_added(added_order: u32) -> Option<i64>` — computes `SystemTime::now()` as Unix seconds and subtracts `added_order as i64 * 43200` (12 hours per rank unit)
- [ ] 3.2 Update all 46 `LibraryItem::new(...)` calls in `stubs.rs` to pass `stub_date_added(added_order)` as the new final argument

## 4. Detail panel — detail_panel_view.rs

- [ ] 4.1 In `render_metadata_table`, import `Tooltip` from `gpui_component::tooltip` and `SharedString` from `gpui`
- [ ] 4.2 Import `crate::util::datetime::{format_relative, format_absolute}`
- [ ] 4.3 After the existing `rows` loop, conditionally append an "Added" child when `item.date_added.is_some()`: render a `div().id(SharedString::from(format!("detail-added-{}", item.id))).tooltip(...)` wrapping the relative label text, where the tooltip calls `Tooltip::new(absolute_text).build(window, cx)`
- [ ] 4.4 Update `render_metadata_table` signature to accept `window: &mut Window` and `cx: &mut App` (needed for `Tooltip::build`); update its one call site in `render_detail_panel` accordingly

## 5. Verification

- [ ] 5.1 Run `cargo test --all-features -- datetime` — all datetime unit tests pass
- [ ] 5.2 Run `cargo check --all-targets` — no compile errors
- [ ] 5.3 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings
- [ ] 5.4 Open the detail panel for an item; confirm the "Added" row shows a relative label
- [ ] 5.5 Hover the "Added" value; confirm a tooltip appears with the full date and time
- [ ] 5.6 Confirm items with `added_order` values in each time bucket show the correct label format
