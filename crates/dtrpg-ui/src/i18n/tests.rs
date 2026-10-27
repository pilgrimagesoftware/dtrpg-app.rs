use crate::i18n::locale::{Locale, resolve_locale};

/// A placeholder is written `%{name}`. A bare `{name}` is printed as it is,
/// braces and all, so no text may have one.
#[test]
fn every_placeholder_in_every_language_file_uses_the_percent_form() {
    let files = [("en", include_str!("../../i18n/en.yaml")),
                 ("de", include_str!("../../i18n/de.yaml")),
                 ("fr", include_str!("../../i18n/fr.yaml"))];
    for (locale, text) in files {
        for (line_number, line) in text.lines().enumerate() {
            let bytes = line.as_bytes();
            for (ix, byte) in bytes.iter().enumerate() {
                let bare_brace = *byte == b'{' && (ix == 0 || bytes[ix - 1] != b'%');
                assert!(!bare_brace,
                        "{locale}.yaml line {}: `{{` is not a placeholder; write `%{{name}}`: {line}",
                        line_number + 1);
            }
        }
    }
}

#[test]
fn locale_code_and_label_round_trip() {
    for locale in Locale::ALL {
        assert_eq!(Locale::from_code(locale.code()), Some(locale));
        assert!(!locale.label().is_empty());
    }
}

#[test]
fn from_code_rejects_unsupported() {
    assert_eq!(Locale::from_code("xx"), None);
}

#[test]
fn resolve_locale_prefers_valid_persisted_override() {
    assert_eq!(resolve_locale(Some("fr"), "en"), "fr");
}

#[test]
fn resolve_locale_falls_back_on_unsupported_persisted_code() {
    assert_eq!(resolve_locale(Some("xx"), "de"), "de");
}

#[test]
fn resolve_locale_falls_back_when_nothing_persisted() {
    assert_eq!(resolve_locale(None, "de"), "de");
}
