//! The text of a theme file: the fonts it names, whether a piece of text is a
//! valid theme, and saving an edited user theme.

use std::io;

use crate::data::theme_catalog::ThemeId;
use crate::data::theme_loader::{
    FontsToml, PaletteToml, builtin_toml, parse_hex, parse_hex_alpha, parse_theme_toml,
    user_theme_path,
};

/// The font families a theme file names. A role the file leaves out is `None`
/// and uses the built-in default.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ThemeFonts {
    /// For the catalog and detail text.
    pub body:  Option<String>,
    /// For data values, such as the cache details.
    pub value: Option<String>,
    /// For field and row labels.
    pub label: Option<String>,
    /// For fixed-width text, such as paths.
    pub mono:  Option<String>,
}

impl ThemeFonts {
    /// Reads the font keys from theme text. Text that is not valid TOML names
    /// no fonts.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_ui::data::theme_source::ThemeFonts;
    ///
    /// let fonts = ThemeFonts::from_source("name = \"Mine\"\nbody_font = \"Inter\"\n");
    /// assert_eq!(fonts.body.as_deref(), Some("Inter"));
    /// assert_eq!(fonts.mono, None);
    /// ```
    #[must_use]
    pub fn from_source(source: &str) -> Self {
        let fonts: FontsToml = toml::from_str(source).unwrap_or_default();
        Self { body:  non_blank(fonts.body_font),
               value: non_blank(fonts.value_font),
               label: non_blank(fonts.label_font),
               mono:  non_blank(fonts.mono_font), }
    }
}

/// `None` for a missing or blank family name.
fn non_blank(name: Option<String>) -> Option<String> {
    name.map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
}

/// The fonts the theme `id` names, or none if its file cannot be read.
#[must_use]
pub fn fonts_of(id: &ThemeId) -> ThemeFonts {
    crate::data::theme_loader::theme_source(id).map(|source| ThemeFonts::from_source(&source))
                                               .unwrap_or_default()
}

/// Whether `id` is a built-in theme, which can be copied but not edited.
#[must_use]
pub fn is_builtin(id: &ThemeId) -> bool {
    builtin_toml(id.as_str()).is_some()
}

/// Checks that `source` is a theme Libri can load: valid TOML with a name, and
/// every color that is present written as a color.
///
/// A missing color is fine, as it falls back to the built-in light or dark
/// theme, and so is a key Libri does not know.
///
/// # Errors
///
/// Returns a one-line message saying what is wrong, for showing to the person
/// editing the theme.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::data::theme_source::validate_source;
///
/// assert!(validate_source("name = \"Mine\"\nsurface = \"#101010\"\n").is_ok());
/// assert!(validate_source("name = \"Mine\"\nsurface = \"blue\"\n").is_err());
/// ```
pub fn validate_source(source: &str) -> Result<(), String> {
    parse_theme_toml(source).map_err(|error| error.message().to_owned())?;
    let palette: PaletteToml = toml::from_str(source).map_err(|error| error.message().to_owned())?;
    let colors = [("desktop_bg", &palette.desktop_bg, false),
                  ("surface", &palette.surface, false),
                  ("surface_alt", &palette.surface_alt, false),
                  ("hover", &palette.hover, false),
                  ("text_primary", &palette.text_primary, false),
                  ("text_secondary", &palette.text_secondary, false),
                  ("text_tertiary", &palette.text_tertiary, false),
                  ("border", &palette.border, false),
                  ("border_strong", &palette.border_strong, false),
                  ("accent", &palette.accent, false),
                  ("accent_soft", &palette.accent_soft, true),
                  ("accent_on", &palette.accent_on, false),
                  ("shadow", &palette.shadow, true),
                  ("scrim", &palette.scrim, true)];
    for (key, value, has_alpha) in colors {
        let Some(value) = value
        else {
            continue;
        };
        let valid = if has_alpha {
            parse_hex_alpha(value).is_some()
        }
        else {
            parse_hex(value).is_some()
        };
        if !valid {
            return Err(format!("{key}: {value:?} is not a color like \"#1B1812\""));
        }
    }
    Ok(())
}

/// Validates `source` and writes it as the user theme `id`.
///
/// # Errors
///
/// Returns an error if `source` is not a valid theme, if `id` is not a safe
/// file name, or if the file cannot be written.
pub fn save_user_theme(id: &ThemeId, source: &str) -> io::Result<()> {
    validate_source(source).map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
    let path = user_theme_path(id.as_str()).ok_or_else(|| {
                                               io::Error::new(io::ErrorKind::InvalidInput,
                                                              "not a usable theme id")
                                           })?;
    std::fs::write(path, source)
}

#[cfg(test)]
mod tests;
