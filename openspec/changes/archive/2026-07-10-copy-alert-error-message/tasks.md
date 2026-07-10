## 1. Copy control

- [x] 1.1 In `alert_history_view.rs::render_entry_row`, wrap the message row in `.id(...)`
  + `.group(...)` (unique per entry, e.g. `format!("alert-msg-{}", entry.id)`), following
  `detail_panel_view.rs::copyable_value`'s hover-group pattern
- [x] 1.2 Add a hover-revealed `gpui_component::clipboard::Clipboard` next to the message
  text, with `.value(message.clone())` and a tooltip using a new `alert_history.copy_tooltip`
  i18n key
- [x] 1.3 Import `gpui_component::clipboard::Clipboard` in `alert_history_view.rs`

## 2. Localization

- [x] 2.1 Add `alert_history.copy_tooltip` key to `en.yaml`, `de.yaml`, `fr.yaml`

## 3. Verify

- [x] 3.1 Run `cargo check --workspace --all-targets`
- [x] 3.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 3.3 Run `cargo fmt --all -- --check`
- [x] 3.4 Run `cargo test --workspace`
- [x] 3.5 Manually trigger a failing operation (e.g. failed collection delete), open
  "Window > Show Alert History", hover the resulting row, click the copy control, and paste
  to confirm the clipboard contains the exact error message text
