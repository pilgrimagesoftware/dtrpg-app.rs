## 1. Apply Monospace Font to the API Key Hint

- [x] 1.1 In `settings_account_view.rs`, in `render_authenticated`, add `.font_family("Menlo")` to the hint `div` (the `div().text_xs().text_color(colors.text_tertiary).child(hint.clone())` element inside the `if let Some(hint)` block)
- [x] 1.2 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 2. Verification

- [ ] 2.1 Build and run the app; sign in; open Settings → Account; confirm the API key hint row renders in a visually distinct monospaced font
- [ ] 2.2 Confirm the "Account" label and email address above the hint remain in the proportional application font
