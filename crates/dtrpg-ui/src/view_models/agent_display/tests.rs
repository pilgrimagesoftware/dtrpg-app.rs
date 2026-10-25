use std::path::Path;

use rust_i18n::t;

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

/// The Agent & MCP page's own strings: every `t!` key the page shows.
const PAGE_KEYS: [&str; 8] = ["agent.rotate_token",
                              "agent.rotate_note",
                              "agent.transport_title",
                              "agent.transport_value",
                              "agent.transport_note",
                              "agent.bind_title",
                              "agent.bind_note",
                              "agent.open_log"];

#[test]
fn the_page_strings_resolve_rather_than_falling_back_to_their_key() {
    for key in PAGE_KEYS {
        assert_ne!(t!(key).to_string(), key.to_string(), "{key} has no text");
    }
}

#[test]
fn the_page_strings_are_written_in_every_locale() {
    // A missing de/fr string falls back to English at runtime; the stub
    // locales are meant to carry real text, so check the catalogues on disk.
    for locale in ["en", "de", "fr"] {
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n")
                                                                   .join(format!("{locale}.yaml"));
        let text = std::fs::read_to_string(&file)
                       .unwrap_or_else(|error| panic!("could not read {locale}.yaml: {error}"));
        for key in PAGE_KEYS {
            let suffix = key.rsplit('.')
                            .next()
                            .unwrap_or_else(|| panic!("malformed key {key}"));
            assert!(text.contains(&format!("  {suffix}:")),
                    "{locale}.yaml lacks {key}");
        }
    }
}

#[test]
fn a_refused_call_says_so() {
    assert!(summary_label(&entry("a", true)).starts_with("a ("));
    assert_eq!(summary_label(&entry("a", false)), "a");
    assert_eq!(summary_label(&entry("", false)), "");
}

#[test]
fn the_config_names_the_server_by_its_full_path() {
    let config = agent_config_json("/opt/libri/libri-mcp");

    assert!(config.contains("\"command\": \"/opt/libri/libri-mcp\""));
    assert_eq!(config.matches('{').count(), config.matches('}').count());
}

#[test]
fn a_path_with_backslashes_or_quotes_is_escaped() {
    let config = agent_config_json(r#"C:\Libri\"x"\libri-mcp.exe"#);

    assert!(config.contains(r#""command": "C:\\Libri\\\"x\"\\libri-mcp.exe""#));
}

#[test]
fn the_server_is_looked_for_next_to_the_app() {
    let path = locate_mcp_server(Path::new("/opt/libri"));

    assert!(path.starts_with("/opt/libri"));
    assert_eq!(path.parent(), Some(Path::new("/opt/libri")));
    assert!(path.to_string_lossy().contains("libri-mcp"));
}
