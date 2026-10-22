//! The footer under the sidebar: what the library holds, in two lines, level
//! with the status bar beside it.

use gpui::prelude::*;
use gpui::{Div, div, px};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::util::pluralize::pluralize;
use crate::util::size::size_format;
use crate::view_models::summary_display::grouped;

/// What the footer says about the library.
pub struct SidebarFooterData {
    /// How many titles the library holds.
    pub titles:    usize,
    /// Their total size, in megabytes.
    pub total_mb:  f64,
    /// Their total page count.
    pub pages:     u64,
    /// How many storage locations there are.
    pub locations: usize,
}

/// Renders the footer: titles and size on one line, pages and locations on the
/// next. The caller gives it the sidebar's width and background.
pub fn render_sidebar_footer(data: &SidebarFooterData, colors: &ColorTokens) -> Div {
    let line = |text: String| {
        div().text_xs()
             .text_color(colors.text_secondary)
             .child(text)
    };
    div().flex()
         .flex_col()
         .justify_center()
         .gap(px(2.0))
         .px(px(12.0))
         .py(px(6.0))
         .child(line(t!("sidebar.footer_totals",
                        titles = pluralize(data.titles, "count.title", "count.titles"),
                        size = size_format(data.total_mb)).to_string()))
         .child(line(t!("sidebar.footer_pages",
                        pages = grouped(data.pages),
                        locations = locations_label(data.locations)).to_string()))
}

/// "1 location" or "3 locations".
fn locations_label(count: usize) -> String {
    pluralize(count, "count.location", "count.locations")
}

#[cfg(test)]
mod tests {
    use rust_i18n::t;

    use super::*;

    #[test]
    fn the_footer_lines_say_titles_size_pages_and_locations() {
        let totals = t!("sidebar.footer_totals",
                        titles = "18 titles",
                        size = "2.6 GB").to_string();
        let pages = t!("sidebar.footer_pages",
                       pages = "4,804",
                       locations = "3 locations").to_string();

        assert!(totals.contains("18 titles") && totals.contains("2.6 GB"));
        assert!(pages.contains("4,804") && pages.contains("3 locations"));
    }

    #[test]
    fn locations_are_counted_in_the_singular_and_the_plural() {
        assert_eq!(locations_label(1), "1 location");
        assert_eq!(locations_label(3), "3 locations");
    }

    #[test]
    fn the_status_bar_texts_resolve() {
        for key in ["status_bar.titles_of", "status_bar.downloading"] {
            assert_ne!(t!(key, shown = 1, total = "2 titles", n = 2).to_string(),
                       key);
        }
    }
}
