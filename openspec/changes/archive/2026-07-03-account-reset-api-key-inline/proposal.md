## Why

The Account settings section stacks the Log Out and Reset API Key buttons vertically below the account info, which wastes vertical space and visually separates the Reset API Key action from the information it relates to. Placing Reset API Key inline to the right of the account info row makes the relationship clearer and frees up space for the Log Out button to stand alone as the primary action.

## What Changes

- **Account info row layout**: The "Account" label + "Signed in to DriveThruRPG" column gains a Reset API Key button on the right side of the same row, right-aligned.
- **Actions section**: The Reset API Key button is removed from the vertical button stack below the divider; only the Log Out button remains there.

## Capabilities

### New Capabilities

### Modified Capabilities

## Impact

- **`dtrpg-ui/src/ui/views/settings_account_view.rs`**: Restructure `render_account_section` — the identity column becomes a flex row with `justify_between`, adding the Reset API Key button on the right; remove it from the actions column below the divider.
- No model, controller, dependency, or API changes.
