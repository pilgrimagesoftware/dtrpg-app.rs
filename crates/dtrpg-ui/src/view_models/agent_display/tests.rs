use super::*;

fn entry(summary: &str, denied: bool) -> AuditEntry {
    AuditEntry { time_secs: 1_700_000_000,
                 tool: "get_item".to_owned(),
                 summary: summary.to_owned(),
                 scope: McpScope::Read,
                 denied }
}

#[test]
fn nothing_is_exposed_until_access_is_on_and_then_only_the_read_tools() {
    let mut settings = McpSettingsFile::default();
    assert_eq!(exposed_tools(&settings), 0);

    settings.enabled = true;
    let read_tools = McpTool::ALL.iter()
                                 .filter(|tool| tool.scope() == McpScope::Read)
                                 .count();
    assert_eq!(exposed_tools(&settings), read_tools);

    settings.change = true;
    settings.download_and_spend = true;
    assert_eq!(exposed_tools(&settings), McpTool::ALL.len());
}

#[test]
fn a_tool_switched_off_is_not_exposed() {
    let mut settings = McpSettingsFile { enabled: true,
                                         ..McpSettingsFile::default() };
    let before = exposed_tools(&settings);

    settings.disabled_tools.insert("get_item".to_owned());

    assert_eq!(exposed_tools(&settings), before - 1);
}

#[test]
fn every_tool_and_scope_has_text() {
    for tool in McpTool::ALL {
        let key = format!("agent.tool_{}", tool.name());
        assert_ne!(tool_description(tool), key, "{key} has no text");
    }
    for scope in McpScope::ALL {
        assert_ne!(scope_label(scope), "agent.scope_read");
    }
}

#[test]
fn a_refused_call_says_so() {
    assert!(summary_label(&entry("a", true)).starts_with("a ("));
    assert_eq!(summary_label(&entry("a", false)), "a");
    assert_eq!(summary_label(&entry("", false)), "");
}

#[test]
fn the_config_names_the_server_and_its_braces_match() {
    assert!(AGENT_CONFIG_JSON.contains("\"command\": \"libri-mcp\""));
    assert_eq!(AGENT_CONFIG_JSON.matches('{').count(),
               AGENT_CONFIG_JSON.matches('}').count());
}
