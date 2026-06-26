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
}

/// Sorts `items` in place according to `method`.
pub fn sort_items(items: &mut [LibraryItem], method: SortMethod) {
    match method {
        SortMethod::Title => items.sort_by(|a, b| a.title.cmp(&b.title)),
        SortMethod::Publisher => items.sort_by(|a, b| {
            a.publisher
                .cmp(&b.publisher)
                .then_with(|| a.title.cmp(&b.title))
        }),
        SortMethod::DateAdded => items.sort_by_key(|i| i.added_order),
        SortMethod::PageCount => items.sort_by(|a, b| {
            b.pages.cmp(&a.pages).then_with(|| a.title.cmp(&b.title))
        }),
    }
}
