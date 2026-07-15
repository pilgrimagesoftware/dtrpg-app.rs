## 1. Locale Keys

- [ ] 1.1 Add `activity.creating_collection`, `activity.deleting_collection`,
      `activity.loading_thumbnails`, `activity.loading_thumbnails_remaining`,
      `activity.downloading_file`, `activity.downloading_file_entry`,
      `activity.session_setup_failed` to `i18n/en.yaml`
- [ ] 1.2 Add the same keys with German translations to `i18n/de.yaml`
- [ ] 1.3 Add the same keys with French translations to `i18n/fr.yaml`

## 2. Replace Hardcoded Strings

- [ ] 2.1 `controllers/library.rs`: replace both `format!("Creating collection '{name}'...")`
      call sites with `t!("activity.creating_collection", name = name)`
- [ ] 2.2 `controllers/library.rs`: replace `"Deleting collection…".to_string()` with
      `t!("activity.deleting_collection").to_string()`
- [ ] 2.3 `controllers/library.rs`: replace the `"Loading thumbnails…"` activity-start label
      with `t!("activity.loading_thumbnails")`
- [ ] 2.4 `controllers/library.rs`: replace the `"Loading thumbnails… ({remaining}
      remaining)"` progress label with `t!("activity.loading_thumbnails_remaining",
      remaining = remaining)`
- [ ] 2.5 `controllers/library.rs`: replace both download-progress `format!()` call sites
      (`"Downloading {title}..."` and `"Downloading {title} — {file_name}..."`) with
      `t!("activity.downloading_file", title = title)` and
      `t!("activity.downloading_file_entry", title = title, file_name = file_name)`
- [ ] 2.6 `controllers/settings.rs`: replace the hardcoded prefix in
      `format!("Session setup failed after sign-in: {}", e.0)` with
      `format!("{}: {}", t!("activity.session_setup_failed"), e.0)`

## 3. Build and Quality

- [ ] 3.1 `cargo check --workspace`
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace`
- [ ] 3.4 Grep all three locale files for each new key name to confirm no locale was missed
