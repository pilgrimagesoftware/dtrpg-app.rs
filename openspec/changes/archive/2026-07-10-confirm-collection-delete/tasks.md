## 1. Confirmation copy

- [x] 1.1 Add `collections.delete_confirm_title` (interpolating the collection name, e.g. `"Delete \"%{name}\"?"`) and `collections.delete_confirm_description` (e.g. `"This collection will be permanently deleted. This cannot be undone."`) to `crates/dtrpg-ui/i18n/en.yaml`
- [x] 1.2 Add the same two keys to `crates/dtrpg-ui/i18n/de.yaml` and `crates/dtrpg-ui/i18n/fr.yaml`

## 2. Wire the confirmation dialog

- [x] 2.1 In `sidebar_view.rs`'s `render_collection_row`, capture `window` (currently `_`) in the "Delete" `PopupMenuItem`'s `on_click` closure
- [x] 2.2 Wrap the existing `ctrl.delete_collection(col_id, cx)` call in `window.open_alert_dialog(cx, |alert, _, _| alert.confirm().title(...).description(...).on_ok(...))`, using `row.name` for the dialog copy

## 3. Verification

- [x] 3.1 `cargo build --workspace --all-features`
- [x] 3.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 3.3 Launch app: select "Delete" on a collection, confirm the dialog appears with the collection's name, confirming deletes it and cancelling leaves it untouched
