//! Locale initialization and the embedded string catalog.
//!
//! Call [`init`] once at startup, before any view renders. After that, use
//! `t!("module.key")` anywhere in the crate to get a translated string.
//!
//! # Supported locales
//! `en` (authoritative), `fr` (stub), `de` (stub). The system locale is
//! detected at startup via `sys-locale`; unrecognized locales fall back to `en`.

/// Detect the system locale, select the best available locale, and activate it.
///
/// Must be called before any `t!()` macro is evaluated.
pub fn init() {
    let raw = sys_locale::get_locale();

    let locale = raw
        .as_deref()
        .and_then(|l| l.split(['-', '_']).next())
        .unwrap_or("en")
        .to_owned();

    let supported = ["en", "fr", "de"];
    let chosen = if supported.contains(&locale.as_str()) {
        locale
    } else {
        "en".to_owned()
    };

    rust_i18n::set_locale(&chosen);
    tracing::debug!(locale = %chosen, "i18n locale initialized");
}
