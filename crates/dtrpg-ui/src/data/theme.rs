//! Libri theme system: four color themes and two density variants.

use gpui::{px, Hsla, Pixels}; // Pixels kept for GPUI layout fields

// ── Color tokens ──────────────────────────────────────────────────────────────

/// Semantic color tokens for one Libri theme.
#[derive(Debug, Clone)]
pub struct ColorTokens {
    /// App desktop background color.
    pub desktop_bg: Hsla,
    /// Main window / panel background.
    pub surface: Hsla,
    /// Sidebar / secondary surface background.
    pub surface_alt: Hsla,
    /// Hover state background.
    pub hover: Hsla,
    /// Primary text.
    pub text_primary: Hsla,
    /// Secondary / dimmed text.
    pub text_secondary: Hsla,
    /// Tertiary / placeholder text.
    pub text_tertiary: Hsla,
    /// Default border / divider.
    pub border: Hsla,
    /// Stronger border for inputs.
    pub border_strong: Hsla,
    /// Accent (active nav, focus rings).
    pub accent: Hsla,
    /// Accent at low opacity for backgrounds.
    pub accent_soft: Hsla,
    /// Text color drawn on top of an accent background.
    pub accent_on: Hsla,
    /// Drop shadow color.
    pub shadow: Hsla,
    /// Overlay scrim.
    pub scrim: Hsla,
    /// Error / destructive state (red).
    pub error: Hsla,
    /// Warning banner background (amber, low opacity).
    pub warning_bg: Hsla,
    /// Warning banner text / icon color (amber, full opacity).
    pub warning_text: Hsla,
}

// ── Density ───────────────────────────────────────────────────────────────────

/// Spacing density variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Density {
    #[default]
    Comfortable,
    Compact,
}

/// Layout constants that vary with density.
#[derive(Debug, Clone)]
pub struct DensityConstants {
    /// Height of a text-list row.
    pub row_text_height: Pixels,
    /// Width of a thumbnail in a thumbs-list row (plain f32 for arithmetic).
    pub thumb_width: f32,
    /// Minimum width of a grid card (plain f32 for arithmetic).
    pub card_min_width: f32,
    /// Horizontal gap between grid cards.
    pub card_gap_x: Pixels,
    /// Vertical gap between grid cards.
    pub card_gap_y: Pixels,
    /// Catalog area padding (top/side/bottom).
    pub catalog_pad_top: Pixels,
    pub catalog_pad_side: Pixels,
    pub catalog_pad_bottom: Pixels,
}

impl DensityConstants {
    fn comfortable() -> Self {
        Self {
            row_text_height: px(44.0),
            thumb_width: 46.0,
            card_min_width: 158.0,
            card_gap_x: px(22.0),
            card_gap_y: px(26.0),
            catalog_pad_top: px(18.0),
            catalog_pad_side: px(22.0),
            catalog_pad_bottom: px(48.0),
        }
    }

    fn compact() -> Self {
        Self {
            row_text_height: px(33.0),
            thumb_width: 40.0,
            card_min_width: 132.0,
            card_gap_x: px(16.0),
            card_gap_y: px(18.0),
            catalog_pad_top: px(12.0),
            catalog_pad_side: px(20.0),
            catalog_pad_bottom: px(40.0),
        }
    }
}

// ── Theme key ─────────────────────────────────────────────────────────────────

/// Identifies one of the four named themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeKey {
    #[default]
    Parchment,
    Slate,
    Sage,
    Ink,
}

// ── LibriTheme ────────────────────────────────────────────────────────────────

/// GPUI app-level global containing the active Libri theme and density.
#[derive(Debug, Clone)]
pub struct LibriTheme {
    pub key: ThemeKey,
    pub colors: ColorTokens,
    pub density: Density,
    pub density_constants: DensityConstants,
}

impl gpui::Global for LibriTheme {}

