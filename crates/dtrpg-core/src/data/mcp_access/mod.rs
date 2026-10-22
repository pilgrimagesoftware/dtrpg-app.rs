//! What an AI agent may do through the MCP server: the tools, the scopes that
//! group them, the settings that switch them on, and the log of every call.
//!
//! The settings and the log are files, because the MCP server is a separate
//! process from the app: the app writes the settings and reads the log, the
//! server reads the settings on each call and appends to the log.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

mod audit;
mod settings;

pub use audit::{AuditEntry, AuditLog};
pub use settings::{Denial, McpSettings, McpSettingsFile};

/// A group of tools that an agent is trusted with as one, from the least to
/// the most that can go wrong.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum McpScope {
    /// Reads the library and changes nothing.
    Read,
    /// Changes the library: tags, collections.
    Change,
    /// Starts downloads or spends summary tokens.
    DownloadAndSpend,
}

impl McpScope {
    /// Every scope, lowest risk first.
    pub const ALL: [Self; 3] = [Self::Read, Self::Change, Self::DownloadAndSpend];

    /// The scope's name as it is stored and logged.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Change => "change",
            Self::DownloadAndSpend => "download_and_spend",
        }
    }
}

impl fmt::Display for McpScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.key())
    }
}

impl FromStr for McpScope {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL.into_iter()
                 .find(|scope| scope.key() == text)
                 .ok_or_else(|| format!("unknown scope {text:?}"))
    }
}

/// A tool the MCP server offers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum McpTool {
    /// `list_items`
    ListItems,
    /// `get_item`
    GetItem,
    /// `search_library`
    SearchLibrary,
    /// `list_tags`
    ListTags,
    /// `get_item_tags`
    GetItemTags,
    /// `get_summary`
    GetSummary,
    /// `list_archive_entries`
    ListArchiveEntries,
    /// `list_collections`
    ListCollections,
    /// `get_library_stats`
    GetLibraryStats,
    /// `set_tags`
    SetTags,
    /// `set_tag_color`
    SetTagColor,
    /// `create_collection`
    CreateCollection,
    /// `add_to_collection`
    AddToCollection,
    /// `summarize_item`
    SummarizeItem,
    /// `download_item`
    DownloadItem,
}

impl McpTool {
    /// Every tool, in the order the settings list them.
    pub const ALL: [Self; 15] = [Self::ListItems,
                                 Self::GetItem,
                                 Self::SearchLibrary,
                                 Self::ListTags,
                                 Self::GetItemTags,
                                 Self::GetSummary,
                                 Self::ListArchiveEntries,
                                 Self::ListCollections,
                                 Self::GetLibraryStats,
                                 Self::SetTags,
                                 Self::SetTagColor,
                                 Self::CreateCollection,
                                 Self::AddToCollection,
                                 Self::SummarizeItem,
                                 Self::DownloadItem];

    /// The tool's name as the agent calls it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ListItems => "list_items",
            Self::GetItem => "get_item",
            Self::SearchLibrary => "search_library",
            Self::ListTags => "list_tags",
            Self::GetItemTags => "get_item_tags",
            Self::GetSummary => "get_summary",
            Self::ListArchiveEntries => "list_archive_entries",
            Self::ListCollections => "list_collections",
            Self::GetLibraryStats => "get_library_stats",
            Self::SetTags => "set_tags",
            Self::SetTagColor => "set_tag_color",
            Self::CreateCollection => "create_collection",
            Self::AddToCollection => "add_to_collection",
            Self::SummarizeItem => "summarize_item",
            Self::DownloadItem => "download_item",
        }
    }

    /// The scope a call of this tool needs.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::data::mcp_access::{McpScope, McpTool};
    ///
    /// assert_eq!(McpTool::ListItems.scope(), McpScope::Read);
    /// assert_eq!(McpTool::SetTags.scope(), McpScope::Change);
    /// assert_eq!(McpTool::DownloadItem.scope(), McpScope::DownloadAndSpend);
    /// ```
    #[must_use]
    pub const fn scope(self) -> McpScope {
        match self {
            Self::ListItems
            | Self::GetItem
            | Self::SearchLibrary
            | Self::ListTags
            | Self::GetItemTags
            | Self::GetSummary
            | Self::ListArchiveEntries
            | Self::ListCollections
            | Self::GetLibraryStats => McpScope::Read,
            Self::SetTags | Self::SetTagColor | Self::CreateCollection | Self::AddToCollection => {
                McpScope::Change
            }
            Self::SummarizeItem | Self::DownloadItem => McpScope::DownloadAndSpend,
        }
    }
}

impl fmt::Display for McpTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for McpTool {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL.into_iter()
                 .find(|tool| tool.name() == text)
                 .ok_or_else(|| format!("unknown tool {text:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tool_name_round_trips() {
        for tool in McpTool::ALL {
            assert_eq!(tool.name().parse::<McpTool>(), Ok(tool));
        }
    }

    #[test]
    fn every_scope_key_round_trips() {
        for scope in McpScope::ALL {
            assert_eq!(scope.key().parse::<McpScope>(), Ok(scope));
        }
    }

    #[test]
    fn the_tools_that_change_things_are_not_in_the_read_scope() {
        for tool in [McpTool::SetTags,
                     McpTool::CreateCollection,
                     McpTool::DownloadItem]
        {
            assert_ne!(tool.scope(), McpScope::Read);
        }
    }
}
