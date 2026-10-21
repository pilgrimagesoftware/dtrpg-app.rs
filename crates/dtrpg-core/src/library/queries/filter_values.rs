//! The filter values the catalog has, whichever items are on show.

use std::sync::PoisonError;

use crate::data::index::ItemFilter;
use crate::library::Library;
use crate::util::search::Field;

impl Library {
    /// Every distinct value of `field` that any item in the catalog has, one
    /// spelling of each. Only the fields that read product filters (system,
    /// type, format and genre) have values. This is what the settings page
    /// lists for the user to show or hide.
    ///
    /// Reads the index, so call it when the catalog changes, not while drawing.
    #[must_use]
    pub fn filter_values(&self, field: Field) -> Vec<String> {
        self.index
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .facet_counts(&ItemFilter::default(), field)
            .inspect_err(|error| tracing::warn!(%error, ?field, "could not list filter values"))
            .unwrap_or_default()
            .into_iter()
            .map(|count| count.value)
            .collect()
    }
}
