//! What the server asks of the app, and what the app answers.

use serde::{Deserialize, Serialize};

use super::super::McpTool;

/// Something the app does for an agent.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AppRequest {
    /// Makes a collection with this name.
    CreateCollection {
        /// The new collection's name.
        name: String,
    },
    /// Puts an item in a collection.
    AddToCollection {
        /// The collection's id, or its name, ignoring case.
        collection: String,
        /// The item's id, as `list_items` gives it.
        item_id:    String,
    },
    /// Starts downloading all of an item's files that are not downloaded yet.
    DownloadItem {
        /// The item's id, as `list_items` gives it.
        item_id: String,
    },
}

impl AppRequest {
    /// The tool this request is made for, which decides the scope it needs.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::data::mcp_access::{AppRequest, McpTool};
    ///
    /// let request = AppRequest::DownloadItem { item_id: "7".to_owned(), };
    /// assert_eq!(request.tool(), McpTool::DownloadItem);
    /// ```
    #[must_use]
    pub const fn tool(&self) -> McpTool {
        match self {
            Self::CreateCollection { .. } => McpTool::CreateCollection,
            Self::AddToCollection { .. } => McpTool::AddToCollection,
            Self::DownloadItem { .. } => McpTool::DownloadItem,
        }
    }
}

/// The app's answer to an [`AppRequest`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AppReply {
    /// It was done, or started. `data` is what the tool tells the agent.
    Done {
        /// The result, as JSON.
        data: serde_json::Value,
    },
    /// It was refused or failed. `message` says why, for the agent.
    Failed {
        /// Why, in plain words.
        message: String,
    },
}

impl AppReply {
    /// A failure with `message`.
    #[must_use]
    pub fn failed(message: impl Into<String>) -> Self {
        Self::Failed { message: message.into(), }
    }
}

/// A request with the token that proves who sent it, as it goes over the
/// socket: one line of JSON.
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Envelope {
    /// The token from the endpoint's token file.
    pub token:   String,
    /// What is asked.
    pub request: AppRequest,
}
