## Context

`render_account_section` in `settings_account_view.rs` currently renders:

```
[Account label]
[Signed in to DriveThruRPG]
─── divider ───
[stub notice text]
[Log Out button]
[Reset API Key button]
```

The change restructures the top block into a horizontal row:

```
[Account label + subtitle]        [Reset API Key]
─── divider ───
[stub notice text]
[Log Out button]
```

## Goals / Non-Goals

**Goals:**
- Reset API Key button appears inline to the right of the account info text column.
- Log Out remains as the sole button in the actions area below the divider.
- No behavior or styling changes to the buttons themselves.

**Non-Goals:**
- Making either button functional (blocked on `secure-credential-storage`).
- Any changes outside `settings_account_view.rs`.

## Decisions

### Decision 1: Wrap the identity block in a `flex + justify_between` row

The identity div currently uses `flex_col`. We wrap it in an outer `flex + items_center + justify_between` row that contains the existing `flex_col` label/subtitle on the left and the Reset API Key button on the right.

The button keeps its existing style (`render_action_button`). No new layout primitives are needed.

## Risks / Trade-offs

**[Risk] Button may overflow if the account text is very long** → The identity column gets `flex_1` + `min_w_0` so it shrinks and the button stays at its natural width. Reset API Key is a fixed short label so overflow is unlikely.

## Migration Plan

Single-file edit to `settings_account_view.rs`: restructure the identity row div and remove Reset API Key from the actions section.
