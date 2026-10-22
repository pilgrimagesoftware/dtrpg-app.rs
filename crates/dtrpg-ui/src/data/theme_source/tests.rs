//! Tests for reading, checking and saving theme text.

#![allow(clippy::unwrap_used)] // Test fixtures intentionally use unwrap for concise assertions.

use super::*;

#[test]
fn fonts_are_read_from_the_theme_text() {
    let fonts =
        ThemeFonts::from_source("name = \"A\"\nbody_font = \"Inter\"\nmono_font = \" Menlo \"\n");

    assert_eq!(fonts.body.as_deref(), Some("Inter"));
    assert_eq!(fonts.mono.as_deref(), Some("Menlo"));
    assert_eq!(fonts.label, None);
    assert_eq!(fonts.value, None);
}

#[test]
fn a_blank_font_name_is_the_same_as_none() {
    assert_eq!(ThemeFonts::from_source("name = \"A\"\nbody_font = \"  \"\n").body,
               None);
}

#[test]
fn text_that_is_not_toml_names_no_fonts() {
    assert_eq!(ThemeFonts::from_source("name = [broken"),
               ThemeFonts::default());
}

#[test]
fn a_built_in_theme_names_no_fonts_and_is_built_in() {
    let ink = ThemeId::from("ink");

    assert!(is_builtin(&ink));
    assert_eq!(fonts_of(&ink), ThemeFonts::default());
}

#[test]
fn a_theme_with_a_bad_color_is_rejected_and_says_which() {
    let error = validate_source("name = \"A\"\naccent = \"orange\"\n").unwrap_err();

    assert!(error.starts_with("accent"));
}

#[test]
fn a_theme_with_alpha_colors_and_unknown_keys_is_valid() {
    let source = "name = \"A\"\nshadow = \"#000000 0.5\"\nfuture_key = 3\n";

    assert!(validate_source(source).is_ok());
}

#[test]
fn a_theme_without_a_name_is_rejected() {
    assert!(validate_source("surface = \"#101010\"\n").is_err());
}

#[test]
fn an_unsafe_id_is_never_saved() {
    let error = save_user_theme(&ThemeId::from("../evil"), "name = \"A\"\n").unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[test]
fn invalid_text_is_not_saved() {
    let error = save_user_theme(&ThemeId::from("mine"), "name = [broken").unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
}
