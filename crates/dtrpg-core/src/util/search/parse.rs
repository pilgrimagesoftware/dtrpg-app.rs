//! Reading a search typed in the search box.

use super::{Field, SearchQuery, Term};

/// Removes `prefix` from the start of `text`, ignoring ASCII case.
fn strip_prefix_ignoring_case<'text>(text: &'text str, prefix: &str) -> Option<&'text str> {
    let head = text.get(..prefix.len())?;
    head.eq_ignore_ascii_case(prefix)
        .then(|| &text[prefix.len()..])
}

/// Reads a value from the start of `text`: a quoted value up to the closing
/// quote (or the end), or up to the next space. Returns the value and the
/// rest.
fn read_value(text: &str) -> (&str, &str) {
    if let Some(quoted) = text.strip_prefix('"') {
        return match quoted.split_once('"') {
            Some((value, rest)) => (value, rest),
            None => (quoted, ""),
        };
    }
    text.split_once(char::is_whitespace).unwrap_or((text, ""))
}

/// Reads the term at the start of `text`, if one starts there. The term is
/// `None` when it names nothing (`tag:` with no value), so it asks for
/// nothing. Returns what was read as `Some((term, rest))`.
fn read_term(text: &str) -> Option<(Option<Term>, &str)> {
    let (negated, unsigned) = match text.strip_prefix('-') {
        Some(after) => (true, after),
        None => (false, text),
    };
    let (field, after_keyword) =
        Field::ALL.into_iter().find_map(|field| {
                                   let keyword = format!("{}:", field.keyword());
                                   strip_prefix_ignoring_case(unsigned, &keyword).map(|after| {
                                                                                     (field, after)
                                                                                 })
                               })?;
    let (value, rest) = read_value(after_keyword);
    let value = value.trim().to_lowercase();
    let term = (!value.is_empty()).then_some(Term { field,
                                                    value,
                                                    negated });
    Some((term, rest.trim_start()))
}

impl SearchQuery {
    /// Splits what the user typed into free text and terms.
    ///
    /// The stored text is always lower cased. A query with no term keeps the
    /// typed text's spacing verbatim (untrimmed, whole-query), so a plain
    /// search behaves as it always has; with a term the leftover words rejoin
    /// with single spaces. A term with no value asks for nothing, and is left
    /// out of the text. A word before a colon that is not a field, such as
    /// `colour:cloud`, is free text.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::SearchQuery;
    ///
    /// let query = SearchQuery::parse("moria type:adventure");
    /// assert_eq!(query.to_string(), "type:adventure moria");
    /// ```
    #[must_use]
    pub fn parse(query: &str) -> Self {
        let mut terms = Vec::new();
        let mut words: Vec<&str> = Vec::new();
        let mut saw_a_term = false;
        let mut rest = query.trim_start();
        while !rest.is_empty() {
            if let Some((term, remainder)) = read_term(rest) {
                saw_a_term = true;
                terms.extend(term);
                rest = remainder;
            }
            else {
                let (word, remainder) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
                words.push(word);
                rest = remainder.trim_start();
            }
        }
        let text = if saw_a_term {
            words.join(" ").to_lowercase()
        }
        else {
            query.to_lowercase()
        };
        Self { text, terms }
    }
}
