## 1. Dependencies

- [ ] 1.1 Add `rust-i18n = "3"` to `[workspace.dependencies]` in the root `Cargo.toml`
- [ ] 1.2 Add `sys-locale = "0.3"` to `[workspace.dependencies]` in the root `Cargo.toml`
- [ ] 1.3 Add `rust-i18n` and `sys-locale` to `[dependencies]` in `crates/dtrpg-ui/Cargo.toml`

## 2. Locale Files

- [ ] 2.1 Create `crates/dtrpg-ui/i18n/en.yaml` with all string keys (see key inventory below)
- [ ] 2.2 Create `crates/dtrpg-ui/i18n/fr.yaml` as an English-value stub with a `# UNTRANSLATED STUB` header comment
- [ ] 2.3 Create `crates/dtrpg-ui/i18n/de.yaml` as an English-value stub with a `# UNTRANSLATED STUB` header comment

Key inventory for `en.yaml` (module.key: value):
```
sidebar.all_titles: "All Titles"
sidebar.recently_added: "Recently Added"
sidebar.on_this_device: "On This Device"
sidebar.in_the_cloud: "In the Cloud"
sidebar.publishers: "Publishers"
sidebar.collections: "Collections"
sidebar.app_name: "Libri"

toolbar.sort_by: "Sort by"
toolbar.sort_title: "Title"
toolbar.sort_publisher: "Publisher"
toolbar.sort_date_added: "Date Added"
toolbar.sort_pages: "Pages"
toolbar.sort_ascending: "Ascending"
toolbar.sort_descending: "Descending"
toolbar.group_by_publisher: "Group by Publisher"
toolbar.view_list: "List"
toolbar.view_thumbs: "Thumbs"
toolbar.view_grid: "Grid"
toolbar.sign_in: "Sign In…"
toolbar.log_out: "Log Out"
toolbar.page_size_label: "{n} / page"

catalog.no_titles_match: "No titles match."
catalog.library_empty: "Your library is empty."
catalog.try_clear_search: "Try clearing your search."
catalog.try_different_section: "Try selecting a different section."
catalog.col_title: "Title"
catalog.col_publisher: "Publisher"
catalog.col_system: "System"
catalog.col_pages: "Pages"
catalog.col_size: "Size"
catalog.col_added: "Added"
catalog.action_download: "Download"
catalog.action_remove_download: "Remove Download"
catalog.action_load_thumbnail: "Load Thumbnail"
catalog.action_show_in_finder: "Show in Finder"
catalog.action_show_in_explorer: "Show in Explorer"
catalog.action_show_in_files: "Show in Files"

detail.read_button: "Read"
detail.download_button: "Download"
detail.downloaded_button: "Downloaded"
detail.show_in_finder: "Show in Finder"
detail.show_in_explorer: "Show in Explorer"
detail.show_in_files: "Show in Files"
detail.tooltip_download_first: "Download this item first"
detail.field_system: "System"
detail.field_category: "Category"
detail.field_format: "Format"
detail.field_pages: "Pages"
detail.field_file_size: "File size"
detail.field_released: "Released"
detail.field_status: "Status"
detail.field_added: "Added"
detail.status_on_device: "On this device"
detail.status_in_cloud: "In the cloud"

settings.title: "Settings"
settings.account_title: "Account"
settings.not_signed_in: "Not signed in to DriveThruRPG"
settings.sign_in_prompt: "Sign in with your DriveThruRPG API key to access your library."
settings.log_out_button: "Log Out"
settings.storage_title: "Catalog Storage Location"
settings.storage_missing: "Storage folder does not exist. Downloads may fail."
settings.file_openers_title: "File Openers"
settings.file_openers_description: "Override which application opens a file type."
settings.file_openers_empty: "No overrides configured. Click Add to set up an opener."

activity.title: "Activity"
activity.empty: "No recent activity."
activity.empty_hint: "Activity will appear here as operations run."
```

## 3. i18n Module

- [ ] 3.1 Create `crates/dtrpg-ui/src/i18n/mod.rs` with the `rust_i18n::i18n!("i18n")` macro invocation (points to `crates/dtrpg-ui/i18n/`) and an `pub fn init()` function that reads `sys_locale::get_locale()`, strips the region tag, and calls `rust_i18n::set_locale()` (defaulting to `"en"`)
- [ ] 3.2 Declare `pub mod i18n;` in `crates/dtrpg-ui/src/lib.rs`
- [ ] 3.3 Call `dtrpg_ui::i18n::init()` (or `crate::i18n::init()`) early in app startup, before any view renders -- place this in `util/init.rs` or the top of `ui/windows/app.rs`

## 4. Replace Strings in Views

- [ ] 4.1 `sidebar_view.rs`: replace the 6 nav item label literals and the "Libri" wordmark with `t!()` calls
- [ ] 4.2 `toolbar_view.rs`: replace sort menu labels, direction labels, group toggle label, view tab labels ("List", "Thumbs", "Grid"), sign-in/log-out menu item labels, and the `filter_title()` function's match arms
- [ ] 4.3 `catalog_view.rs`: replace column name strings, empty-state messages, context menu item labels ("Download", "Remove Download"), and "Load Thumbnail" / platform reveal labels
- [ ] 4.4 `detail_panel_view.rs`: replace button labels ("Read", "Download", "Downloaded", reveal label), tooltip text, `DescriptionItem` labels, and status strings ("On this device", "In the cloud")
- [ ] 4.5 `settings_view.rs`: replace the "Settings" heading
- [ ] 4.6 `settings_account_view.rs`: replace "Account", "Not signed in to DriveThruRPG", sign-in prompt, and "Log Out" label
- [ ] 4.7 `settings_storage_view.rs`: replace "Catalog Storage Location" and the storage-missing warning
- [ ] 4.8 `settings_file_openers_view.rs`: replace "File Openers", the description, and the empty state message
- [ ] 4.9 `activity_panel_view.rs`: replace "Activity", "No recent activity.", and the empty-hint string

## 5. Build and Lint

- [ ] 5.1 Run `cargo check --workspace` -- no errors
- [ ] 5.2 Run `cargo clippy --all-targets --all-features -- -D warnings` -- no new warnings
- [ ] 5.3 Run `cargo fmt --all` -- no formatting changes
- [ ] 5.4 Run `cargo test --workspace` -- all tests pass (update any tests that assert on literal English strings)

## 6. Manual Verification

- [ ] 6.1 Launch the app on an English system locale and confirm all UI strings render correctly
- [ ] 6.2 Temporarily set `rust_i18n::set_locale("fr")` in `i18n::init()` and confirm the app still renders (with English stub values -- no crashes, no empty labels)
