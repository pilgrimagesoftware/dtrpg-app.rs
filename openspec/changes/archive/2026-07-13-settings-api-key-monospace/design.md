## Context

GPUI's `Styled` trait provides `.font_family(impl Into<SharedString>)` which sets the font family for the element's text style. The method takes an explicit font name string — GPUI does not resolve CSS generic family names like "monospace". The hint `div` in `render_authenticated` (`settings_account_view.rs`) is the sole target.

## Goals / Non-Goals

**Goals:**
- Apply a monospaced font to the API key hint row only.

**Non-Goals:**
- Changing any other text in the settings view.
- Registering or bundling a custom monospace font.
- Cross-platform font fallback chains.

## Decisions

**Font: platform-conditional `MONOSPACE_FONT` const**

A `#[cfg]`-gated const in `settings_account_view.rs` selects the right system monospace per platform:
- macOS: `"Menlo"` — ships since 10.6, no bundling required
- Windows: `"Consolas"` — ships with Windows Vista+
- Other (Linux): `"Liberation Mono"` — included in most distros; Courier New is a fallback if absent

Alternative considered: hardcoded `"Menlo"` — simple but silently falls back to the default proportional font on non-macOS platforms. Platform-conditional const is a one-time cost with correct behavior everywhere.

## Risks / Trade-offs

- [Linux font availability] "Liberation Mono" is standard on most distros but not guaranteed on minimal installs. The text falls back to the system default if absent — non-critical for a macOS-primary app.
