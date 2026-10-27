//! The tools that read the library. Each asks the gate first, which refuses it
//! when the read scope or the tool is switched off.

use std::path::PathBuf;

use dtrpg_core::data::collection::entry::CollectionEntry;
use dtrpg_core::data::index::{ItemFilter, LibraryIndex};
use dtrpg_core::data::mcp_access::McpTool;
use dtrpg_core::library::insights::Insights;
use dtrpg_core::util::datetime::now_secs;
use dtrpg_core::util::search::SearchQuery;
use dtrpg_core::util::sort::{SortDirection, SortMethod};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use serde::Serialize;

use crate::requests::{
    GetItemRequest, ListArchiveEntriesRequest, SearchLibraryRequest, SummaryInfo, TagInfo,
};
use crate::server::{LibriMcpServer, error_json};

/// A collection as a tool reports it.
#[derive(Serialize)]
struct CollectionInfo {
    id:     u64,
    name:   String,
    /// How many products the collection lists.
    titles: usize,
}

impl From<&CollectionEntry> for CollectionInfo {
    fn from(entry: &CollectionEntry) -> Self {
        Self { id:     entry.id,
               name:   entry.name.to_string(),
               titles: entry.member_ids.len(), }
    }
}

#[tool_router(router = read_router, vis = "pub(crate)")]
impl LibriMcpServer {
    #[tool(description = "List items in the local library index")]
    pub(crate) async fn list_items(&self) -> String {
        if let Some(denied) = self.gate(McpTool::ListItems, "") {
            return denied;
        }
        match self.api.index_list_items() {
            Ok(items) => serde_json::to_string(&items).unwrap_or_default(),
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Get a library item by id")]
    pub(crate) async fn get_item(&self,
                                 Parameters(GetItemRequest { id }): Parameters<GetItemRequest>)
                                 -> String {
        if let Some(denied) = self.gate(McpTool::GetItem, &id) {
            return denied;
        }
        match self.api.index_get_item(&id) {
            Ok(Some(item)) => serde_json::to_string(&item).unwrap_or_default(),
            Ok(None) => error_json(&format!("item {id} not found")),
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Search the library the way Libri's search box does: words, or field:value terms such as publisher:paizo, tag:solo, format:pdf or availability:cloud. A leading - leaves a term out. Replies with the matching items, by title.")]
    pub(crate) async fn search_library(&self,
                                       Parameters(SearchLibraryRequest { query }): Parameters<SearchLibraryRequest>)
                                       -> String {
        if let Some(denied) = self.gate(McpTool::SearchLibrary, &query) {
            return denied;
        }
        let parsed = SearchQuery::parse(&query);
        let found =
            LibraryIndex::open_and_migrate(self.api.index_root()).and_then(|index| {
                let filter = ItemFilter { search: Some(&parsed),
                                          ..ItemFilter::default() };
                index.list_items_filtered(&filter, SortMethod::Title, SortDirection::Ascending)
            });
        match found {
            Ok(items) => serde_json::to_string(&items).unwrap_or_default(),
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "List every tag in the library, with its color and how many items have it")]
    pub(crate) async fn list_tags(&self) -> String {
        if let Some(denied) = self.gate(McpTool::ListTags, "") {
            return denied;
        }
        match self.api.index_tag_library() {
            Ok(library) => {
                let tags: Vec<TagInfo> =
                    library.entries()
                           .iter()
                           .map(|entry| TagInfo { name:  entry.name.to_string(),
                                                  color: entry.color.map(|c| c.to_string()),
                                                  count: Some(entry.count), })
                           .collect();
                serde_json::to_string(&tags).unwrap_or_default()
            }
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Get the tags of a library item by id")]
    pub(crate) async fn get_item_tags(&self,
                                      Parameters(GetItemRequest { id }): Parameters<GetItemRequest>)
                                      -> String {
        if let Some(denied) = self.gate(McpTool::GetItemTags, &id) {
            return denied;
        }
        match self.api.index_tag_library() {
            Ok(library) => {
                let tags: Vec<TagInfo> =
                    library.tags_of(&id)
                           .into_iter()
                           .map(|entry| TagInfo { name:  entry.name.to_string(),
                                                  color: entry.color.map(|c| c.to_string()),
                                                  count: None, })
                           .collect();
                serde_json::to_string(&tags).unwrap_or_default()
            }
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Get the saved content summary of a library item by id, or null if it has none")]
    pub(crate) async fn get_summary(&self,
                                    Parameters(GetItemRequest { id }): Parameters<GetItemRequest>)
                                    -> String {
        if let Some(denied) = self.gate(McpTool::GetSummary, &id) {
            return denied;
        }
        match self.api.index_get_summary(&id) {
            Ok(summary) => {
                let summary = summary.map(SummaryInfo::from);
                serde_json::to_string(&summary).unwrap_or_default()
            }
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "List the collections in the library, with how many titles each lists, as of the last time Libri loaded them")]
    pub(crate) async fn list_collections(&self) -> String {
        if let Some(denied) = self.gate(McpTool::ListCollections, "") {
            return denied;
        }
        match LibraryIndex::open_and_migrate(self.api.index_root()).and_then(|index| {
                                                                       index.list_collections()
                                                                   }) {
            Ok(collections) => {
                let collections: Vec<CollectionInfo> =
                    collections.iter().map(CollectionInfo::from).collect();
                serde_json::to_string(&collections).unwrap_or_default()
            }
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Get figures about the whole library: titles, size, pages, how many are downloaded, publishers, systems, titles per publisher, system and type, disk use per publisher, and titles added per month")]
    pub(crate) async fn get_library_stats(&self) -> String {
        if let Some(denied) = self.gate(McpTool::GetLibraryStats, "") {
            return denied;
        }
        match self.api.index_list_items() {
            Ok(items) => {
                let insights = Insights::compute(&items, now_secs());
                serde_json::to_string(&stats_json(&insights)).unwrap_or_default()
            }
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "List entries in a local archive file")]
    pub(crate) async fn list_archive_entries(&self,
                                             Parameters(ListArchiveEntriesRequest { path }): Parameters<ListArchiveEntriesRequest>)
                                             -> String {
        if let Some(denied) = self.gate(McpTool::ListArchiveEntries, &path) {
            return denied;
        }
        match self.api.list_archive_entries(&PathBuf::from(&path)) {
            Ok(entries) => serde_json::to_string(&entries).unwrap_or_default(),
            Err(err) => error_json(&err.to_string()),
        }
    }
}

/// The aggregates as the JSON `get_library_stats` replies with.
fn stats_json(insights: &Insights) -> serde_json::Value {
    let counts = |counts: &[dtrpg_core::library::insights::Count]| {
        counts.iter()
              .map(|entry| serde_json::json!({ "name": entry.label, "titles": entry.count }))
              .collect::<Vec<_>>()
    };
    serde_json::json!({
        "titles": insights.titles,
        "total_size_mb": insights.total_size_mb,
        "pages": insights.total_pages,
        "downloaded": insights.downloaded,
        "publishers": insights.publishers,
        "systems": insights.systems,
        "added_this_month": insights.added_this_month,
        "by_publisher": counts(&insights.by_publisher),
        "by_system": counts(&insights.by_system),
        "by_type": counts(&insights.by_type),
        "disk_mb_by_publisher": insights.disk_by_publisher.iter()
            .map(|share| serde_json::json!({ "name": share.label, "mb": share.mb }))
            .collect::<Vec<_>>(),
        "added_by_month": insights.added_by_month.iter()
            .map(|month| serde_json::json!({
                "month": format!("{}-{:02}", month.year, month.month),
                "titles": month.count,
            }))
            .collect::<Vec<_>>(),
    })
}
