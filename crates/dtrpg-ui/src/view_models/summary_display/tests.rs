//! Tests for the Summaries settings' choices and wording.

use super::*;

#[test]
fn a_model_saved_by_hand_is_added_to_the_menu() {
    let choices = model_choices("my-model");

    assert_eq!(choices.len(), SUMMARY_MODELS.len() + 1);
    assert_eq!(choices.last().map(String::as_str), Some("my-model"));
}

#[test]
fn a_listed_model_is_not_added_again() {
    assert_eq!(model_choices("claude-opus-5-5").len(), SUMMARY_MODELS.len());
}

#[test]
fn an_unlisted_ceiling_is_put_in_order() {
    assert_eq!(ceiling_choices(300_000),
               [100_000, 250_000, 300_000, 500_000, 1_000_000, 2_500_000]);
    assert_eq!(ceiling_choices(500_000), TOKEN_CEILINGS);
}

#[test]
fn counts_are_grouped_and_shortened() {
    assert_eq!(grouped(0), "0");
    assert_eq!(grouped(999), "999");
    assert_eq!(grouped(1_000), "1,000");
    assert_eq!(compact(999), "999");
    assert_eq!(compact(1_000), "1k");
    assert_eq!(compact(18_449), "18.4k");
    assert_eq!(compact(2_500_000), "2.5M");
}

#[test]
fn the_ceiling_note_resolves() {
    assert_ne!(ceiling_note(18_400, 500_000), "agent.summary_ceiling_note");
}

#[test]
fn the_key_hint_says_where_the_key_comes_from_and_the_environment_comes_first() {
    let none = key_hint(AnthropicKeyStatus { env_var: false,
                                             saved:   false, });
    let saved = key_hint(AnthropicKeyStatus { env_var: false,
                                              saved:   true, });
    let env = key_hint(AnthropicKeyStatus { env_var: true,
                                            saved:   false, });
    let both = key_hint(AnthropicKeyStatus { env_var: true,
                                             saved:   true, });

    assert_ne!(none, saved);
    assert_ne!(saved, env);
    assert_eq!(env, both);
    for key in ["agent.summary_key_missing",
                "agent.summary_key_saved",
                "agent.summary_key_env",
                "agent.summary_key_placeholder"]
    {
        assert_ne!(t!(key).to_string(), key, "{key} has no text");
    }
}
