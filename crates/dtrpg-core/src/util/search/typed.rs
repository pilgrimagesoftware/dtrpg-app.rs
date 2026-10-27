//! What typing in the search box does to a search: text stays text until the
//! user commits it, and a committed `field:value` term becomes a chip.

use super::{SearchQuery, Term};

/// The result of applying the text in the search box to a search whose chips
/// are `chips`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypedSearch {
    /// The user is still typing: the search is `query`, but the chips and the
    /// text in the box are as they were.
    Typing {
        /// The search as text: the chips' terms followed by what was typed.
        query: String,
    },
    /// The user finished one or more `field:value` terms: they are chips now.
    Committed {
        /// The chips, with the new terms after the old.
        chips: Vec<Term>,
        /// What is left to show in the box once the terms have left it.
        text:  String,
    },
}

/// Applies `typed`, the text in the search box, to a search whose chips are
/// `chips`.
///
/// A term becomes a chip only when committed: `commit` is true (Enter was
/// pressed) or the text ends in whitespace. A term still being typed (`tag:so`)
/// is part of the search text, never a chip.
///
/// # Examples
///
/// ```
/// use dtrpg_core::util::search::{TypedSearch, apply_typed};
///
/// assert_eq!(apply_typed(&[], "tag:so", false),
///            TypedSearch::Typing { query: "tag:so".into(), });
/// let TypedSearch::Committed { chips, text } = apply_typed(&[], "tag:solo ", false)
/// else {
///     panic!("a finished term commits");
/// };
/// assert_eq!(chips.len(), 1);
/// assert_eq!(text, "");
/// ```
#[must_use]
pub fn apply_typed(chips: &[Term], typed: &str, commit: bool) -> TypedSearch {
    let parsed = SearchQuery::parse(typed);
    let finished = commit || typed.ends_with(char::is_whitespace);
    if finished && !parsed.terms().is_empty() {
        let mut all = chips.to_vec();
        all.extend(parsed.terms().iter().cloned());
        return TypedSearch::Committed { chips: all,
                                        text:  parsed.free_text().to_owned(), };
    }
    TypedSearch::Typing { query: join(chips, typed), }
}

/// The search made of `chips` and the text `typed`, as text.
#[must_use]
pub fn join(chips: &[Term], typed: &str) -> String {
    let mut parts: Vec<String> = chips.iter().map(ToString::to_string).collect();
    if !typed.trim().is_empty() {
        parts.push(typed.to_owned());
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(chips: &mut Vec<Term>, typed: &mut String, ch: char) {
        typed.push(ch);
        if let TypedSearch::Committed { chips: next, text } = apply_typed(chips, typed, false) {
            *chips = next;
            *typed = text;
        }
    }

    #[test]
    fn typing_a_term_slowly_makes_no_chip_until_it_is_committed() {
        let (mut chips, mut typed) = (Vec::new(), String::new());
        for ch in "tag:solo".chars() {
            feed(&mut chips, &mut typed, ch);
            assert!(chips.is_empty(), "a chip appeared after {typed:?}");
        }
        feed(&mut chips, &mut typed, ' ');
        assert_eq!(chips.len(), 1);
        assert_eq!(chips[0].to_string(), "tag:solo");
        assert_eq!(typed, "");
    }

    #[test]
    fn enter_commits_a_term_without_a_trailing_space() {
        let outcome = apply_typed(&[], "tag:solo", true);
        assert!(matches!(outcome, TypedSearch::Committed { ref chips, ref text }
                         if chips.len() == 1 && text.is_empty()));
    }

    #[test]
    fn free_text_alongside_a_committed_term_stays_in_the_box() {
        let outcome = apply_typed(&[], "tag:solo moria ", false);
        assert!(matches!(outcome, TypedSearch::Committed { ref text, .. } if text == "moria"));
    }

    #[test]
    fn typing_is_searched_with_the_chips_in_front() {
        let chips = SearchQuery::parse("type:adventure").terms().to_vec();
        assert_eq!(apply_typed(&chips, "tag:so", false),
                   TypedSearch::Typing { query: "type:adventure tag:so".into(), });
    }
}
