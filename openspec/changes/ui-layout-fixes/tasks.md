## Tasks

- [x] 1. Fix reload doubles catalog: clear `self.catalog` in `start_load_inner` before `append_catalog_page` when `force_reload` is true (`crates/dtrpg-ui/src/controllers/library.rs`)
- [x] 2. Fix detail panel close button: move close button outside the `overflow_hidden` scroll container in `detail_panel_view.rs` so it is not clipped
- [x] 3. Fix file opener remove button: audit `settings_file_openers_view.rs` remove button click handler and `remove_file_opener` on the controller; wire the handler if missing
- [ ] 4. Fix account/avatar info: populate `email` from API auth response if available; if not, show the API key hint as the account identifier in the Account settings tab and avatar menu
- [ ] 5. Remove catalog right-resize handle: replace the three-panel `h_resizable` in `root_view.rs` with a flex row where the catalog is `flex_1` and the detail (when present) is `flex_none` at fixed 320 px with no resize splitter
- [ ] 6. Add First/Last pagination buttons: in `catalog_view.rs`, flank the `Pagination` widget with "First" and "Last" `Button` elements; disable each when already on the target page
- [ ] 7. Persist sidebar section collapse state: add `collections_collapsed: Option<bool>` and `publishers_collapsed: Option<bool>` to `UiPrefsFile`; add getters and a `save_sidebar_collapse` method to `UiPrefs`; in `sidebar_view.rs` read initial state from `UiPrefs` and persist on toggle
- [ ] 8. Fix collections not loading on startup: trace `replace_service` to confirm `load_collections` is called after auth; add a `tracing::debug!` to verify the authenticated service is reached; fix any silent failure
