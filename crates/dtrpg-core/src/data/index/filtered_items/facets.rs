//! `facet_counts`: how many items of the current result have each value of a
//! facet field, for the filter column's counts.

use rusqlite::{Connection, params_from_iter};

use super::clause::{ItemFilter, build_where};
use crate::data::index::IndexError;
use crate::util::search::Field;

/// One value of a facet and how many items have it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FacetCount {
    /// The value as stored, in its own case. When values differ only in case,
    /// this is one of them.
    pub value: String,
    /// How many items have it.
    pub count: usize,
}

/// Counts the items matching `filter` by each value of `field`, the most
/// common value first, then by name.
///
/// Only the fields a facet can show are counted; any other gives no values.
/// Items with nothing in the field (no game system, say) are not counted
/// under any value. Tags count each item once per tag.
///
/// # Errors
///
/// Returns [`IndexError::Sqlite`] if the query fails.
pub(in crate::data::index) fn facet_counts(conn: &Connection, filter: &ItemFilter<'_>,
                                           field: Field)
                                           -> Result<Vec<FacetCount>, IndexError> {
    let (where_clause, mut args) = build_where(filter);
    let sql = match field {
        Field::Availability => {
            format!("SELECT status, COUNT(*) FROM items {where_clause} \
                     GROUP BY status ORDER BY COUNT(*) DESC, status")
        }
        Field::System | Field::Type | Field::Format | Field::Genre => {
            let category = field.category().unwrap_or_default().to_lowercase();
            let n = args.len() + 1;
            args.push(Box::new(category));
            format!("SELECT MIN(f.name), COUNT(DISTINCT f.item_id) \
                     FROM item_filters f \
                     WHERE ulower(f.category) = ?{n} \
                     AND f.item_id IN (SELECT id FROM items {where_clause}) \
                     GROUP BY ulower(f.name) \
                     ORDER BY COUNT(DISTINCT f.item_id) DESC, ulower(MIN(f.name))")
        }
        Field::Tag => {
            format!("SELECT MIN(tg.name), COUNT(DISTINCT it.item_id) \
                     FROM item_tags it JOIN tags tg ON tg.id = it.tag_id \
                     WHERE it.item_id IN (SELECT id FROM items {where_clause}) \
                     GROUP BY ulower(tg.name) \
                     ORDER BY COUNT(DISTINCT it.item_id) DESC, ulower(MIN(tg.name))")
        }
        Field::Location => {
            format!("SELECT MIN(fl.location), COUNT(DISTINCT fl.item_id) \
                     FROM file_locations fl \
                     WHERE fl.item_id IN (SELECT id FROM items {where_clause}) \
                     GROUP BY ulower(fl.location) \
                     ORDER BY COUNT(DISTINCT fl.item_id) DESC, ulower(MIN(fl.location))")
        }
        Field::Publisher | Field::Collection => return Ok(Vec::new()),
    };
    let mut stmt = conn.prepare(&sql)?;
    let counts = stmt.query_map(params_from_iter(args.iter().map(AsRef::as_ref)), |row| {
                         let count: i64 = row.get(1)?;
                         Ok(FacetCount { value: row.get(0)?,
                                         count: usize::try_from(count).unwrap_or_default(), })
                     })?
                     .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(counts)
}