impl LibriTheme {
    /// Constructs the theme for `key` and `density`.
    pub fn new(key: ThemeKey, density: Density) -> Self {
        let colors = match key {
            ThemeKey::Parchment => parchment_colors(),
            ThemeKey::Slate => slate_colors(),
            ThemeKey::Sage => sage_colors(),
            ThemeKey::Ink => ink_colors(),
        };
        let density_constants = match density {
            Density::Comfortable => DensityConstants::comfortable(),
            Density::Compact => DensityConstants::compact(),
        };
        Self { key, colors, density, density_constants }
    }

    /// Returns the default theme (parchment, comfortable).
    pub fn default_theme() -> Self {
        Self::new(ThemeKey::Parchment, Density::Comfortable)
    }
}

// ── Color constructors ────────────────────────────────────────────────────────

fn hex(r: u8, g: u8, b: u8) -> Hsla {
    let n = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
    gpui::rgb(n).into()
}

fn hex_a(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    let n = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
    let base: Hsla = gpui::rgb(n).into();
    Hsla { a, ..base }
}

// ── Parchment ─────────────────────────────────────────────────────────────────
// --bg:#FAF7F0  --surface:#FCF9F3  --surface-2:#F2ECDF  --hover:#EDE6D6
// --text:#26211A  --text-2:#5B5346  --text-3:#8C8270
// --line:#E7DFCD  --line-2:#DBD1BB  --accent-on:#FCF9F3
// --shadow:rgba(58,46,26,0.18)  --scrim:rgba(30,22,10,0.26)
// accent: oklch(0.47 0.105 25) ≈ #8C4A22 (warm brown-orange)

fn parchment_colors() -> ColorTokens {
    ColorTokens {
        desktop_bg:     hex(0xC5, 0xB9, 0x9D),
        surface:        hex(0xFC, 0xF9, 0xF3),
        surface_alt:    hex(0xF2, 0xEC, 0xDF),
        hover:          hex(0xED, 0xE6, 0xD6),
        text_primary:   hex(0x26, 0x21, 0x1A),
        text_secondary: hex(0x5B, 0x53, 0x46),
        text_tertiary:  hex(0x8C, 0x82, 0x70),
        border:         hex(0xE7, 0xDF, 0xCD),
        border_strong:  hex(0xDB, 0xD1, 0xBB),
        accent:         hex(0x8C, 0x4A, 0x22),
        accent_soft:    hex_a(0x8C, 0x4A, 0x22, 0.13),
        accent_on:      hex(0xFC, 0xF9, 0xF3),
        shadow:         hex_a(0x3A, 0x2E, 0x1A, 0.18),
        scrim:          hex_a(0x1E, 0x16, 0x0A, 0.26),
        error:          hex(0xB0, 0x30, 0x28),
        warning_bg:     gpui::hsla(0.11, 0.9, 0.5, 0.12),
        warning_text:   gpui::hsla(0.08, 0.85, 0.35, 1.0),
    }
}

// ── Slate ─────────────────────────────────────────────────────────────────────
// --bg:#FAFBFC  --surface:#FCFDFE  --surface-2:#EEF1F4  --hover:#E7ECF0
// --text:#1B2530  --text-2:#4C5965  --text-3:#7E8B98
// --line:#E4E9ED  --line-2:#D5DCE2  --accent-on:#FCFDFE
// --shadow:rgba(28,42,58,0.18)  --scrim:rgba(18,28,40,0.26)
// accent: oklch(0.47 0.095 25) ≈ #7A4220 (cooler warm)

fn slate_colors() -> ColorTokens {
    ColorTokens {
        desktop_bg:     hex(0xAD, 0xB7, 0xBF),
        surface:        hex(0xFC, 0xFD, 0xFE),
        surface_alt:    hex(0xEE, 0xF1, 0xF4),
        hover:          hex(0xE7, 0xEC, 0xF0),
        text_primary:   hex(0x1B, 0x25, 0x30),
        text_secondary: hex(0x4C, 0x59, 0x65),
        text_tertiary:  hex(0x7E, 0x8B, 0x98),
        border:         hex(0xE4, 0xE9, 0xED),
        border_strong:  hex(0xD5, 0xDC, 0xE2),
        accent:         hex(0x7A, 0x42, 0x20),
        accent_soft:    hex_a(0x7A, 0x42, 0x20, 0.13),
        accent_on:      hex(0xFC, 0xFD, 0xFE),
        shadow:         hex_a(0x1C, 0x2A, 0x3A, 0.18),
        scrim:          hex_a(0x12, 0x1C, 0x28, 0.26),
        error:          hex(0xB0, 0x30, 0x28),
        warning_bg:     gpui::hsla(0.11, 0.9, 0.5, 0.12),
        warning_text:   gpui::hsla(0.08, 0.85, 0.35, 1.0),
    }
}

