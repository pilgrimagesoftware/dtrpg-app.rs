//! The placement-rule list of a [`StorageConfig`]: reading it and editing it.

use super::StorageConfig;
use super::placement::{PlacementCondition, PlacementRule};

impl StorageConfig {
    /// Returns the configured placement rules, in evaluation order.
    #[must_use]
    pub fn rules(&self) -> &[PlacementRule] {
        &self.rules
    }

    /// Appends a new placement rule to the end of the evaluation order.
    pub fn add_rule(&mut self, condition: PlacementCondition, target_root_id: u32) {
        self.rules.push(PlacementRule { condition,
                                        target_root_id });
        self.save();
    }

    /// Sends every title from `publisher` to the root `root_id` from now on.
    ///
    /// The rule goes first in the evaluation order so the user's explicit
    /// choice wins over rules set earlier (a size rule, say), and replaces any
    /// publisher rule `publisher` already had, so a publisher never has two.
    pub fn remember_publisher_root(&mut self, publisher: &str, root_id: u32) {
        self.set_publisher_rule(publisher, root_id);
        self.save();
    }

    /// Makes the change [`Self::remember_publisher_root`] describes, without
    /// saving it.
    pub(in crate::data::storage) fn set_publisher_rule(&mut self, publisher: &str, root_id: u32) {
        self.rules
            .retain(|rule| rule.condition != PlacementCondition::Publisher(publisher.to_owned()));
        self.rules.insert(0,
                          PlacementRule { condition:
                                              PlacementCondition::Publisher(publisher.to_owned()),
                                          target_root_id: root_id, });
    }

    /// Removes the rule at `index`, if one exists there.
    pub fn remove_rule(&mut self, index: usize) {
        if index < self.rules.len() {
            self.rules.remove(index);
            self.save();
        }
    }

    /// Moves the rule at `index` earlier in evaluation order, swapping it
    /// with its predecessor. A no-op if `index` is `0` or out of range.
    pub fn move_rule_up(&mut self, index: usize) {
        if index > 0 && index < self.rules.len() {
            self.rules.swap(index, index - 1);
            self.save();
        }
    }

    /// Moves the rule at `index` later in evaluation order, swapping it
    /// with its successor. A no-op if `index` is the last rule or out of
    /// range.
    pub fn move_rule_down(&mut self, index: usize) {
        if index + 1 < self.rules.len() {
            self.rules.swap(index, index + 1);
            self.save();
        }
    }
}
