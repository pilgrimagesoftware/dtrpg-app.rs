## 1. Account section right-alignment

- [x] 1.1 In `settings_account_view.rs`, update the Email row's wrapping `div()` to `justify_between` and add `.text_right()` (or `.flex_1()` + right alignment) to the value div
- [x] 1.2 Apply the same change to the API Key row
- [ ] 1.3 Run the app and visually confirm both rows right-align without breaking the existing `items_baseline()` vertical alignment

## 2. Downloads icon buttons

- [x] 2.1 In `settings_storage_view.rs`, import `gpui_component::button::{Button, ButtonVariants}` and `gpui_component::IconName` (if not already imported)
- [x] 2.2 Replace the "Change…" hand-rolled `div()` with `Button::new("change-storage").ghost().outline().icon(IconName::Folder).tooltip(t!("settings.storage_change_tooltip").to_string())`, keeping the existing `on_click` folder-picker logic
- [x] 2.3 Replace the reveal hand-rolled `div()` with `Button::new("reveal-storage").ghost().outline().icon(IconName::FolderOpen).tooltip(reveal_label)`, keeping the existing `on_click` reveal logic
- [x] 2.4 Remove now-unused manual tooltip/hover styling code specific to the old `div()` buttons

## 3. Build info capture

- [x] 3.1 Create `dtrpg-ui/build.rs`: shell out to `git rev-parse --short HEAD`, falling back to `"unknown"` on any failure (git not found, not a git checkout, non-zero exit); emit `cargo:rustc-env=DTRPG_GIT_HASH=<hash>`
- [x] 3.2 Capture a build date (UTC, e.g. `YYYY-MM-DD`) and emit `cargo:rustc-env=DTRPG_BUILD_DATE=<date>`
- [x] 3.3 Re-emit Cargo's `TARGET` build-script env var as `cargo:rustc-env=DTRPG_TARGET=<target>` so it's visible to `env!()` in the library crate
- [x] 3.4 Add a `build = "build.rs"` entry to `dtrpg-ui/Cargo.toml` if not automatically picked up (not needed — Cargo autodetects `build.rs` at the crate root; confirmed `dtrpg-core` relies on the same autodetection)
- [x] 3.5 Add `pub mod build_info` to `dtrpg-ui` (e.g. in `lib.rs` or `data/mod.rs`) exposing `pub const GIT_HASH: &str = env!("DTRPG_GIT_HASH")`, `BUILD_DATE`, `TARGET` as associated consts

## 4. About section rework

- [x] 4.1 In `settings_advanced_view.rs`, import `gpui_component::description_list::{DescriptionItem, DescriptionList}` and `crate::build_info` (or wherever it's exposed)
- [x] 4.2 Replace `render_about_section`'s plain version/description divs with: app name heading (unchanged), a `DescriptionList::horizontal().columns(1).bordered(false)` containing Version, Commit, Build Date, and Target rows (each value wrapped in `div().w_full().text_right().child(...)` for right-alignment — `DescriptionList` doesn't right-align values on its own), then the existing tagline (`about.description`) as free-standing text below the list
- [x] 4.3 Add `settings.about_version_label`, `settings.about_commit_label`, `settings.about_build_date_label`, `settings.about_target_label` i18n keys (en/de/fr) (implemented under the existing `about:` namespace as `about.version_label`/`about.commit_label`/`about.build_date_label`/`about.target_label`, consistent with the file's existing `about.version`/`about.description` keys)

## 5. Verification

- [x] 5.1 Run `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo +nightly fmt -- --check`, `cargo test --all-features --workspace`
- [ ] 5.2 Manually verify: Account's Email/API Key rows right-align
- [ ] 5.3 Manually verify: Downloads' "Change…" and reveal buttons show folder icons, still open the folder picker / reveal the storage location, and still show their tooltips on hover
- [ ] 5.4 Manually verify: About shows a real (non-"unknown") git commit hash, a plausible build date, and the correct target triple for a normal `cargo build` in this git checkout
- [ ] 5.5 Manually verify: About's rows are right-aligned with no visible borders
