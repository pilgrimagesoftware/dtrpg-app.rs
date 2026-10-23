//! Smart collections: collections whose members are the library items that
//! match a set of rules, instead of a list chosen by hand.
//!
//! A rule tests one of the fields the search understands (tag, publisher,
//! system, type, availability), so a smart collection is held by the same
//! facts the search box and the filters column use.

mod matching;
mod mirror;
mod model;
mod store;
mod tracker;

pub use mirror::{clear_mirror, smart_collection_dir, sync_mirror};
pub use model::{MatchMode, Operator, Rule, SmartCollection};
pub use store::{SmartCollectionError, SmartCollectionStore};
pub use tracker::MatchTracker;
