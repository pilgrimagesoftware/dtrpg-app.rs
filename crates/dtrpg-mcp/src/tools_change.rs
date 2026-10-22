//! The tools that change the library or spend tokens. Each asks the gate
//! first, which refuses it when its scope or the tool is switched off.

use dtrpg_core::api::provider_for;
use dtrpg_core::data::mcp_access::McpTool;
use dtrpg_core::data::storage::StorageConfig;
use dtrpg_core::data::summary_settings::SummarySettings;
use dtrpg_core::data::tag_color::TagColor;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};

use crate::requests::{GetItemRequest, SetTagColorRequest, SetTagsRequest, SummaryInfo};
use crate::server::{LibriMcpServer, error_json};

#[tool_router(router = change_router, vis = "pub(crate)")]
impl LibriMcpServer {
    #[tool(description = "Replace the tags of a library item. Changes the library; refused unless the change scope is on in Libri's settings. The new tags reach the item's files the next time Libri starts or the library is reloaded.")]
    pub(crate) async fn set_tags(&self,
                                 Parameters(SetTagsRequest { id, tags }): Parameters<SetTagsRequest>)
                                 -> String {
        if let Some(denied) = self.gate(McpTool::SetTags, &id) {
            return denied;
        }
        match self.api.index_set_item_tags(&id, &tags) {
            Ok(()) => serde_json::json!({ "ok": true }).to_string(),
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Set the color of a tag, or clear it. Changes the library; refused unless the change scope is on in Libri's settings.")]
    pub(crate) async fn set_tag_color(&self,
                                      Parameters(SetTagColorRequest { tag, color }): Parameters<SetTagColorRequest>)
                                      -> String {
        if let Some(denied) = self.gate(McpTool::SetTagColor, &tag) {
            return denied;
        }
        let color = match color.as_deref().map(str::trim) {
            None | Some("") | Some("none") => None,
            Some(name) => match name.parse::<TagColor>() {
                Ok(color) => Some(color),
                Err(err) => return error_json(&err.to_string()),
            },
        };
        match self.api.index_set_tag_color(&tag, color) {
            Ok(true) => serde_json::json!({ "ok": true }).to_string(),
            Ok(false) => error_json(&format!("no item has the tag {tag:?}")),
            Err(err) => error_json(&err.to_string()),
        }
    }

    #[tool(description = "Write and save a content summary of a downloaded library item, using the provider chosen in Libri's settings. Spends tokens against the monthly ceiling set there. Refused unless the download and spend scope is on in Libri's settings.")]
    pub(crate) async fn summarize_item(&self,
                                       Parameters(GetItemRequest { id }): Parameters<GetItemRequest>)
                                       -> String {
        if let Some(denied) = self.gate(McpTool::SummarizeItem, &id) {
            return denied;
        }
        let settings = SummarySettings::load().snapshot();
        let provider = match provider_for(&settings) {
            Ok(provider) => provider,
            Err(err) => return error_json(&err.to_string()),
        };
        let storage = StorageConfig::load();
        let api = &self.api;
        let summarized = tokio::task::block_in_place(|| {
            api.summarize_item(&id, &storage, &settings, &*provider)
        });
        match summarized {
            Ok(summary) => serde_json::to_string(&SummaryInfo::from(summary)).unwrap_or_default(),
            Err(err) => error_json(&err.to_string()),
        }
    }
}
