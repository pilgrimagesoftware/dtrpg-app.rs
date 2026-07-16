## Why

Every time the app is launched, the catalog resets to its defaults (all titles, title sort, ungrouped, grid view, empty search). Users who prefer a different sort order, grouped view, or a particular sidebar filter have to re-apply their preferences each session. Remembering these values eliminates that friction.

## What Changes

- Persist the following catalog view state values between sessions: sidebar filter, sort method, grouping (on/off), and presentation mode (List / Thumbs / Grid).
- Search query is intentionally **not** persisted — it is ephemeral and should always start empty.
- On launch, load persisted values and initialize `LibraryController` from them instead of from hardcoded defaults.
- On each user-driven change (filter, sort, grouped toggle, presentation toggle), save the new values immediately.

## Capabilities

### New Capabilities

- `catalog-view-state-persistence`: Load and save catalog view preferences (filter, sort, grouped, presentation) to user-defaults across app sessions.

### Modified Capabilities

*(none — no existing spec-level behavior changes)*

## Impact

- `dtrpg-ui/src/data/catalog_view_prefs.rs` (new): persistence struct / free functions.
- `dtrpg-ui/src/controllers/library.rs`: load preferences on init; save on each mutation method.
- No API, SDK, or settings panel changes.
- `SidebarFilter::Publisher(Arc<str>)` requires special handling: if the stored publisher no longer appears in the user's library at launch, fall back to `AllTitles`.
