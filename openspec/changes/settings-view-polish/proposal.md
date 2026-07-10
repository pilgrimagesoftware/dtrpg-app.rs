## Why

Three Settings pages have small visual inconsistencies left over from their initial implementation: Account's Email/API Key rows aren't right-aligned like the Advanced page's Cache Details rows now are; Downloads' folder-picker and reveal buttons use raw emoji/unicode glyphs instead of the `gpui-component` icon set used everywhere else; and About shows only the version string with no way to identify exactly which build is running.

## What Changes

- Account section: right-align the Email and API Key row values, matching the label/value row pattern Advanced settings' Cache Details already uses.
- Downloads section: replace the "📂" and "↗" emoji-glyph buttons with `Button` + `IconName` icons (the pattern already used for the equivalent folder/reveal actions in the detail panel and item popover), each keeping its existing tooltip.
- About section: add build information (git commit short hash, build date, target platform) below the version string, and right-align all these values — via `DescriptionList` in its borderless, horizontal-axis configuration (already used elsewhere in this app), rather than hand-rolling another label/value row layout.

## Capabilities

### New Capabilities

- `settings-view-visual-consistency`: Account, Downloads, and About sections present label/value data consistently (right-aligned values), use the app's shared icon set for icon buttons instead of raw emoji glyphs, and About shows build information identifying the running build.

### Modified Capabilities

- None — `rust-settings-window-implementation` covers window mechanics (page navigation, persistence, focus handling), none of which change here; this is presentation-only within existing pages.

## Impact

- `dtrpg-ui/src/ui/views/settings_account_view.rs`: Email/API Key rows gain `justify_between`-based right-alignment.
- `dtrpg-ui/src/ui/views/settings_storage_view.rs`: "Change…" and reveal buttons become `Button::new(...).ghost().outline().icon(IconName::Folder)` / `.icon(IconName::FolderOpen)`, matching `detail_panel_view.rs`'s existing reveal-button pattern; hand-rolled glyph `div()`s removed.
- `dtrpg-ui/src/ui/views/settings_advanced_view.rs`: `render_about_section` gains a `DescriptionList` (bordered false, horizontal axis) for Version + the three new build-info fields.
- New `dtrpg-ui/build.rs`: captures git short commit hash, build timestamp, and target triple as compile-time `rustc-env` variables, following the existing pattern in `dtrpg-core/build.rs` (which only forwards Sentry config today). `dtrpg-ui` needs its own build script since `dtrpg-core` depends on `dtrpg-ui`, not the reverse, and the About section lives in `dtrpg-ui`.
- i18n: new `about.build_commit`/`about.build_date`/`about.build_target` labels (en/de/fr); existing `about.version`/`about.description` keys unchanged.
- No controller, data model, or persisted-state changes.
