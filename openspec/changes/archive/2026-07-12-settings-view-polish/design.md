## Context

Three unrelated small inconsistencies, bundled into one change because they're all quick visual fixes to Settings pages touched by recent work (cache-details right-alignment, chrome-free settings window):

- **Account** (`settings_account_view.rs`): the Email and API Key rows are `div().flex().items_baseline().gap(px(6.0))` — label then value, left-aligned, no `justify_between`. Advanced settings' Cache Details rows (`settings_advanced_view.rs`'s `row_frame`) already establish a right-aligned label/value pattern; Account predates that and was never updated to match.
- **Downloads** (`settings_storage_view.rs`): the "Change…" and reveal buttons are hand-rolled `div()`s with `.child(div().text_sm().text_color(text_primary).child("📂"))` / `.child("↗")` — raw glyphs, not the `gpui-component` `IconName` set every other icon button in the app uses (e.g. `detail_panel_view.rs`'s reveal button: `Button::new("detail-tab-reveal").ghost().outline().icon(IconName::FolderOpen).tooltip(...)`).
- **About** (`settings_advanced_view.rs`'s `render_about_section`): shows only app name, `CARGO_PKG_VERSION`, and a tagline. No way to tell exactly which build is running (useful for bug reports) — no git commit, build date, or target triple anywhere in the app.

`DescriptionList` (`gpui_component::description_list`) is already used four times in `detail_panel_view.rs`/`item_popover_view.rs`, always as `DescriptionList::vertical().columns(2).bordered(false)` — but "vertical"/"horizontal" set the *internal* item layout (`Axis`), not row flow: `vertical()` stacks each item's label above its value; `horizontal()` puts label and value side by side on one line via a fixed-width label column (`.label_width()`) and a `flex_1` value cell. No existing call site in this codebase uses `.horizontal()` yet. Checked the component's `render()` directly: the value cell is a plain `div().flex_1()...child(value)` with no alignment applied — `DescriptionList` does not right-align values on its own; the *label column* is what gets a fixed width, not the value.

## Goals / Non-Goals

**Goals:**

- Account's Email/API Key rows right-align the same way Advanced's Cache Details rows do.
- Downloads' two icon buttons use `IconName` icons instead of emoji glyphs, keeping their existing tooltips and click behavior unchanged.
- About shows build info (commit, build date, target) and right-aligns Version + the new fields, using `DescriptionList` in its horizontal/borderless configuration per the request, with values explicitly right-aligned (the component doesn't do this on its own — see Decisions).

**Non-Goals:**

- No new build/release tooling (versioning scheme, changelog, CI changes) — this only surfaces information already available at compile time (or trivially captured by a small build script) in the UI.
- No reproducible-build guarantees — embedding a build timestamp is inherently non-reproducible; not a concern for a desktop app that isn't targeting bit-for-bit reproducible builds today.
- Account and Downloads keep their current hand-rolled `div()`-based layouts otherwise; only the specific rows/buttons named above change. Not a rewrite of either section.

## Decisions

### Account: reuse the `row_frame` pattern, not a new component

`settings_advanced_view.rs`'s `row_frame` (label left, value right via `justify_between` + `.text_right()` on the value div) already exists and is exactly the layout Account needs. Rather than importing it across modules (it's currently private to `settings_advanced_view.rs` and tailored to that file's three-line label/value/description shape, which Account's two rows don't need — no description line), Account's Email/API Key rows get the same two-part treatment inline: `div().flex().justify_between().items_baseline().gap(px(6.0))` wrapping the existing label div and a value div with `.text_right()` added. Minimal diff, no new shared abstraction for a two-call-site need.

### Downloads: `IconName::Folder` for "Change…", `IconName::FolderOpen` for reveal

Both icons already exist in `gpui-component`'s built-in set (confirmed in use elsewhere in this app). `IconName::FolderOpen` matches `detail_panel_view.rs`'s reveal button exactly (same action: open the OS file manager at a path) — reusing that association keeps the icon meaning consistent app-wide rather than picking independently. `IconName::Folder` (closed) for "Change…" distinguishes the picker action (choose a new folder) from the reveal action (open an existing one) using the same icon family.

Buttons become `Button::new("change-storage").ghost().outline().icon(IconName::Folder).tooltip(t!("settings.storage_change_tooltip").to_string())` and `Button::new("reveal-storage").ghost().outline().icon(IconName::FolderOpen).tooltip(reveal_label)`, replacing the hand-rolled `div()`+glyph+manual click/tooltip wiring — `Button` already handles hover/pressed states and tooltip positioning, which the current hand-rolled version reimplements ad hoc.

### About: build info captured via a new `dtrpg-ui/build.rs`

`dtrpg-core` (the binary) depends on `dtrpg-ui` (the library) — not the reverse — so `dtrpg-core/build.rs`'s existing Sentry-config forwarding can't be reused; `dtrpg-ui` needs its own build script, following the same `cargo:rustc-env` forwarding shape:

```rust
// dtrpg-ui/build.rs
fn main() {
    let git_hash = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=DTRPG_GIT_HASH={git_hash}");

    let build_date = /* current UTC date, e.g. via `date -u +%Y-%m-%d` or a
                         minimal manual UTC calculation — no new dependency */;
    println!("cargo:rustc-env=DTRPG_BUILD_DATE={build_date}");

    // TARGET is already provided by Cargo to build scripts — just re-emit it
    // as a normal env! target for the library crate, since build-script env
    // vars aren't automatically visible to `env!()` in the crate being built.
    println!("cargo:rustc-env=DTRPG_TARGET={}", std::env::var("TARGET").unwrap());
}
```

Exposed as `pub mod build_info` in `dtrpg-ui` (`pub const GIT_HASH: &str = env!("DTRPG_GIT_HASH")`, etc.) for `settings_advanced_view.rs` to consume. Git-hash lookup gracefully degrades to `"unknown"` if `git` isn't on `PATH` or the source isn't a git checkout (e.g. a source tarball) — matches this repo's existing tolerance pattern for optional build-time context (`dtrpg-core/build.rs`'s Sentry vars are similarly optional/empty-string-tolerant).

_Alternative considered:_ the `vergen`/`built` crates, which do this generically. Rejected — pulling in a new build-dependency for three env vars is disproportionate; the manual `cargo:rustc-env` approach already has a working precedent in this exact codebase (`dtrpg-core/build.rs`).

### About: `DescriptionList::horizontal().bordered(false)`, with values wrapped for right-alignment

Matches the parenthetical request directly, but since `DescriptionList` doesn't right-align its value cells by default (checked the component source — the value cell is a plain `flex_1` div with no alignment styling), each `DescriptionItem::value(...)` call wraps its content in `div().w_full().text_right().child(...)` to get the right-aligned look, e.g.:

```rust
DescriptionList::horizontal()
    .columns(1)
    .bordered(false)
    .child(DescriptionItem::new(t!("about.version_label"))
               .value(div().w_full().text_right().child(env!("CARGO_PKG_VERSION"))))
    .child(DescriptionItem::new(t!("about.build_commit"))
               .value(div().w_full().text_right().child(build_info::GIT_HASH)))
    // ... build date, target
```

`.columns(1)` (not the `.columns(2)` used elsewhere in this codebase) since About's four short fields read better as a single stacked list in a fairly narrow settings-page column, rather than a 2-column grid tuned for wider detail-panel contexts. The existing tagline (`about.description`) stays as free-standing prose below the list, not folded into a row — it isn't a label/value fact.

## Risks / Trade-offs

- **[Risk]** `git rev-parse` requires `git` on `PATH` and a `.git` directory at build time; packaged release builds run from CI where both are typically present, but a source tarball build would show `"unknown"`. → Acceptable; matches the existing optional-build-context tolerance pattern, and `"unknown"` is still more useful than nothing.
- **[Trade-off]** Wrapping every `DescriptionList` value in a manual right-align `div()` is a small amount of repetition per row (4 rows). → Acceptable at this scale; if a fifth `DescriptionList` usage elsewhere in the app wants the same right-aligned-value treatment, promoting this to a small shared helper becomes worth it then, not preemptively here.
