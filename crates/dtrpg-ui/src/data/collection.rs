//! Collection domain type.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// A user-created product list (DTRPG "collection").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEntry {
    /// Numeric product list identifier.
    pub id: u64,
    /// Display name of the collection.
    pub name: Arc<str>,
    /// Product `order_product_id` values belonging to this collection.
    pub member_ids: Arc<[u64]>,
}
