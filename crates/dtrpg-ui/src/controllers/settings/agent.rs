//! The Agent & MCP page's switches and the log of what agents did.

use dtrpg_core::data::mcp_access::{McpScope, McpTool};

use super::*;
use crate::view_models::agent_display::locate_mcp_server;

/// The index of the Agent & MCP page in the settings window.
pub const AGENT_PAGE_IX: usize = 4;

/// How many of the latest calls the page lists.
pub const MCP_ACTIVITY_ROWS: usize = 10;

impl SettingsController {
    /// Saves whether agents may use Libri at all.
    pub fn set_mcp_enabled(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.mcp.save_enabled(enabled);
        cx.notify();
    }

    /// Saves whether `scope` is on.
    pub fn set_mcp_scope(&mut self, scope: McpScope, enabled: bool, cx: &mut Context<Self>) {
        self.mcp.save_scope(scope, enabled);
        cx.notify();
    }

    /// Saves whether `tool` is on by itself.
    pub fn set_mcp_tool(&mut self, tool: McpTool, enabled: bool, cx: &mut Context<Self>) {
        self.mcp.save_tool(tool, enabled);
        cx.notify();
    }

    /// Reads the latest calls from the log. The server writes the log from
    /// another process, so the page asks for it when it opens and on request,
    /// never while drawing.
    pub fn refresh_mcp_activity(&mut self, cx: &mut Context<Self>) {
        self.mcp_activity = AuditLog::open().recent(MCP_ACTIVITY_ROWS);
        cx.notify();
    }
}

/// The full path of the MCP server program: the one next to the running app.
pub(super) fn mcp_command_path() -> String {
    std::env::current_exe().ok()
                           .and_then(|exe| exe.parent().map(locate_mcp_server))
                           .map_or_else(String::new, |path| path.display().to_string())
}
