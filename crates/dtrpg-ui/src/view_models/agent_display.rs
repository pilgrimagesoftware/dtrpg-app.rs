//! What the Agent & MCP page says, as plain data: which tools are exposed, the
//! text for each scope and tool, and the line for each logged call. No window
//! and no I/O, so it can be tested directly.

use std::path::{Path, PathBuf};

use dtrpg_core::data::mcp_access::{AuditEntry, McpScope, McpSettings, McpSettingsFile, McpTool};
use rust_i18n::t;

/// The name of the MCP server program, as the build produces it.
const SERVER_PROGRAM: &str = "dtrpg-mcp";

/// Where the MCP server program is, given the folder the app runs from: the
/// program next to the app. The server is built alongside the app, so it is
/// there in a build and in a release.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use dtrpg_ui::view_models::agent_display::locate_mcp_server;
///
/// let path = locate_mcp_server(Path::new("/opt/libri"));
/// assert!(path.starts_with("/opt/libri"));
/// ```
#[must_use]
pub fn locate_mcp_server(app_dir: &Path) -> PathBuf {
    app_dir.join(format!("{SERVER_PROGRAM}{}", std::env::consts::EXE_SUFFIX))
}

/// The config an agent's MCP settings need to start Libri's server, with
/// `command` as the full path of the server program.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::agent_display::agent_config_json;
///
/// assert!(agent_config_json("/opt/libri/dtrpg-mcp").contains("\"command\": \"/opt/libri/dtrpg-mcp\""));
/// ```
#[must_use]
pub fn agent_config_json(command: &str) -> String {
    let command = command.replace('\\', "\\\\").replace('"', "\\\"");
    format!("{{\n  \"mcpServers\": {{\n    \"libri\": {{\n      \"command\": \"{command}\"\n    }}\n  }}\n}}")
}

/// What the page shows about everything it lists: the settings, and the
/// latest calls, newest first.
#[derive(Clone, Debug, Default)]
pub struct AgentPageData {
    /// What agents may do.
    pub settings:     McpSettingsFile,
    /// The latest calls agents made.
    pub activity:     Vec<AuditEntry>,
    /// The full path of the MCP server program, for the agent's config.
    pub command_path: String,
    /// Where the log of calls is kept.
    pub log_path:     String,
}

/// How many tools a call can use right now: all of them when the scope and
/// the tool are on, none when agent access is off.
///
/// # Examples
///
/// ```
/// use dtrpg_core::data::mcp_access::McpSettingsFile;
/// use dtrpg_ui::view_models::agent_display::exposed_tools;
///
/// assert_eq!(exposed_tools(&McpSettingsFile::default()),
///            0,
///            "access starts off");
/// ```
#[must_use]
pub fn exposed_tools(settings: &McpSettingsFile) -> usize {
    let live = McpSettings::from_file(settings.clone());
    McpTool::ALL.into_iter()
                .filter(|tool| live.check(*tool).is_ok())
                .count()
}

/// The short label a scope carries in the tools list.
#[must_use]
pub fn scope_label(scope: McpScope) -> String {
    match scope {
        McpScope::Read => t!("agent.scope_read"),
        McpScope::Change => t!("agent.scope_change"),
        McpScope::DownloadAndSpend => t!("agent.scope_spend"),
    }.to_string()
}

/// What `tool` does, in a few words.
#[must_use]
pub fn tool_description(tool: McpTool) -> String {
    let key = format!("agent.tool_{}", tool.name());
    t!(&key).to_string()
}

/// The time of a logged call as `HH:MM:SS`, in the local time zone.
#[must_use]
pub fn time_label(entry: &AuditEntry) -> String {
    chrono::DateTime::from_timestamp(entry.time_secs, 0).map_or_else(String::new, |time| {
                                                            time.with_timezone(&chrono::Local)
                                                                .format("%H:%M:%S")
                                                                .to_string()
                                                        })
}

/// What a logged call was about, with a note when it was refused.
#[must_use]
pub fn summary_label(entry: &AuditEntry) -> String {
    match (entry.summary.is_empty(), entry.denied) {
        (true, false) => String::new(),
        (false, false) => entry.summary.clone(),
        (true, true) => format!("({})", t!("agent.denied")),
        (false, true) => format!("{} ({})", entry.summary, t!("agent.denied")),
    }
}

#[cfg(test)]
mod tests;
