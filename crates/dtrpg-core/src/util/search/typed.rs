//! What typing in the search box does to a search: text stays text until the
//! user commits it, and a committed `field:value` term becomes a chip.

use super::{Field, SearchQuery, Term};

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

/// The field whose keyword the word being typed at the end of `typed` is the
/// start of, ignoring case, so `typ` suggests `type`. A word that already has
/// its colon, or is the whole of a keyword, suggests nothing. A leading `-`
/// (a negated term) is not part of the word.
///
/// # Examples
///
/// ```
/// use dtrpg_core::util::search::{Field, suggest_key};
///
/// assert_eq!(suggest_key("moria TYP"), Some(Field::Type));
/// assert_eq!(suggest_key("type:"), None);
/// ```
#[must_use]
pub fn suggest_key(typed: &str) -> Option<Field> {
    let word = typed.rsplit(char::is_whitespace).next().unwrap_or_default();
    let word = word.strip_prefix('-').unwrap_or(word);
    if word.is_empty() || word.contains(':') {
        return None;
    }
    Field::ALL.into_iter().find(|field| {
                                    let keyword = field.keyword();
                                    keyword.len() > word.len()
                                    && keyword.as_bytes()[..word.len()].eq_ignore_ascii_case(word.as_bytes())
                                })
}

/// `typed` with the word being typed at its end completed to `field`'s
/// keyword and a colon, keeping any `-` in front of it.
///
/// # Examples
///
/// ```
/// use dtrpg_core::util::search::{Field, accept_key};
///
/// assert_eq!(accept_key("moria -ta", Field::Tag), "moria -tag:");
/// ```
#[must_use]
pub fn accept_key(typed: &str, field: Field) -> String {
    let start =
        typed.rfind(char::is_whitespace)
             .map_or(0, |ix| {
                 ix + typed[ix..].chars().next().map_or(1, char::len_utf8)
             });
    let word = &typed[start..];
    let sign = if word.starts_with('-') { "-" } else { "" };
    format!("{}{sign}{}:", &typed[..start], field.keyword())
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

    #[test]
    fn a_prefix_suggests_its_key_ignoring_case() {
        assert_eq!(suggest_key("typ"), Some(Field::Type));
        assert_eq!(suggest_key("TYP"), Some(Field::Type));
        assert_eq!(suggest_key("moria -ta"), Some(Field::Tag));
    }

    #[test]
    fn nothing_is_suggested_once_the_key_is_complete_or_unknown() {
        assert_eq!(suggest_key("type"), None);
        assert_eq!(suggest_key("type:adv"), None);
        assert_eq!(suggest_key("zzz"), None);
        assert_eq!(suggest_key("moria "), None);
        assert_eq!(suggest_key(""), None);
    }

    #[test]
    fn accepting_replaces_only_the_last_word() {
        assert_eq!(accept_key("typ", Field::Type), "type:");
        assert_eq!(accept_key("moria typ", Field::Type), "moria type:");
        assert_eq!(accept_key("-pub", Field::Publisher), "-publisher:");
    }

    #[test]
    fn an_accepted_key_is_not_a_chip_until_it_has_a_value_and_is_committed() {
        let accepted = accept_key("typ", Field::Type);
        assert!(matches!(apply_typed(&[], &accepted, false),
                         TypedSearch::Typing { .. }));
    }
}
