//! What the Agent & MCP page says, as plain data: which tools are exposed, the
//! text for each scope and tool, and the line for each logged call. No window
//! and no I/O, so it can be tested directly.

use std::path::{Path, PathBuf};

use dtrpg_core::data::mcp_access::{AuditEntry, McpScope, McpSettings, McpSettingsFile, McpTool};
use dtrpg_core::data::summary_settings::SummarySettingsFile;
use rust_i18n::t;

use crate::view_models::summary_display::SummaryPage;

/// The name of the MCP server program, as the build produces it.
const SERVER_PROGRAM: &str = "libri-mcp";

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
/// assert!(agent_config_json("/opt/libri/libri-mcp").contains("\"command\": \"/opt/libri/libri-mcp\""));
/// ```
#[must_use]
pub fn agent_config_json(command: &str) -> String {
    let command = command.replace('\\', "\\\\").replace('"', "\\\"");
    format!("{{\n  \"mcpServers\": {{\n    \"libri\": {{\n      \"command\": \"{command}\"\n    }}\n  }}\n}}")
}

/// An agent the Connect block can show the setup command for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentHarness {
    /// Anthropic's Claude Code.
    #[default]
    ClaudeCode,
    /// OpenAI's Codex.
    Codex,
    /// Google's Gemini CLI.
    Gemini,
    /// OpenCode.
    OpenCode,
    /// Any other agent: the JSON config, for its MCP settings file.
    Other,
}

impl AgentHarness {
    /// Every harness, in the order the selector lists them.
    pub const ALL: [Self; 5] = [Self::ClaudeCode,
                                Self::Codex,
                                Self::Gemini,
                                Self::OpenCode,
                                Self::Other];

    /// The label on the selector.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::ClaudeCode => t!("agent.harness_claude_code"),
            Self::Codex => t!("agent.harness_codex"),
            Self::Gemini => t!("agent.harness_gemini"),
            Self::OpenCode => t!("agent.harness_opencode"),
            Self::Other => t!("agent.harness_other"),
        }.to_string()
    }

    /// What to copy to connect this agent to the server at `command`: a
    /// terminal command, or the JSON config for [`Self::Other`].
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_ui::view_models::agent_display::AgentHarness;
    ///
    /// assert_eq!(AgentHarness::ClaudeCode.setup_text("/opt/libri/libri-mcp"),
    ///            "claude mcp add libri -- '/opt/libri/libri-mcp'");
    /// ```
    #[must_use]
    pub fn setup_text(self, command: &str) -> String {
        let quoted = shell_quote(command);
        match self {
            Self::ClaudeCode => format!("claude mcp add libri -- {quoted}"),
            Self::Codex => format!("codex mcp add libri -- {quoted}"),
            Self::Gemini => format!("gemini mcp add libri {quoted}"),
            Self::OpenCode => format!("opencode mcp add libri --global -- {quoted}"),
            Self::Other => agent_config_json(command),
        }
    }
}

/// `text` as one POSIX shell word: in single quotes, with any single quote in
/// it closed, escaped and reopened.
fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
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
    /// Where the agent's requests are listened for: the socket file on
    /// macOS and Linux, the named pipe on Windows.
    pub bind_display: String,
    /// Where the log of calls is kept.
    pub log_path:     String,
    /// Which agent the Connect block shows the setup command for.
    pub harness:      AgentHarness,
    /// How summaries are made.
    pub summaries:    SummarySettingsFile,
    /// What the Summaries card reads besides the settings.
    pub summary_page: SummaryPage,
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
