#![allow(clippy::unwrap_used)]

//! Tests for the Storage page's text.

use std::path::PathBuf;

use super::*;

fn roots() -> Vec<StorageRoot> {
    vec![StorageRoot { id:   0,
                       name: "Local Library".to_owned(),
                       path: PathBuf::from("/a"), },
         StorageRoot { id:   1,
                       name: "Archive Vault".to_owned(),
                       path: PathBuf::from("/b"), }]
}

fn rule(condition: PlacementCondition, target_root_id: u32) -> PlacementRule {
    PlacementRule { condition,
                    target_root_id }
}

fn assert_resolved(text: &str) {
    assert!(!text.starts_with("settings."),
            "{text} is a missing text key");
}

#[test]
fn a_publisher_rule_names_the_publisher_and_the_location() {
    let description = describe_rule(&rule(PlacementCondition::Publisher("Paizo".to_owned()), 1),
                                    &roots(),
                                    &HashMap::new());

    assert_resolved(&description.title);
    assert!(description.title.contains("Paizo"));
    assert!(description.title.contains("Archive Vault"));
    assert!(!description.title.contains("root"));
}

#[test]
fn a_collection_rule_names_the_collection_or_says_it_is_gone() {
    let names = HashMap::from([(7, Arc::<str>::from("Thursday Game"))]);

    let known = describe_rule(&rule(PlacementCondition::Collection(7), 0),
                              &roots(),
                              &names);
    let gone = describe_rule(&rule(PlacementCondition::Collection(8), 0),
                             &roots(),
                             &names);

    assert!(known.title.contains("Thursday Game"));
    assert!(!gone.title.contains('8'));
    assert_resolved(&gone.title);
}

#[test]
fn a_size_rule_says_how_large_and_a_removed_location_is_named_as_removed() {
    let description = describe_rule(&rule(PlacementCondition::SizeAtLeastMb(250.0), 9),
                                    &roots(),
                                    &HashMap::new());

    assert!(description.title.contains("250.0"));
    assert!(!description.title.contains("Archive"));
    assert_resolved(&description.title);
    assert_resolved(&description.category);
}

#[test]
fn sizes_move_up_a_unit_at_a_gigabyte_and_at_a_terabyte() {
    assert!(volume_size_label(512.0).starts_with("512.0 "));
    assert!(volume_size_label(1024.0).starts_with("1.0 "));
    assert!(volume_size_label(4096.0).starts_with("4.0 "));
    assert!(volume_size_label(2.0 * 1024.0 * 1024.0).starts_with("2.0 "));
    assert_ne!(volume_size_label(512.0), volume_size_label(1024.0));
}

#[test]
fn the_free_space_line_needs_both_numbers() {
    assert!(free_space_label(Some(582.0), Some(994.0)).is_some());
    assert_eq!(free_space_label(None, Some(994.0)), None);
    assert_eq!(free_space_label(Some(582.0), None), None);
}

#[test]
fn the_titles_here_count_resolves_for_one_and_many() {
    assert_resolved(&titles_here_label(1));
    assert_resolved(&titles_here_label(6));
    assert!(titles_here_label(6).contains('6'));
}
