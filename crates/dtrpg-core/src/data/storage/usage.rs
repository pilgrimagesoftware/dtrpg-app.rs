//! How many titles each storage root holds.

use std::collections::HashMap;
use std::sync::Arc;

use super::roots::StorageRoot;

/// How many titles each root holds, by root id, from the recorded locations of
/// their files: `recorded` is the names of the locations each title has files
/// in, by item id. A title with files in two locations counts once for each. A
/// root with no titles is not in the map, and a location that is no longer a
/// root (it was removed) counts for nothing.
#[must_use]
pub fn titles_per_root(recorded: &HashMap<Arc<str>, Vec<String>>, roots: &[StorageRoot])
                       -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    for location in recorded.values().flatten() {
        if let Some(root) = roots.iter().find(|root| &root.name == location) {
            *counts.entry(root.id).or_insert(0) += 1;
        }
    }
    counts
}
