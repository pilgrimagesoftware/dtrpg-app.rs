//! What the tools take and what they reply with.

use dtrpg_core::data::summary::ContentSummary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A tag as a tool reports it.
#[derive(Serialize)]
pub struct TagInfo {
    pub name:  String,
    /// The tag's color, or `null`.
    pub color: Option<String>,
    /// How many items have the tag. Absent for an item's own tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetItemRequest {
    pub id: String,
}

/// A summary as a tool reports it.
#[derive(Serialize)]
pub struct SummaryInfo {
    pub text:       String,
    /// The provider and model that wrote it.
    pub model:      String,
    /// Unix timestamp, in seconds, of when it was written.
    pub created_at: i64,
}

impl From<ContentSummary> for SummaryInfo {
    fn from(summary: ContentSummary) -> Self {
        Self { text:       summary.text,
               model:      summary.model,
               created_at: summary.created_at, }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SetTagsRequest {
    /// The item's id, as `list_items` gives it.
    pub id:   String,
    /// Every tag the item should have after this. Replaces the item's tags.
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateCollectionRequest {
    /// The new collection's name.
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AddToCollectionRequest {
    /// The collection's id, or its name, ignoring case, as `list_collections`
    /// gives them.
    pub collection: String,
    /// The item's id, as `list_items` gives it.
    pub id:         String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SetTagColorRequest {
    /// The tag's name, ignoring case.
    pub tag:   String,
    /// One of gray, green, purple, blue, yellow, red or orange, or `null` or
    /// "none" to clear the color.
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ListArchiveEntriesRequest {
    pub path: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SearchLibraryRequest {
    /// The search, in the same form as Libri's search box: words, or
    /// `field:value` terms such as `publisher:paizo`, `tag:solo`,
    /// `format:pdf` or `availability:cloud`. A leading `-` leaves a term
    /// out.
    pub query: String,
}
