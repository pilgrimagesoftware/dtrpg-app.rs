//! Tests for the match tracker.

use std::collections::HashSet;
use std::sync::Arc;

use super::MatchTracker;

fn ids(names: &[&str]) -> HashSet<Arc<str>> {
    names.iter().copied().map(Arc::from).collect()
}

#[test]
fn the_first_look_reports_nothing() {
    let mut tracker = MatchTracker::default();

    assert!(tracker.observe(1, ids(&["a", "b"])).is_empty());
}

#[test]
fn items_that_start_to_match_are_reported_sorted() {
    let mut tracker = MatchTracker::default();
    tracker.observe(1, ids(&["a"]));

    let started = tracker.observe(1, ids(&["a", "z", "c"]));

    assert_eq!(started, vec![Arc::<str>::from("c"), Arc::from("z")]);
}

#[test]
fn an_item_is_reported_once() {
    let mut tracker = MatchTracker::default();
    tracker.observe(1, ids(&[]));
    tracker.observe(1, ids(&["a"]));

    assert!(tracker.observe(1, ids(&["a"])).is_empty());
}

#[test]
fn an_item_that_leaves_and_returns_is_reported_again() {
    let mut tracker = MatchTracker::default();
    tracker.observe(1, ids(&["a"]));
    tracker.observe(1, ids(&[]));

    assert_eq!(tracker.observe(1, ids(&["a"])), vec![Arc::<str>::from("a")]);
}

#[test]
fn collections_are_tracked_separately() {
    let mut tracker = MatchTracker::default();
    tracker.observe(1, ids(&[]));

    assert!(tracker.observe(2, ids(&["a"])).is_empty());
}

#[test]
fn a_forgotten_collection_gets_a_first_look_again() {
    let mut tracker = MatchTracker::default();
    tracker.observe(1, ids(&[]));
    tracker.forget(1);

    assert!(tracker.observe(1, ids(&["a"])).is_empty());
}
