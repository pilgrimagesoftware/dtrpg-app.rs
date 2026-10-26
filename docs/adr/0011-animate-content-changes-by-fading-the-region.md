## Status

Proposed

## Context

The catalog's content views flicker and jump when the visible items change, because Thumbs and Grid are index-addressed `uniform_list`s and List is a `DataTable`, neither of which has keyed items or enter, exit, or move hooks. The main window's frame rate is a hard requirement (`dtrpg/.claude/rules/ui-performance.md`): frames render computed values only, and computation runs off the UI thread and only when a change invalidates it. gpui offers `with_animation`, which honors `App::reduce_motion()`.

## Decision

Animate the content region, not the rows: fade the whole content area from partly transparent to opaque for at most 150 ms when the visible set changes, fade covers in on arrival only, and follow the OS reduce-motion setting. A revision counter computed when the visible set is built says when to start a transition.

## Consequences

- Easier: the change is one opacity animation over already-computed layout. It adds no per-frame work when idle, and leaves scroll and virtualization alone.
- Easier: reduce motion works through gpui's own handling once the OS setting is passed to `set_reduce_motion`.
- Harder: individual items do not slide, appear, or disappear; rows still change identity inside the fade. Per-item motion would need ghost rows and position tracking that the list widgets do not support, and would need its own measured budget and a new record.
- Cost: the id hash is computed in the same pass that builds the visible set, once per change.
