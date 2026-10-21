#![allow(clippy::unwrap_used)]

//! Tests for reading a product's filters.

use super::*;

fn filter(id: u64, parent_id: u64, category: &str, name: &str) -> ProductFilter {
    ProductFilter { id,
                    parent_id,
                    category: category.into(),
                    name: name.into() }
}

#[test]
fn the_most_specific_filter_is_the_one_nothing_sits_under() {
    let filters = [filter(10, 0, "Rule System", "Pathfinder / Starfinder"),
                   filter(20, 10, "Rule System", "Pathfinder OGL / ORC"),
                   filter(30, 20, "Rule System", "Pathfinder OGL 1E"),
                   filter(40, 0, "Format", "PDF")];

    let found = most_specific(&filters, "Rule System").unwrap();

    assert_eq!(found.name.as_ref(), "Pathfinder OGL 1E");
}

#[test]
fn with_two_branches_the_first_listed_leaf_is_chosen() {
    let filters = [filter(1, 0, "Rule System", "Basic/BECMI or OD&D"),
                   filter(2, 0, "Rule System", "Official D&D")];

    assert_eq!(most_specific(&filters, "Rule System").unwrap()
                                                     .name
                                                     .as_ref(),
               "Basic/BECMI or OD&D");
}

#[test]
fn a_product_with_no_filter_in_the_category_has_no_most_specific_one() {
    let filters = [filter(1, 0, "Format", "PDF")];

    assert!(most_specific(&filters, "Rule System").is_none());
    assert!(most_specific(&[], "Rule System").is_none());
}

#[test]
fn categories_are_compared_ignoring_case() {
    let filters = [filter(1, 0, "Rule System", "OSR")];

    assert_eq!(names_in(&filters, "rule system"), ["OSR"]);
}

#[test]
fn names_are_listed_once_in_the_order_given() {
    let filters = [filter(1, 0, "Genre", "Fantasy"),
                   filter(2, 0, "Genre", "Horror"),
                   filter(3, 0, "Genre", "Fantasy")];

    assert_eq!(names_in(&filters, "Genre"), ["Fantasy", "Horror"]);
}
