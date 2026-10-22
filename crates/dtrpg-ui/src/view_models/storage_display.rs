//! What the Storage settings page says about locations and placement rules,
//! as plain text. No I/O and no window, so it can be tested directly.

use std::collections::HashMap;
use std::sync::Arc;

use dtrpg_core::data::storage::placement::{PlacementCondition, PlacementRule};
use dtrpg_core::data::storage::roots::StorageRoot;
use rust_i18n::t;

/// Everything the Storage page shows about locations beyond their names and
/// paths, gathered once per frame from the controllers.
#[derive(Clone, Debug, Default)]
pub struct StorageDisplay {
    /// The location downloads go to when no rule matches.
    pub default_root_id:  u32,
    /// Free and total space in megabytes, by location id, when readable.
    pub volumes:          HashMap<u32, (Option<f64>, Option<f64>)>,
    /// Downloaded titles in each location, by location id.
    pub title_counts:     HashMap<u32, usize>,
    /// Collection names by id, for describing rules.
    pub collection_names: HashMap<u64, Arc<str>>,
}

/// How a placement rule reads in the list: a title such as
/// "Paizo → Archive Vault" and a line under it such as "Publisher rule".
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDescription {
    /// What the rule does, as "what → where".
    pub title:    String,
    /// What kind of rule it is.
    pub category: String,
}

/// Describes `rule` using the names of the locations and collections, so the
/// list says "Archive Vault" and not "root 1".
///
/// A location that has been removed, or a collection that no longer exists, is
/// named as such, since a rule can outlive them.
#[must_use]
pub fn describe_rule(rule: &PlacementRule, roots: &[StorageRoot],
                     collection_names: &HashMap<u64, Arc<str>>)
                     -> RuleDescription {
    let location = roots.iter()
                        .find(|root| root.id == rule.target_root_id)
                        .map_or_else(|| t!("settings.storage_rule_removed_location").to_string(),
                                     |root| root.name.clone());
    let (what, category) = match &rule.condition {
        PlacementCondition::Publisher(name) => {
            (name.clone(), t!("settings.storage_rule_kind_publisher"))
        }
        PlacementCondition::Collection(id) => {
            let name = collection_names.get(id)
                                       .map_or_else(|| t!("settings.storage_rule_removed_collection").to_string(),
                                                    |name| format!("\u{201c}{name}\u{201d}"));
            (name, t!("settings.storage_rule_kind_collection"))
        }
        PlacementCondition::SizeAtLeastMb(mb) => (t!("settings.storage_rule_larger_than",
                                                     size = volume_size_label(*mb)).to_string(),
                                                  t!("settings.storage_rule_kind_size")),
    };
    RuleDescription { title:    t!("settings.storage_rule_title",
                                   what = what,
                                   location = location).to_string(),
                      category: category.to_string(), }
}

/// A size in megabytes as the page writes it: megabytes below a gigabyte,
/// then gigabytes, then terabytes, with one decimal place, such as `3.9 GB`.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::storage_display::volume_size_label;
///
/// assert!(volume_size_label(512.0).starts_with("512.0"));
/// assert!(volume_size_label(4096.0).starts_with("4.0"));
/// ```
#[must_use]
pub fn volume_size_label(mb: f64) -> String {
    const MB_PER_GB: f64 = 1024.0;
    const MB_PER_TB: f64 = 1024.0 * 1024.0;
    if mb >= MB_PER_TB {
        format!("{:.1} {}", mb / MB_PER_TB, t!("size.tb"))
    }
    else if mb >= MB_PER_GB {
        format!("{:.1} {}", mb / MB_PER_GB, t!("size.gb"))
    }
    else {
        format!("{mb:.1} {}", t!("size.mb"))
    }
}

/// The line that says how full a location's volume is, such as
/// "582.0 MB free of 994.0 MB", or `None` if the volume could not be read.
#[must_use]
pub fn free_space_label(free_mb: Option<f64>, total_mb: Option<f64>) -> Option<String> {
    let (free, total) = (free_mb?, total_mb?);
    Some(t!("settings.storage_free_of",
            free = volume_size_label(free),
            total = volume_size_label(total)).to_string())
}

/// The count under a location's name, such as "6 titles here".
#[must_use]
pub fn titles_here_label(count: usize) -> String {
    crate::util::pluralize::pluralize(count,
                                      "settings.storage_title_here",
                                      "settings.storage_titles_here")
}

#[cfg(test)]
mod tests;
