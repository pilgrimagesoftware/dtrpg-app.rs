//! The filter dimensions (`ItemFilter`) and the shared WHERE-clause builder
//! used by both `query::list_items_filtered` and `query::count_items_filtered`.
//!
//! The search-box dimension reproduces `crate::util::search::SearchQuery`
//! exactly — see `openspec/changes/search-box-semantics-sql/design.md` for
//! why `instr(ulower(col), ?)` and not `LIKE`, FTS5 `MATCH`, or trigrams.

use std::collections::HashSet;

use rusqlite::ToSql;

use crate::data::enums::ItemStatus;
use crate::data::index::items::status_to_text;
use crate::util::search::{Field, SearchQuery, Term};

/// A sidebar-filter/search dimension to apply, beyond what's always true
/// (`SidebarFilter::AllTitles`). At most one *sidebar* dimension is ever set
/// by the current UI (see `catalog-reads-from-index`'s design notes), but
/// each is independent of the others here rather than folded into one enum,
/// since a future caller combining them should not need a new query
/// function. `search` is orthogonal: it composes with whichever sidebar
/// dimension is active.
#[derive(Debug, Default)]
pub struct ItemFilter<'a> {
    pub status:                Option<ItemStatus>,
    pub publisher:             Option<&'a str>,
    /// Member ids to match an item's `order_product_id` or `product_id`
    /// against, resolved by the caller (the controller's own
    /// `collection_members`, kept current by optimistic edits) rather than
    /// joined against the index's own `collection_membership` table — see
    /// design.md's risk note on why those two can briefly disagree.
    pub collection_member_ids: Option<&'a HashSet<u64>>,
    /// `(now_secs, window_days)`, matching
    /// [`super::items::count_recently_updated`]'s window semantics exactly.
    pub recently_updated:      Option<(i64, u32)>,
    /// The `Tag` sidebar filter, by tag name, matched case-insensitively
    /// against the index's `tags`/`item_tags` tables — the SQL equivalent of
    /// the old post-query pass over `TagLibrary::items_with(name)`'s id set.
    pub tag_name:              Option<&'a str>,
    /// The parsed search-box query, exactly as `SearchQuery::parse` produced
    /// it: free text substring-matched over title/publisher/line/tag names, and
    /// `field:value` terms required (`-field:value` forbidden). Parsing itself
    /// never moves into SQL.
    pub search:                Option<&'a SearchQuery>,
}

/// One item's `EXISTS`/`NOT EXISTS` over a tag name, bound at parameter `n`
/// (which holds the already-lower-cased name).
fn tag_exists_clause(negated: bool, n: usize) -> String {
    let exists = "EXISTS (SELECT 1 FROM item_tags it \
                  JOIN tags tg ON tg.id = it.tag_id \
                  WHERE it.item_id = items.id AND ulower(tg.name) = ?";
    let prefix = if negated { "NOT " } else { "" };
    format!("{prefix}{exists}{n})")
}

/// One item's `EXISTS`/`NOT EXISTS` over a product filter, bound at parameters
/// `category_n` (the lower-cased category) and `name_n` (the lower-cased name).
fn filter_exists_clause(negated: bool, category_n: usize, name_n: usize) -> String {
    let exists = "EXISTS (SELECT 1 FROM item_filters f \
                  WHERE f.item_id = items.id AND ulower(f.category) = ?";
    let prefix = if negated { "NOT " } else { "" };
    format!("{prefix}{exists}{category_n} AND ulower(f.name) = ?{name_n})")
}

/// The clause for one search term, or `None` when the term keeps every item.
/// Pushes the term's argument, so the clause's `?n` matches its position.
///
/// Values are lower cased already, and `ulower` folds the columns the same
/// way. An availability value that names no state holds for no item, so a
/// positive term keeps none and a negated one keeps all.
fn term_clause(term: &Term, args: &mut Vec<Box<dyn ToSql>>) -> Option<String> {
    let column = match term.field {
        Field::Tag => {
            let n = args.len() + 1;
            args.push(Box::new(term.value.clone()));
            return Some(tag_exists_clause(term.negated, n));
        }
        Field::Availability => {
            let Some(status) = term.availability()
            else {
                return (!term.negated).then(|| "1 = 0".to_owned());
            };
            let n = args.len() + 1;
            args.push(Box::new(status_to_text(status)));
            let operator = if term.negated { "<>" } else { "=" };
            return Some(format!("status {operator} ?{n}"));
        }
        Field::Publisher => "publisher",
        Field::System | Field::Type | Field::Format | Field::Genre => {
            let category = term.field.category().unwrap_or_default().to_lowercase();
            let n = args.len() + 1;
            args.push(Box::new(category));
            args.push(Box::new(term.value.clone()));
            return Some(filter_exists_clause(term.negated, n, n + 1));
        }
    };
    let n = args.len() + 1;
    args.push(Box::new(term.value.clone()));
    let operator = if term.negated { "<>" } else { "=" };
    Some(format!("ulower({column}) {operator} ?{n}"))
}

