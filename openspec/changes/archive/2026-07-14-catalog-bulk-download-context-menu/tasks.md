## 1. Controller: Batch Download Methods

- [x] 1.1 In `crates/dtrpg-ui/src/controllers/library.rs`, add
      `pub fn download_all_for_collection(&mut self, collection_id: u64, cx: &mut Context<Self>)`:
      look up the `CollectionEntry` in `self.collections` by id, collect `(id, title)` pairs for
      every `self.catalog` item where `member_ids_contain(&entry.member_ids, item.order_product_id, item.product_id)`
      is true, then call `self.enqueue_download(&id, title, cx)` per collected pair
- [x] 1.2 Add `pub fn download_all_for_publisher(&mut self, publisher: &str, cx: &mut Context<Self>)`:
      collect `(id, title)` pairs for every `self.catalog` item where `item.publisher.as_ref() == publisher`,
      then call `self.enqueue_download(&id, title, cx)` per collected pair
- [x] 1.3 No-op gracefully if the collection id doesn't exist or the publisher has no matching
      items (empty iterator, no panic)

## 2. Collection Sidebar Context Menu

- [x] 2.1 In `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`, add a "Download All" `PopupMenuItem`
      to the collection row's existing `context_menu` (before or after "Reload", ahead of
      "Delete"), calling `entity.update(cx, |ctrl, cx| ctrl.download_all_for_collection(col_id, cx))`
- [x] 2.2 Add the `collections.download_all` key to `crates/dtrpg-ui/i18n/en.yaml`, `de.yaml`,
      and `fr.yaml`

## 3. Publisher Group Header Context Menu

- [x] 3.1 In `crates/dtrpg-ui/src/ui/views/catalog_view.rs`, add an `entity: Entity<LibraryController>`
      parameter to `render_group_header` and attach `.context_menu(...)` with a "Download All"
      item calling `entity.update(cx, |ctrl, cx| ctrl.download_all_for_publisher(&publisher, cx))`
- [x] 3.2 Update both `render_group_header` call sites (grid ~line 1257, thumbs ~line 1364) to
      pass the controller entity through
- [x] 3.3 Add the equivalent context menu to `GroupedCatalogListDelegate::render_td`'s
      `GroupedRow::Header` branch (List presentation), using `self.controller` (already held by
      the delegate) and the row's `publisher`/`count` fields
- [x] 3.4 Add the `catalog.publisher_download_all` key to `crates/dtrpg-ui/i18n/en.yaml`,
      `de.yaml`, and `fr.yaml`

## 4. Build and Quality

- [x] 4.1 `cargo check --workspace`
- [x] 4.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 4.3 `cargo test --workspace`

## 5. Tests

- [x] 5.1 Add a controller-level test (or extend the closest existing coverage) asserting
      `download_all_for_collection` enqueues every not-yet-downloaded member item and skips
      already-downloaded ones. Delivered as unit tests on the extracted pure
      `collection_download_targets` helper (membership selection); the "skip already-downloaded"
      behavior is enforced downstream by `enqueue_download`'s existing `missing_file_indices`
      no-op contract (already covered by `missing_file_indices_*` tests) rather than re-tested
      here — this file has no `gpui::test`/`TestAppContext` harness to exercise `&mut self, cx`
      methods directly. End-to-end skip behavior is confirmed manually in 6.2/6.4.
- [x] 5.2 Add an equivalent test for `download_all_for_publisher`. Same scope note as 5.1, via
      `publisher_download_targets`.

## 6. Manual Verification

- [x] 6.1 Right-click a collection with several undownloaded items, select "Download All", and
      confirm all of them start downloading (activity panel shows an entry per item)
- [x] 6.2 Right-click a collection where some items are already downloaded and confirm only the
      remaining items are enqueued
- [x] 6.3 Right-click a publisher group header in Grid, Thumbs, and List presentations and
      confirm "Download All" appears and works in all three
- [x] 6.4 Confirm re-selecting "Download All" while a prior batch is still queued/downloading
      does not duplicate any in-flight or queued downloads
