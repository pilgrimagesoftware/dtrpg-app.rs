//! The server's state, and the gate every tool call passes through.

use std::path::PathBuf;

use dtrpg_core::api::LibraryApi;
use dtrpg_core::data::mcp_access::{
    AppReply, AppRequest, AuditEntry, AuditLog, IpcEndpoint, McpSettings, McpTool, send_request,
};
use dtrpg_core::util::datetime::now_secs;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{Implementation, InitializeResult, ProtocolVersion, ServerCapabilities};
use rmcp::{ServerHandler, tool_handler};

/// The longest summary the audit log keeps of a call, in characters.
const MAX_SUMMARY_CHARS: usize = 80;

/// Where the server reads what an agent may do, and where it records what the
/// agent did.
pub struct Access {
    /// The settings file, or `None` for the one in the app's preferences
    /// folder.
    settings_path: Option<PathBuf>,
    audit:         AuditLog,
    /// Where the running app listens for the requests only it can carry out.
    endpoint:      IpcEndpoint,
}

impl Access {
    /// The settings and audit log in the app's own folders, which the app's
    /// Settings window edits and shows.
    pub fn production() -> Self {
        Self { settings_path: None,
               audit:         AuditLog::open(),
               endpoint:      IpcEndpoint::production(), }
    }

    /// Settings read from `settings_path` and a log written to `audit_path`.
    #[cfg(test)]
    pub fn at(settings_path: PathBuf, audit_path: PathBuf, endpoint: IpcEndpoint) -> Self {
        Self { settings_path: Some(settings_path),
               audit: AuditLog::at(audit_path),
               endpoint }
    }

    /// The settings as they are on disk now. They are read on every call, so a
    /// change in Libri's settings applies to the very next call.
    fn settings(&self) -> McpSettings {
        self.settings_path
            .as_deref()
            .map_or_else(McpSettings::load, McpSettings::load_from)
    }
}

pub struct LibriMcpServer {
    pub(crate) api: LibraryApi,
    access:         Access,
    tool_router:    ToolRouter<Self>,
}

impl LibriMcpServer {
    pub fn new(api: LibraryApi, access: Access) -> Self {
        Self { api,
               access,
               tool_router: Self::read_router() + Self::change_router() }
    }

    /// Lets the call of `tool` go ahead, or refuses it. Either way the call is
    /// recorded in the audit log, with `summary` (cut short) saying what it was
    /// about. Returns the reply to send back when the call is refused.
    ///
    /// `summary` must never hold credentials; the tools pass ids, names and
    /// search text only.
    pub(crate) fn gate(&self, tool: McpTool, summary: &str) -> Option<String> {
        let denial = self.access.settings().check(tool).err();
        self.access
            .audit
            .append(&AuditEntry { time_secs: now_secs(),
                                  tool:      tool.name().to_owned(),
                                  summary:   summary.chars().take(MAX_SUMMARY_CHARS).collect(),
                                  scope:     tool.scope(),
                                  denied:    denial.is_some(), });
        denial.map(|denial| error_json(&denial.message()))
    }
}

impl LibriMcpServer {
    /// Asks the running app to carry out `request` and returns what the tool
    /// replies with. The app does the work because it holds the person's
    /// session; this server holds no credentials.
    pub(crate) async fn ask_app(&self, request: AppRequest) -> String {
        match send_request(&self.access.endpoint, request).await {
            Ok(AppReply::Done { data }) => data.to_string(),
            Ok(AppReply::Failed { message }) => error_json(&message),
            Err(error) => error_json(&error.to_string()),
        }
    }
}

/// An error as the JSON a tool replies with, which is valid whatever the
/// message contains.
pub fn error_json(message: &str) -> String {
    serde_json::json!({ "error": message }).to_string()
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for LibriMcpServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::LATEST)
            .with_server_info(Implementation::new("libri-mcp".to_string(),
                                                  env!("CARGO_PKG_VERSION").to_string()))
    }
}
