//! Count-noun formatting with correct singular/plural selection.
//!
//! All UI count strings MUST route through [`pluralize`] so that a future
//! i18n layer has a single replacement point.

/// Formats `count` with the correct noun form.
///
/// Returns `"{count} {singular}"` when `count == 1`, otherwise `"{count} {plural}"`.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::util::pluralize::pluralize;
///
/// assert_eq!(pluralize(1, "item", "items"), "1 item");
/// assert_eq!(pluralize(0, "item", "items"), "0 items");
/// assert_eq!(pluralize(42, "title", "titles"), "42 titles");
/// ```
#[must_use]
pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_uses_plural() {
        assert_eq!(pluralize(0, "item", "items"), "0 items");
    }

    #[test]
    fn one_uses_singular() {
        assert_eq!(pluralize(1, "item", "items"), "1 item");
    }

    #[test]
    fn many_uses_plural() {
        assert_eq!(pluralize(42, "title", "titles"), "42 titles");
    }

    #[test]
    fn irregular_plural_caller_supplied() {
        assert_eq!(pluralize(1, "library", "libraries"), "1 library");
        assert_eq!(pluralize(3, "library", "libraries"), "3 libraries");
    }
}
