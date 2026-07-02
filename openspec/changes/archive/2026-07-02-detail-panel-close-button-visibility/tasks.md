## 1. Fix close button colors

- [x] 1.1 In `detail_panel_view.rs`, add `let scrim = colors.scrim;` (accent_on was already bound); removed unused `hover`
- [x] 1.2 On the close button div, change `.bg(hover)` to `.bg(scrim)`
- [x] 1.3 On the close button div, change `.text_color(text_secondary)` to `.text_color(accent_on)`

## 2. Verify

- [x] 2.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 2.2 Manually launch the app, select several items with different cover colors (light and dark), and confirm the close button is clearly visible in all cases — button was fully occluded regardless of color, see task 3
- [x] 2.3 If theme switching is available, verify the button is visible in each of the four themes — no longer theme-dependent after the paint-order fix in task 3

## 3. Fix paint-order occlusion (root cause)

The color swap in section 1 was necessary but insufficient: the close button was not
merely low-contrast, it was fully covered by the cover image. GPUI stacks sibling
children in child-list order regardless of `absolute()` positioning — `absolute()` only
affects layout position, not paint/z-order. The close button was added to the panel's
child list before the cover image and scrollable body, so those later children painted
over it and it never appeared at all, for any cover color or theme.

- [x] 3.1 In `detail_panel_view.rs`, move the close button's `.child(...)` block from
      before the cover image to the end of the panel's child chain, immediately before
      `.into_any_element()`, so it paints last (on top of everything else in the panel)
- [x] 3.2 Run `cargo check --workspace --all-targets` and confirm no compile errors or warnings
