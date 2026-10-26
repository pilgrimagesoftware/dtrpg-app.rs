# Tasks

## 1. Motion foundation

- [ ] 1.1 Add the motion constants (region fade 120 ms, cover fade 160 ms, start opacity 0.4) to the crate's `consts.rs`, and verify `grep` finds no duration literals outside it
- [ ] 1.2 Add the `motion` module with `fade_in(element, id, revision)` over `with_animation`, with a gpui test that a revision change restarts the animation and an unchanged revision continues it
- [ ] 1.3 Add a gpui test that with `set_reduce_motion(true)` the helper renders the end state and `simulate_next_frame` runs no callbacks, and that an idle window schedules none

## 2. Reduce motion from the OS

- [ ] 2.1 Add `objc2-app-kit` (latest stable, checked on crates.io) with the narrowest features, and verify `cargo check -p dtrpg-ui` passes on macOS
- [ ] 2.2 Read `accessibilityDisplayShouldReduceMotion` in one function, `false` on other platforms; unit-test the mapping from the raw value
- [ ] 2.3 Apply it with `set_reduce_motion` at startup and on each application activation, and verify with a test that a changed value is applied on the next activation
- [ ] 2.4 Document the behavior in `docs/site/` (one short section: transitions follow the OS setting) and verify the page builds

## 3. Visible-set revision

- [ ] 3.1 Hash the ordered item ids in the pass that builds the visible set in `Library::invalidate_cache`, store it with the cache, and verify the render guard test still passes
- [ ] 3.2 Expose `visible_revision()` on the controller, advancing only when the hash differs, with tests: identical ids keep the revision, a reorder and a removal change it, a thumbnail or availability update keeps it

## 4. Region fade

- [ ] 4.1 Track the last drawn revision, presentation, grouping, and content state kind in `CatalogView`; start a transition when any differs and none is running, and verify with a test that a second change during a running transition does not restart it
- [ ] 4.2 Wrap the List, grouped List, Thumbs, Grid, and grouped Thumbs/Grid content containers with `fade_in`, and verify scroll offset is unchanged across a transition that keeps the same items
- [ ] 4.3 Fade content in when it replaces the skeleton or an empty state, and verify with a test that the state kind change starts one transition
- [ ] 4.4 Verify the grid width reflow (inspector or filters opening, divider drag) starts no transition

## 5. Cover fade-in

- [ ] 5.1 Add the `fresh` set to `CoverCache`, filled when an image is inserted for an id that had none, and cleared by a one-shot timer after the fade; test that the id leaves the set
- [ ] 5.2 Render a fresh cover through `fade_in` in the Grid and Thumbs cards and the List thumbnail cell, and verify a cached cover scrolled into view starts no animation

## 6. Verification and wrap-up

- [ ] 6.1 Add a `tracing` span around `render_catalog_view`, measure it over a 10,000-item library for a filter change, a presentation switch, and a cover burst with transitions on and off, and record the numbers against the budget in design.md in the PR
- [ ] 6.2 Run `make fmt`, `make size-check`, clippy with `-D warnings`, and `cargo test --workspace`
- [ ] 6.3 Set ADR 0011 to Accepted and update the README index row when the work merges

## Workflow follow-up

- Land after, or rebase onto, `simplify-catalog-status-indicators`, which edits the same card and row rendering.
- Archive the change after the PR merges.
