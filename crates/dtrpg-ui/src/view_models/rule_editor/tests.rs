//! Tests for turning the rule editor's text into rules and suggestions.

use super::*;

fn collections() -> HashMap<u64, Arc<str>> {
    HashMap::from([(7, Arc::from("Solo Play")), (9, Arc::from("Horror"))])
}

#[test]
fn a_publisher_is_trimmed_and_an_empty_one_is_no_rule() {
    let rule = condition_from_text(RuleEditorType::Publisher, "  Free League ", &collections());
    let none = condition_from_text(RuleEditorType::Publisher, "   ", &collections());

    assert_eq!(rule,
               Some(PlacementCondition::Publisher("Free League".to_owned())));
    assert_eq!(none, None);
}

#[test]
fn a_collection_is_found_by_name_ignoring_case_or_by_number() {
    let by_name = condition_from_text(RuleEditorType::Collection, "solo play", &collections());
    let by_number = condition_from_text(RuleEditorType::Collection, "9", &collections());
    let neither = condition_from_text(RuleEditorType::Collection, "Nothing", &collections());

    assert_eq!(by_name, Some(PlacementCondition::Collection(7)));
    assert_eq!(by_number, Some(PlacementCondition::Collection(9)));
    assert_eq!(neither, None);
}

#[test]
fn a_size_must_be_a_number_of_at_least_zero() {
    let ok = condition_from_text(RuleEditorType::Size, "250.5", &collections());

    assert_eq!(ok, Some(PlacementCondition::SizeAtLeastMb(250.5)));
    for bad in ["", "big", "-1", "NaN", "inf"] {
        assert_eq!(condition_from_text(RuleEditorType::Size, bad, &collections()),
                   None,
                   "{bad:?}");
    }
}

#[test]
fn the_value_text_of_a_rule_reads_back_into_the_same_rule() {
    let names = collections();
    for condition in [PlacementCondition::Publisher("Chaosium".to_owned()),
                      PlacementCondition::Collection(7),
                      PlacementCondition::SizeAtLeastMb(100.0)]
    {
        let text = value_text(&condition, &names);

        let back = condition_from_text(RuleEditorType::of(&condition), &text, &names);

        assert_eq!(back, Some(condition));
    }
}

#[test]
fn suggestions_list_names_that_start_with_the_text_before_those_that_contain_it() {
    let publishers = vec!["Pelgrane Press".to_owned(),
                          "Chaosium".to_owned(),
                          "Cha Cha".to_owned(),
                          "Archaic".to_owned()];

    let names = suggestions(RuleEditorType::Publisher,
                            "CHA",
                            &publishers,
                            &collections());

    assert_eq!(names, ["Cha Cha", "Chaosium", "Archaic"]);
}

#[test]
fn suggestions_leave_out_an_exact_match_and_a_size_has_none() {
    let publishers = vec!["Chaosium".to_owned()];

    assert!(suggestions(RuleEditorType::Publisher,
                        "chaosium",
                        &publishers,
                        &collections()).is_empty());
    assert!(suggestions(RuleEditorType::Size, "1", &publishers, &collections()).is_empty());
    assert!(suggestions(RuleEditorType::Publisher, "", &publishers, &collections()).is_empty());
}

#[test]
fn collection_suggestions_are_the_collection_names() {
    let names = suggestions(RuleEditorType::Collection, "o", &[], &collections());

    assert_eq!(names, ["Horror", "Solo Play"]);
}

#[test]
fn twenty_suggestions_are_kept_for_the_popup_to_scroll() {
    let publishers: Vec<String> = (0..30).map(|n| format!("Press {n:02}")).collect();

    let names = suggestions(RuleEditorType::Publisher,
                            "press",
                            &publishers,
                            &collections());

    assert_eq!(names.len(), MAX_SUGGESTIONS);
    assert_eq!(VISIBLE_SUGGESTIONS, 6);
    assert_eq!(MAX_SUGGESTIONS, 20);
    assert_eq!(names.first().map(String::as_str), Some("Press 00"));
}
