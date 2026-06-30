## Why

The masked API key hint (`abcd••••••••1`) is rendered in the same proportional font as the surrounding text. Because the hint mixes ASCII characters with Unicode bullet characters, proportional spacing makes the field harder to scan and looks inconsistent compared to how API keys appear in other tools. A monospaced font makes the hint immediately recognizable as a code/credential value and aligns the characters uniformly.

## What Changes

- The API key hint row in the authenticated Account section of the settings panel is rendered with a monospace font family.
- No other text in the settings view changes.

## Capabilities

### New Capabilities

### Modified Capabilities

## Impact

- **`dtrpg-ui/src/ui/views/settings_account_view.rs`**: Add `.font_family("monospace")` (or the GPUI equivalent) to the hint `div` in `render_authenticated`.
- No controller, model, or data changes.
