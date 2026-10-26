# Design

## Context

See proposal.md for motivation. What the code does today (all in `crates/dtrpg-ui`):

- **No animation exists.** Nothing in the crate uses `with_animation`, `with_spring`, or `reduce_motion`.
- **Rows are addressed by index.** Thumbs and Grid are `uniform_list`s (`catalog-thumbs`, `catalog-grid`) whose callback reads `visible_items_slice(range)`; List and grouped List are gpui-kit `DataTable`s refreshed with `TableState::refresh`. When `invalidate_cache` swaps the visible-item `Arc`, row N shows a different item in the same frame. This is the "jump".
- **State changes swap whole trees.** `render_catalog_view` returns a different subtree for loading skeleton, empty/no-match, and each `(presentation, grouped)` pair. There is no frame in which old and new coexist.
- **Covers switch in one frame.** `CoverCache::get` returns `Some` or `None`; `None` draws the generated placeholder, `Some` draws the image.
- **Grid reflow lags a frame.** `items_per_row` is recomputed in render from the width of the previous draw, and `on_prepaint` notifies when the width changes.
- **Available primitives.** gpui provides `Animation`, `AnimationExt::with_animation` (state keyed by element id, honors `App::reduce_motion()` by rendering the end state and scheduling no frames), `with_spring`, easing functions, and `Animation::with_max_fps`. gpui-base adds `cubic_bezier`. `uniform_list` and `DataTable` have no keyed items, no enter/exit/move hooks. `App::reduce_motion` defaults to false and nothing on macOS sets it.

Governing rule: `dtrpg/.claude/rules/ui-performance.md`.

## Goals / Non-Goals

**Goals:**
- Remove the flicker and jump on filter, sort, search, grouping, presentation, and data changes with a short fade.
- Fade covers in on arrival only.
- Zero added work in any frame without a running transition.
- Follow the OS reduce-motion setting.

**Non-Goals:**
- Per-item enter, exit, and reorder (FLIP) animation. See Decision 1.
- Animating scroll, hover, selection, or progress.
- Animating the sidebar, inspector, dialogs, or the filters pane.

## Decisions

### 1. Transition the region, not the rows (ADR 0011)

The content area fades from 0.4 to 1.0 opacity over 120 ms (ease-out) when the visible set changes. Opacity multiplies already-computed quads; it changes no layout, so scroll offset and virtualization are untouched.

Alternatives rejected:
- *Per-item FLIP (move/enter/exit).* Needs each item's previous position by id. `uniform_list` and `DataTable` own row placement and expose none. Exiting items are no longer rendered, so they would need retained "ghost" rows. That is per-frame work for every visible item and a second render path, against the performance rule.
- *Spring on card positions.* Same position problem, plus continuous frames while settling.
- *Animate the scroll offset.* Does not address the content swap.

Revisit per-item motion only if the region fade proves insufficient, and then for Grid only, with a measured budget.

### 2. A revision counter says when the visible set changed

`Library::invalidate_cache` already builds the visible items. In the same pass it computes a hash of the item ids in order and stores it with the cache. The controller exposes `visible_revision()`, which advances only when the hash differs from the previous one. The catalog view stores the last revision it drew; when it differs (or the presentation, grouping, or content state kind changed) it starts a transition. This runs when the result changes, not per frame, and adds no catalog scan (the render guard test keeps passing). A thumbnail arrival, availability tick, or progress update does not change the hash, so it does not start a transition and does not disturb scroll.

If a transition is still running when the revision changes again (typing in search), the running transition continues; a new one does not restart from the low opacity. This prevents a strobe.

### 3. One helper, one place for constants

A small `motion` module in `dtrpg-ui` holds `fade_in(element, id, revision)` over `with_animation`, with the id built from a name and the revision so a new revision restarts the animation and an unchanged one continues it. Durations and the starting opacity live in the crate's `consts.rs` (created if absent), per the single-constants-location rule: region fade 120 ms, cover fade 160 ms, start opacity 0.4.

### 4. Cover fade only on arrival

`CoverCache` gains a `fresh` set of item ids whose image was inserted while they had none. A card whose id is in `fresh` renders its image through the same helper; others render as today. A one-shot timer removes the id after the fade, so a card scrolled back into view later shows the cached image at once. The per-card cost is one set lookup next to the existing `get`.

### 5. Loading and empty states

The content fade starts when the content state kind changes from skeleton or empty to content. The old state is not retained for a fade-out: the content starts at 0.4 opacity over the same surface, so the cut is hidden without a second tree.

### 6. Reduce motion

On macOS, read `NSWorkspace.accessibilityDisplayShouldReduceMotion` through `objc2-app-kit` at startup and on each application activation, and pass it to `App::set_reduce_motion`. Re-reading on activation avoids a notification observer and still catches a change made in System Settings (the user has to leave the app to make it). gpui's `with_animation` then renders the end state and schedules no frames. Other platforms report false. The read is isolated in one function so the mapping can be unit-tested.

### 7. Frame-cost budget and verification

Budget:
- No transition running: no added frame work beyond one `u64` compare in the catalog view and one set lookup per visible card.
- Transition running (at most 120 ms, about 7 frames): one opacity multiply on the content container; median frame time at most 0.5 ms above the same view with the transition disabled, and no frame over the 16.6 ms budget that the disabled case would meet.
- Cover fades: at most the visible cards animate; each fade is 160 ms.
- `Animation::with_max_fps` is not used; the durations are short enough that capping would only add judder.

Verification:
- Unit tests: revision unchanged for an identical id list and changed for a reorder; the OS-setting mapping; `fresh` set cleared after the timer.
- gpui tests: with reduce motion on, a transition schedules no animation frame (`simulate_next_frame` count is 0); idle window schedules none either.
- The existing render guard test (`controllers/library/tests/render_guard.rs`) still passes.
- Manual measurement recorded in the PR: render-time log of `render_catalog_view` (tracing span) over a 10,000-item library for a filter change, a presentation switch, and a cover burst, with transitions on and off.

## Risks / Trade-offs

- [Typing in search strobes the content] → A running transition is not restarted (Decision 2); search is already debounced.
- [Opacity on a large subtree costs fill rate] → gpui applies opacity per quad without an offscreen layer; confirmed by the measurement above before merge.
- [The fade hides a real delay] → It is 120 ms; loading states keep their own indicators.
- [Rows still jump inside the fade] → Accepted: the fade masks the swap; per-item motion is a non-goal until measured need.
- [Overlap with `simplify-catalog-status-indicators`] → It edits the same card and row rendering. Land it first or rebase this change onto it; the transition code touches the container and the cover, not the status indicators.