/// Builds the `WHERE` clause (empty string when nothing applies) and its
/// positional arguments, shared verbatim by the list and count queries so
/// the two can never drift apart.
///
/// Invariant: every `?n` is computed from `args.len()` at the point its
/// argument is pushed — clause text order must match argument push order, and
/// no clause may be emitted without pushing exactly the parameters it
/// references.
pub(super) fn build_where(filter: &ItemFilter<'_>) -> (String, Vec<Box<dyn ToSql>>) {
    let mut clauses: Vec<String> = Vec::new();
    let mut args: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(status) = filter.status {
        clauses.push(format!("status = ?{}", args.len() + 1));
        args.push(Box::new(status_to_text(status)));
    }
    if let Some(publisher) = filter.publisher {
        clauses.push(format!("publisher = ?{}", args.len() + 1));
        args.push(Box::new(publisher.to_owned()));
    }
    if let Some((now_secs, window_days)) = filter.recently_updated {
        let window_secs = i64::from(window_days) * 24 * 60 * 60;
        clauses.push(format!("COALESCE(date_updated, date_added) >= ?{} - ?{}",
                             args.len() + 1,
                             args.len() + 2));
        args.push(Box::new(now_secs));
        args.push(Box::new(window_secs));
    }
    if let Some(member_ids) = filter.collection_member_ids {
        if member_ids.is_empty() {
            // A collection section with no members matches nothing. An empty
            // `IN ()` list is a SQLite syntax error whose fallout would
            // differ between the list and count queries, so the empty case
            // is stated directly instead.
            clauses.push("1 = 0".to_owned());
        }
        else {
            // Collected once so the two `IN` lists and the pushed arguments
            // iterate the same order; both lists reference the SAME numbered
            // placeholders (SQLite allows re-referencing ?n), so the ids are
            // bound once, not twice.
            let ids: Vec<i64> = member_ids.iter().map(|id| *id as i64).collect();
            let placeholders = ids.iter()
                                  .enumerate()
                                  .map(|(i, _)| format!("?{}", args.len() + 1 + i))
                                  .collect::<Vec<_>>()
                                  .join(", ");
            clauses.push(format!(
                "(order_product_id IN ({placeholders}) OR product_id IN ({placeholders}))"
            ));
            args.extend(ids.into_iter().map(|id| Box::new(id) as Box<dyn ToSql>));
        }
    }
    if let Some(name) = filter.tag_name {
        let n = args.len() + 1;
        args.push(Box::new(name.to_lowercase()));
        clauses.push(tag_exists_clause(false, n));
    }
    if let Some(query) = filter.search {
        // `SearchQuery` stores its text and tag names already lower-cased,
        // and `ulower` applies the same Rust fold to the columns, so the
        // comparison needs no further normalization on either side.
        let text = query.free_text();
        if !text.is_empty() {
            let n = args.len() + 1;
            args.push(Box::new(text.to_owned()));
            clauses.push(format!(
                "(instr(ulower(title), ?{n}) > 0 OR instr(ulower(publisher), ?{n}) > 0 OR \
                 instr(ulower(line), ?{n}) > 0 OR \
                 EXISTS (SELECT 1 FROM item_tags it JOIN tags tg ON tg.id = it.tag_id \
                         WHERE it.item_id = items.id AND instr(ulower(tg.name), ?{n}) > 0))"
            ));
        }
        // A `-` term and a tag term each have to hold. For every other field,
        // its terms are alternatives, so they join with OR in one clause.
        for term in query.terms()
                         .iter()
                         .filter(|term| term.negated || term.field == Field::Tag)
        {
            clauses.extend(term_clause(term, &mut args));
        }
        for field in Field::ALL.into_iter().filter(|field| *field != Field::Tag) {
            let alternatives: Vec<String> =
                query.terms()
                     .iter()
                     .filter(|term| !term.negated && term.field == field)
                     .filter_map(|term| term_clause(term, &mut args))
                     .collect();
            if !alternatives.is_empty() {
                clauses.push(format!("({})", alternatives.join(" OR ")));
            }
        }
    }

    let where_clause = if clauses.is_empty() {
        String::new()
    }
    else {
        format!("WHERE {}", clauses.join(" AND "))
    };
    (where_clause, args)
}
