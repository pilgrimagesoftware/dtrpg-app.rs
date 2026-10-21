//! Tests for the filter value choices.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn the_app_hides_its_noise_list_until_the_user_says_otherwise() {
    let choices = FilterValueChoices::default();

    assert!(!choices.is_shown("Format", "Human-Created Without AI"));
    assert!(!choices.is_shown("format", "CREATION METHOD"));
    assert!(choices.is_shown("Format", "PDF"));
    assert!(choices.is_shown("Genre", "Human-Created Without AI"),
            "the list is per category");
}

#[test]
fn a_user_choice_overrides_the_default_either_way() {
    let mut choices = FilterValueChoices::default();

    choices.set_shown("Format", "Creation Method", true);
    choices.set_shown("Format", "PDF", false);

    assert!(choices.is_shown("Format", "creation method"));
    assert!(!choices.is_shown("Format", "pdf"));
}

#[test]
fn choosing_what_the_app_does_anyway_keeps_nothing() {
    let mut choices = FilterValueChoices::default();

    choices.set_shown("Format", "PDF", false);
    choices.set_shown("Format", "PDF", true);
    choices.set_shown("Format", "Creation Method", true);
    choices.set_shown("Format", "Creation Method", false);

    assert_eq!(choices, FilterValueChoices::default());
}

#[test]
fn a_value_never_touched_follows_a_changed_default() {
    // The saved choices hold only changes, so a value the user never touched
    // is not in them, whatever the defaults are in a later version.
    let mut choices = FilterValueChoices::default();
    choices.set_shown("Format", "PDF", false);

    assert!(choices.is_shown("Format", "Audio"));
    assert_eq!(choices.changes["format"].hidden.len(), 1);
    assert!(choices.changes["format"].shown.is_empty());
}

#[test]
fn the_values_are_split_into_shown_and_hidden_each_sorted_without_regard_to_case() {
    let values = ["pdf",
                  "Audio",
                  "Creation Method",
                  "EPUB",
                  "Human-Created Without AI",
                  "PDF"].map(String::from);

    let (shown, hidden) = FilterValueChoices::default().split("Format", &values);

    assert_eq!(shown, ["Audio", "EPUB", "pdf"]);
    assert_eq!(hidden, ["Creation Method", "Human-Created Without AI"]);
}

#[test]
fn enabling_a_value_moves_it_to_the_shown_group_and_hiding_moves_it_back() {
    let values = ["PDF", "Creation Method"].map(String::from);
    let mut choices = FilterValueChoices::default();

    choices.set_shown("Format", "Creation Method", true);
    let (shown, hidden) = choices.split("Format", &values);
    assert_eq!((shown.as_slice(), hidden.as_slice()),
               (["Creation Method".to_owned(), "PDF".to_owned()].as_slice(), [].as_slice()));

    choices.set_shown("Format", "Creation Method", false);
    let (shown, hidden) = choices.split("Format", &values);
    assert_eq!((shown, hidden),
               (vec!["PDF".to_owned()], vec!["Creation Method".to_owned()]));
}

#[test]
fn the_choices_survive_being_written_and_read_as_toml() {
    let mut choices = FilterValueChoices::default();
    choices.set_shown("Format", "Creation Method", true);
    choices.set_shown("Genre", "Horror", false);

    let text = toml::to_string(&choices).unwrap();
    let read: FilterValueChoices = toml::from_str(&text).unwrap();

    assert_eq!(read, choices);
}

#[test]
fn nothing_saved_reads_as_the_defaults() {
    let read: FilterValueChoices = toml::from_str("").unwrap();

    assert_eq!(read, FilterValueChoices::default());
}
