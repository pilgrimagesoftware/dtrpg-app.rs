//! The filters DriveThruRPG attaches to a product: its rule system, product
//! type, format, genre and so on.
//!
//! A filter belongs to a category (the name of the root of its tree, such as
//! `Rule System`) and can sit under another filter of the same category
//! (`Pathfinder OGL 1E` under `Pathfinder OGL / ORC` under
//! `Pathfinder / Starfinder`). A product lists every level it belongs to.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// The category of the filters that name a product's game system.
pub const CATEGORY_RULE_SYSTEM: &str = "Rule System";
/// The category of the filters that name a product's type.
pub const CATEGORY_PRODUCT_TYPE: &str = "Product Type";
/// The category of the filters that name a product's format.
pub const CATEGORY_FORMAT: &str = "Format";
/// The category of the filters that name a product's genre.
pub const CATEGORY_GENRE: &str = "Genre";

/// One filter a product has.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProductFilter {
    /// DriveThruRPG's id for the filter. It stays the same when the name
    /// changes.
    pub id:        u64,
    /// The id of the filter this one sits under, or the category's root id.
    pub parent_id: u64,
    /// The category the filter belongs to, such as `Rule System`.
    pub category:  Arc<str>,
    /// The filter's name, such as `Pathfinder OGL 1E`.
    pub name:      Arc<str>,
}

impl ProductFilter {
    /// Whether the filter is in `category`, ignoring case.
    #[must_use]
    pub fn is_in(&self, category: &str) -> bool {
        self.category.eq_ignore_ascii_case(category)
    }
}

/// The names of `filters` in `category`, each once, in the order the product
/// lists them.
///
/// # Examples
///
/// ```
/// use dtrpg_core::data::product_filter::{ProductFilter, names_in};
///
/// let filter = |id, parent_id, name: &str| ProductFilter { id,
///                                                          parent_id,
///                                                          category: "Rule System".into(),
///                                                          name: name.into() };
/// let filters = [filter(1, 0, "Pathfinder / Starfinder"),
///                filter(2, 1, "Pathfinder OGL 1E")];
/// assert_eq!(names_in(&filters, "Rule System"),
///            ["Pathfinder / Starfinder", "Pathfinder OGL 1E"]);
/// ```
#[must_use]
pub fn names_in<'filters>(filters: &'filters [ProductFilter], category: &str)
                          -> Vec<&'filters str> {
    let mut names: Vec<&str> = Vec::new();
    for filter in filters.iter().filter(|filter| filter.is_in(category)) {
        if !names.contains(&filter.name.as_ref()) {
            names.push(&filter.name);
        }
    }
    names
}

/// The most specific filter in `category`: the first one that no other filter
/// of the category sits under. That is `Pathfinder OGL 1E`, not
/// `Pathfinder / Starfinder`. `None` if the product has none in the category.
///
/// # Examples
///
/// ```
/// use dtrpg_core::data::product_filter::{ProductFilter, most_specific};
///
/// let filter = |id, parent_id, name: &str| ProductFilter { id,
///                                                          parent_id,
///                                                          category: "Rule System".into(),
///                                                          name: name.into() };
/// let filters = [filter(1, 0, "Pathfinder / Starfinder"),
///                filter(2, 1, "Pathfinder OGL 1E")];
/// let found = most_specific(&filters, "Rule System").expect("a filter");
/// assert_eq!(found.name.as_ref(), "Pathfinder OGL 1E");
/// ```
#[must_use]
pub fn most_specific<'filters>(filters: &'filters [ProductFilter], category: &str)
                               -> Option<&'filters ProductFilter> {
    let in_category = || filters.iter().filter(|filter| filter.is_in(category));
    in_category().find(|candidate| in_category().all(|other| other.parent_id != candidate.id))
}

#[cfg(test)]
mod tests;
