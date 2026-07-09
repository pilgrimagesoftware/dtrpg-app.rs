## 1. Activity controller

- [x] 1.1 Add `ActivityController::log_alert(&mut self, label: impl Into<Arc<str>>, message: String, cx: &mut Context<Self>)` to `controllers/activity.rs` — allocates an id from `self.next_id`, builds an `AlertEntry { id, label, message, occurred_at: SystemTime::now() }`, pushes it via the existing private `push_alert` helper, and emits `ActivityChanged`
- [ ] 1.2 Unit test: calling `log_alert` appends an entry to `alert_snapshot().entries` with the given label/message, without adding anything to `in_progress` or `recent`

## 2. Wire up failure branches

- [x] 2.1 In `add_item_to_collection` (`controllers/library.rs`), capture the collection's
  name before the optimistic mutation, and in the failure branch call
  `ctrl.activity.update(cx, |a, cx| a.log_alert(format!("Add to collection '{collection_name}'"), e.message.clone(), cx))`
  alongside the existing `cx.emit(CollectionMemberAddFailed { .. })`
- [x] 2.2 In `remove_item_from_collection`, apply the same pattern with a
  "Remove from collection '{collection_name}'" label

## 3. Verify

- [x] 3.1 Run `cargo check --workspace --all-targets`
- [x] 3.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 3.3 Run `cargo +nightly fmt --all -- --check`
- [x] 3.4 Run `cargo test --workspace`
- [ ] 3.5 Manually open the Manage Collections dialog against a stub/error-mode
  collections service, trigger an add-to-collection failure, confirm the toast still
  appears, then open "Window > Show Alert History" and confirm the failure is listed
