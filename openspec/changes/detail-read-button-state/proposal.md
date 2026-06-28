## Why

The "Read" button in the detail panel is always enabled regardless of whether the item has been downloaded, making it possible to click an action that cannot succeed. Users need a clear signal that reading requires a local file first.

## What Changes

- Disable the "Read" button when `item.status != ItemStatus::Downloaded`
- Show a tooltip on the disabled button explaining the item must be downloaded before reading
- Keep the button visually present but styled to communicate the disabled state

## Capabilities

### New Capabilities

- `detail-read-button-download-guard`: The Read button in the detail panel is disabled when the item is not downloaded, with a tooltip indicating the download prerequisite.

### Modified Capabilities

- `rust-main-window-library-layout`: The detail panel action button area gains conditional disabled state on the Read button.

## Impact

- `detail_panel_view.rs`: Read button gains disabled styling and tooltip when `!is_downloaded`; no change to the download or reveal buttons.
