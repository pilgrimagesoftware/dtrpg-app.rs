//! Small text-shaping helpers shared across views.

/// Truncates `text` to at most `max_chars` characters, appending an ellipsis
/// when truncation occurs.
///
/// Counts Unicode scalar values (`char`s) rather than bytes, so multi-byte
/// characters are never split mid-codepoint. `max_chars` is the budget for
/// the kept prefix; the ellipsis is added on top of that budget, matching
/// how truncated labels read in the UI (e.g. a title capped at 40 characters
/// renders as 40 characters plus `…`).
///
/// # Examples
///
/// ```
/// use dtrpg_ui::util::text::truncate_with_ellipsis;
///
/// assert_eq!(truncate_with_ellipsis("Pathfinder", 40), "Pathfinder");
/// assert_eq!(truncate_with_ellipsis("A Very Long Sourcebook Title That Goes On", 10),
///            "A Very Lon\u{2026}");
/// ```
#[must_use]
pub fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut truncated: String = text.chars().take(max_chars).collect();
    truncated.push('\u{2026}');
    truncated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_is_unchanged() {
        assert_eq!(truncate_with_ellipsis("Pathfinder", 40), "Pathfinder");
    }

    #[test]
    fn text_at_the_limit_is_unchanged() {
        assert_eq!(truncate_with_ellipsis("12345", 5), "12345");
    }

    #[test]
    fn long_text_is_truncated_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("123456789", 5), "12345\u{2026}");
    }

    #[test]
    fn truncation_does_not_split_multibyte_characters() {
        // Each emoji is a single `char` but multiple UTF-8 bytes; a byte-based
        // truncation would panic or produce invalid UTF-8 here.
        assert_eq!(truncate_with_ellipsis("😀😀😀😀😀", 2), "😀😀\u{2026}");
    }

    #[test]
    fn empty_text_is_unchanged() {
        assert_eq!(truncate_with_ellipsis("", 10), "");
    }
}
