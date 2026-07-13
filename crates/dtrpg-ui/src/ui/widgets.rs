//! Small reusable rendering helpers shared across views.

use gpui::prelude::*;
use gpui::{AnyElement, ElementId, SharedString, div, rems};
use gpui_component::text::TextView;

use crate::util::text::escape_markdown;

/// Renders `text` as selectable, copyable plain text via `gpui-component`'s
/// `TextView`.
///
/// The text is Markdown-escaped first — `TextView` parses its input as
/// Markdown, and raw catalog/account data can contain characters (`*`, `_`,
/// `#`, ...) that would otherwise be misread as formatting. `id` must be
/// unique among sibling elements, same as any other `gpui` element id.
pub fn selectable_text(id: impl Into<ElementId>, text: impl Into<SharedString>) -> TextView {
    let escaped = escape_markdown(text.into().as_ref());
    TextView::markdown(id, escaped).selectable(true)
}

/// Rendered size of the small-caps letters, as a fraction of `base_rems` —
/// standard small-caps designs run roughly 70-75% of full cap height.
const SMALL_CAPS_SCALE: f32 = 0.74;

/// Renders `text` in small caps: every letter is drawn as a capital, but
/// letters that were lowercase in the source string are drawn smaller than
/// ones that were already uppercase — the actual distinguishing feature of
/// small caps, as opposed to setting the whole string in uniform ALL CAPS.
///
/// `gpui` has no `font-variant: small-caps`, and the equivalent OpenType
/// `smcp`/`c2sc` features are silently ignored by the many fonts (including
/// most system UI fonts) that don't ship small-cap glyphs, which is why this
/// synthesizes the look directly instead of depending on font support.
///
/// `base_rems` is the size (in `rem`s, same units as `.text_sm()`'s `0.875`)
/// that letters already uppercase in `text` render at; letters that were
/// lowercase render at `base_rems * `[`SMALL_CAPS_SCALE`]. Color, weight, and
/// font family are not set here — apply them to the element wrapping the
/// returned value and let them cascade down to each run, same as any other
/// `gpui` text styling.
pub fn small_caps_text(text: impl Into<SharedString>, base_rems: f32) -> AnyElement {
    let text: SharedString = text.into();
    let small_rems = base_rems * SMALL_CAPS_SCALE;

    let mut row = div().flex().items_baseline();
    for (segment, is_lower) in case_runs(&text) {
        let size = if is_lower { small_rems } else { base_rems };
        let content = if is_lower {
            segment.to_uppercase()
        }
        else {
            segment
        };
        row = row.child(div().text_size(rems(size)).child(content));
    }
    row.into_any_element()
}

/// Splits `text` into consecutive runs of lowercase-alphabetic characters vs.
/// everything else (already-uppercase letters, digits, spaces, punctuation),
/// tagging each run with whether it was a lowercase run — the boundaries
/// [`small_caps_text`] renders at two different sizes.
fn case_runs(text: &str) -> Vec<(String, bool)> {
    let mut runs: Vec<(String, bool)> = Vec::new();
    for ch in text.chars() {
        let is_lower = ch.is_lowercase();
        match runs.last_mut() {
            Some((buf, last_lower)) if *last_lower == is_lower => buf.push(ch),
            _ => runs.push((ch.to_string(), is_lower)),
        }
    }
    runs
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::case_runs;

    #[test]
    fn case_runs_splits_on_case_boundaries() {
        assert_eq!(case_runs("Body Font"),
                   vec![("B".to_string(), false),
                        ("ody".to_string(), true),
                        (" F".to_string(), false),
                        ("ont".to_string(), true)]);
    }

    #[test]
    fn case_runs_treats_digits_and_punctuation_as_non_lowercase() {
        assert_eq!(case_runs("v1.0"),
                   vec![("v".to_string(), true), ("1.0".to_string(), false)]);
    }

    #[test]
    fn case_runs_empty_string_yields_no_runs() {
        assert_eq!(case_runs(""), Vec::<(String, bool)>::new());
    }
}
