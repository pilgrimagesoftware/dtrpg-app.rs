//! Spots the items that start to match a smart collection.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[cfg(test)]
mod tests;

/// Remembers what each smart collection matched the last time it was looked
/// at, so the items that match now and did not then can be told apart.
#[derive(Debug, Default)]
pub struct MatchTracker {
    seen: HashMap<u64, HashSet<Arc<str>>>,
}

impl MatchTracker {
    /// Records `matching` as what the collection with `id` matches now, and
    /// returns the items that were not in the previous record, sorted.
    ///
    /// The first look at a collection only records what it matches and
    /// returns nothing: the items it already held did not just start to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::sync::Arc;
    ///
    /// use dtrpg_core::data::smart_collection::MatchTracker;
    ///
    /// let mut tracker = MatchTracker::default();
    /// let one: HashSet<Arc<str>> = ["a"].into_iter().map(Arc::from).collect();
    /// let two: HashSet<Arc<str>> = ["a", "b"].into_iter().map(Arc::from).collect();
    /// assert!(tracker.observe(1, one).is_empty());
    /// assert_eq!(tracker.observe(1, two), vec![Arc::<str>::from("b")]);
    /// ```
    pub fn observe(&mut self, id: u64, matching: HashSet<Arc<str>>) -> Vec<Arc<str>> {
        let Some(before) = self.seen.insert(id, matching)
        else {
            return Vec::new();
        };
        let mut started: Vec<Arc<str>> = self.seen[&id].difference(&before).cloned().collect();
        started.sort();
        started
    }

    /// Forgets the collection with `id`, so the next look at it is a first
    /// look. Called when its rules change or it is deleted.
    pub fn forget(&mut self, id: u64) {
        self.seen.remove(&id);
    }
}
