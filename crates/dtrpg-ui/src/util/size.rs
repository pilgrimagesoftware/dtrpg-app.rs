//! Utilities for formatting size values.

use rust_i18n::t;

/// Formats a size value in bytes as a human-readable string.
pub fn size_format(size: f64) -> String {
    match size {
        size if size >= 1024.0 * 1024.0 => {
            format!("{:.1} {}", size / (1024.0 * 1024.0), t!("size.tb"))
        }
        size if size >= 1024.0 => format!("{:.2} {}", size / 1024.0, t!("size.gb")),
        _ => format!("{:.0} {}", size, t!("size.mb")),
    }
}
