## 1. Confirmation copy

- [x] 1.1 Add `catalog.remove_download_confirm_title` (interpolating the item title, e.g. `"Remove download of \"%{title}\"?"`) and `catalog.remove_download_confirm_description` (e.g. `"The local copy will be removed. You can download it again later."`) to `crates/dtrpg-ui/i18n/en.yaml`
- [x] 1.2 Add the same two keys to `crates/dtrpg-ui/i18n/de.yaml` and `crates/dtrpg-ui/i18n/fr.yaml`

## 2. Catalog context menu

- [x] 2.1 In `catalog_view.rs`'s first `context_menu`/`action_remove_download` handler (Grid presentation, ~line 544), wrap the `ctrl.remove_download(&remove_id, cx)` call in `window.open_alert_dialog(cx, |alert, _, _| alert.confirm().title(...).description(...).on_ok(...))`, using the item's title already in scope
- [x] 2.2 Apply the same wrapping to the second `action_remove_download` handler (Thumbs presentation, ~line 1650/1912 — same pattern, duplicated per row-presentation closure)

## 3. Item popover

- [x] 3.1 In `item_popover_view.rs`'s download toggle `on_click`, split the `is_downloaded` branch: `remove_download` now goes through `window.open_alert_dialog` with the item title from `item_title`/`item_title_for_download`; `enqueue_download` is unchanged

## 4. Detail panel

- [x] 4.1 In `detail_panel_view.rs`'s download button `on_click`, apply the same split: `remove_download` goes through `window.open_alert_dialog` using `download_title`; `enqueue_download` is unchanged

## 5. Verification

- [x] 5.1 `cargo build --workspace --all-features`
- [x] 5.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 5.3 Launch app and confirm: clicking "Remove Download" from each of the three entry points shows the dialog; confirming reverts status to Cloud; cancelling leaves the item Downloaded
