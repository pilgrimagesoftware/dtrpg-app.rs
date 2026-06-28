//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use crate::data::library::LibraryItem;

// ── Sorting ───────────────────────────────────────────────────────────────────

/// Field used to sort the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortMethod {
    #[default]
    Title,
    Publisher,
    DateAdded,
    PageCount,
    /// Sort driven by a column header click; carries the column key string.
    Custom { col_key: &'static str },
}

/// Direction for catalog sorting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

/// Sorts `items` in place according to `method` and `direction`.
pub fn sort_items(items: &mut [LibraryItem], method: SortMethod, direction: SortDirection) {
    match method {
        SortMethod::Title => items.sort_by(|a, b| a.title.cmp(&b.title)),
        SortMethod::Publisher => items.sort_by(|a, b| {
            a.publisher
                .cmp(&b.publisher)
                .then_with(|| a.title.cmp(&b.title))
        }),
        SortMethod::DateAdded => items.sort_by_key(|i| i.added_order),
        SortMethod::PageCount => {
            items.sort_by(|a, b| a.pages.cmp(&b.pages).then_with(|| a.title.cmp(&b.title)))
        }
        SortMethod::Custom { col_key: "publisher" } => items.sort_by(|a, b| {
            a.publisher
                .cmp(&b.publisher)
                .then_with(|| a.title.cmp(&b.title))
        }),
        SortMethod::Custom { col_key: "system" } => {
            items.sort_by(|a, b| a.line.cmp(&b.line).then_with(|| a.title.cmp(&b.title)))
        }
        SortMethod::Custom { col_key: "pages" } => {
            items.sort_by(|a, b| a.pages.cmp(&b.pages).then_with(|| a.title.cmp(&b.title)))
        }
        SortMethod::Custom { col_key: "size" } => items.sort_by(|a, b| {
            a.size_mb
                .partial_cmp(&b.size_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.title.cmp(&b.title))
        }),
        SortMethod::Custom { col_key: "added" } => items.sort_by_key(|i| i.added_order),
        SortMethod::Custom { .. } => items.sort_by(|a, b| a.title.cmp(&b.title)),
    }

    if direction == SortDirection::Descending {
        items.reverse();
    }
}