// ── Sage ──────────────────────────────────────────────────────────────────────
// --bg:#F8F9F4  --surface:#FAFBF6  --surface-2:#EDF0E7  --hover:#E6EBDE
// --text:#23271F  --text-2:#515A4A  --text-3:#848D78
// --line:#E2E7DA  --line-2:#D4DBC8  --accent-on:#FAFBF6
// --shadow:rgba(38,48,28,0.18)  --scrim:rgba(24,32,18,0.26)
// accent: oklch(0.47 0.095 25) ≈ #7A4220

fn sage_colors() -> ColorTokens {
    ColorTokens {
        desktop_bg:     hex(0xB2, 0xBC, 0xA0),
        surface:        hex(0xFA, 0xFB, 0xF6),
        surface_alt:    hex(0xED, 0xF0, 0xE7),
        hover:          hex(0xE6, 0xEB, 0xDE),
        text_primary:   hex(0x23, 0x27, 0x1F),
        text_secondary: hex(0x51, 0x5A, 0x4A),
        text_tertiary:  hex(0x84, 0x8D, 0x78),
        border:         hex(0xE2, 0xE7, 0xDA),
        border_strong:  hex(0xD4, 0xDB, 0xC8),
        accent:         hex(0x7A, 0x42, 0x20),
        accent_soft:    hex_a(0x7A, 0x42, 0x20, 0.13),
        accent_on:      hex(0xFA, 0xFB, 0xF6),
        shadow:         hex_a(0x26, 0x30, 0x1C, 0.18),
        scrim:          hex_a(0x18, 0x20, 0x12, 0.26),
        error:          hex(0xB0, 0x30, 0x28),
        warning_bg:     gpui::hsla(0.11, 0.9, 0.5, 0.12),
        warning_text:   gpui::hsla(0.08, 0.85, 0.35, 1.0),
    }
}

// ── Ink ───────────────────────────────────────────────────────────────────────
// --bg:#16130D  --surface:#1B1812  --surface-2:#211D15  --hover:#2A241B
// --text:#ECE4D3  --text-2:#B4AA94  --text-3:#877D68
// --line:#2C271E  --line-2:#392F23  --accent-on:#1B1812
// --shadow:rgba(0,0,0,0.5)  --scrim:rgba(0,0,0,0.45)
// accent: oklch(0.76 0.115 25) ≈ #E0845A (light warm)

fn ink_colors() -> ColorTokens {
    ColorTokens {
        desktop_bg:     hex(0x14, 0x11, 0x0A),
        surface:        hex(0x1B, 0x18, 0x12),
        surface_alt:    hex(0x21, 0x1D, 0x15),
        hover:          hex(0x2A, 0x24, 0x1B),
        text_primary:   hex(0xEC, 0xE4, 0xD3),
        text_secondary: hex(0xB4, 0xAA, 0x94),
        text_tertiary:  hex(0x87, 0x7D, 0x68),
        border:         hex(0x2C, 0x27, 0x1E),
        border_strong:  hex(0x39, 0x2F, 0x23),
        accent:         hex(0xE0, 0x84, 0x5A),
        accent_soft:    hex_a(0xE0, 0x84, 0x5A, 0.13),
        accent_on:      hex(0x1B, 0x18, 0x12),
        shadow:         hex_a(0x00, 0x00, 0x00, 0.50),
        scrim:          hex_a(0x00, 0x00, 0x00, 0.45),
        error:          hex(0xE0, 0x58, 0x58),
        warning_bg:     gpui::hsla(0.11, 0.9, 0.5, 0.10),
        warning_text:   gpui::hsla(0.10, 0.90, 0.65, 1.0),
    }
}
