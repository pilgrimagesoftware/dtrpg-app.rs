//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_catalog_has_expected_count() {
        assert_eq!(stub_catalog().len(), 46);
    }

    #[test]
    fn section_counts_sum_to_total() {
        let catalog = stub_catalog();
        let counts = section_counts(&catalog);
        assert_eq!(counts.on_device + counts.in_cloud, counts.all);
    }

    #[test]
    fn item_matches_query_case_insensitive() {
        let item = stub_catalog().remove(0); // Player's Handbook
        assert!(item_matches_query(&item, "player"));
        assert!(item_matches_query(&item, "PLAYER"));
        assert!(!item_matches_query(&item, "starfinder"));
    }

    #[test]
    fn publisher_entries_ordered_count_desc() {
        let catalog = stub_catalog();
        let entries = publisher_entries(&catalog);
        for w in entries.windows(2) {
            assert!(
                w[0].count >= w[1].count,
                "publisher entries not sorted by count desc"
            );
        }
    }
}
