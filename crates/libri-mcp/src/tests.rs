#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;

use dtrpg_core::api::LibraryApi;
use dtrpg_core::data::enums::ItemStatus;
use dtrpg_core::data::index::LibraryIndex;
use dtrpg_core::data::library::LibraryItem;
use dtrpg_core::data::mcp_access::{
    AppReply, AuditLog, IpcEndpoint, IpcListener, McpScope, McpSettings, McpTool,
};
use dtrpg_core::data::summary::ContentSummary;
use dtrpg_core::services::sdk::{SdkCollectionsService, SdkLibraryService};
use rmcp::handler::server::wrapper::Parameters;

use crate::requests::{
    AddToCollectionRequest, CreateCollectionRequest, GetItemRequest, SearchLibraryRequest,
    SetTagColorRequest, SetTagsRequest,
};
use crate::server::{Access, LibriMcpServer, error_json};

/// A server over a library index in a fresh temporary folder, holding items
/// `a` and `b`, with the folder to remove afterwards. Agent access is on, with
/// the change and the download and spend scopes on too when `allow_write` is
/// true, and only the read scope when it is false.
fn server(allow_write: bool, name: &str) -> (LibriMcpServer, PathBuf) {
    server_with(name, |settings| {
        settings.save_enabled(true);
        settings.save_scope(McpScope::Change, allow_write);
        settings.save_scope(McpScope::DownloadAndSpend, allow_write);
    })
}

/// As [`server`], with the settings set by `configure`.
fn server_with(name: &str, configure: impl FnOnce(&mut McpSettings)) -> (LibriMcpServer, PathBuf) {
    let dir = std::env::temp_dir().join(format!("libri_mcp_test_{}_{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let item = |id: &str| {
        LibraryItem::new(id,
                         "Title",
                         "Free League",
                         "",
                         "Core",
                         "PDF",
                         100,
                         10.0,
                         2024,
                         0,
                         ItemStatus::Cloud,
                         "#1C2A44",
                         "A game.",
                         None)
    };
    LibraryIndex::open_and_migrate(&dir).unwrap()
                                        .replace_all_items(&[item("a"), item("b")])
                                        .unwrap();
    let settings_path = dir.join("mcp.toml");
    configure(&mut McpSettings::load_from(&settings_path));
    let api = LibraryApi::new(Box::new(SdkLibraryService::unauthenticated()),
                              Box::new(SdkCollectionsService::unauthenticated()),
                              dir.clone());
    let access = Access::at(settings_path,
                            dir.join("audit.jsonl"),
                            IpcEndpoint::at(&dir));
    (LibriMcpServer::new(api, access), dir)
}

/// The audit log of the server over `dir`, newest first.
fn audit(dir: &std::path::Path) -> Vec<dtrpg_core::data::mcp_access::AuditEntry> {
    AuditLog::at(dir.join("audit.jsonl")).recent(50)
}

fn json(reply: &str) -> serde_json::Value {
    serde_json::from_str(reply).unwrap()
}

fn set_tags(id: &str, tags: &[&str]) -> Parameters<SetTagsRequest> {
    Parameters(SetTagsRequest { id:   id.to_owned(),
                                tags: tags.iter().map(|t| (*t).to_owned()).collect(), })
}

fn id(id: &str) -> Parameters<GetItemRequest> {
    Parameters(GetItemRequest { id: id.to_owned() })
}

fn color(tag: &str, color: Option<&str>) -> Parameters<SetTagColorRequest> {
    Parameters(SetTagColorRequest { tag:   tag.to_owned(),
                                    color: color.map(str::to_owned), })
}

#[tokio::test]
async fn the_tools_that_read_list_tags_with_colors_and_counts() {
    let (server, dir) = server(true, "read");
    server.set_tags(set_tags("a", &["solo", "to run"])).await;
    server.set_tags(set_tags("b", &["solo"])).await;
    server.set_tag_color(color("solo", Some("red"))).await;

    let tags = json(&server.list_tags().await);
    assert_eq!(tags[0],
               serde_json::json!({ "name": "solo", "color": "red", "count": 2 }));
    assert_eq!(tags[1],
               serde_json::json!({ "name": "to run", "color": null, "count": 1 }));

    let of_a = json(&server.get_item_tags(id("a")).await);
    assert_eq!(of_a,
               serde_json::json!([{ "name": "solo", "color": "red" },
                                  { "name": "to run", "color": null }]));
    assert_eq!(json(&server.get_item_tags(id("missing")).await),
               serde_json::json!([]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn the_tools_that_change_the_library_refuse_unless_the_change_scope_is_on() {
    let (server, dir) = server(false, "refuse");

    let tagged = json(&server.set_tags(set_tags("a", &["solo"])).await);
    let colored = json(&server.set_tag_color(color("solo", Some("red"))).await);

    assert!(tagged["error"].as_str()
                           .unwrap()
                           .contains("scope is switched off"));
    assert!(colored["error"].as_str()
                            .unwrap()
                            .contains("scope is switched off"));
    assert_eq!(json(&server.list_tags().await), serde_json::json!([]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn setting_tags_replaces_the_items_tags_and_keeps_a_colors_that_stays() {
    let (server, dir) = server(true, "replace");
    server.set_tags(set_tags("a", &["solo", "old"])).await;
    server.set_tag_color(color("solo", Some("blue"))).await;

    let reply = json(&server.set_tags(set_tags("a", &["Solo", "new"])).await);

    assert_eq!(reply, serde_json::json!({ "ok": true }));
    let tags = json(&server.get_item_tags(id("a")).await);
    assert_eq!(tags,
               serde_json::json!([{ "name": "new", "color": null },
                                  { "name": "solo", "color": "blue" }]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_tag_that_cannot_be_stored_or_an_unknown_item_is_an_error_and_changes_nothing() {
    let (server, dir) = server(true, "invalid");
    server.set_tags(set_tags("a", &["solo"])).await;

    let comma = json(&server.set_tags(set_tags("a", &["no, commas"])).await);
    let missing = json(&server.set_tags(set_tags("missing", &["solo"])).await);

    assert!(comma["error"].is_string());
    assert!(missing["error"].is_string());
    assert_eq!(json(&server.get_item_tags(id("a")).await),
               serde_json::json!([{ "name": "solo", "color": null }]));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_color_is_set_cleared_and_checked() {
    let (server, dir) = server(true, "colors");
    server.set_tags(set_tags("a", &["solo"])).await;

    assert_eq!(json(&server.set_tag_color(color("SOLO", Some("green"))).await),
               serde_json::json!({ "ok": true }));
    assert_eq!(json(&server.list_tags().await)[0]["color"], "green");

    assert_eq!(json(&server.set_tag_color(color("solo", None)).await),
               serde_json::json!({ "ok": true }));
    assert!(json(&server.list_tags().await)[0]["color"].is_null());

    let bad = json(&server.set_tag_color(color("solo", Some("teal"))).await);
    assert!(bad["error"].as_str().unwrap().contains("teal"));
    let unknown = json(&server.set_tag_color(color("nothing", Some("red"))).await);
    assert!(unknown["error"].is_string());
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn an_item_with_no_summary_reports_null_and_one_with_a_summary_reports_it() {
    let (server, dir) = server(true, "summary");
    assert_eq!(json(&server.get_summary(id("a")).await),
               serde_json::Value::Null);
    server.api
          .index_save_summary(&ContentSummary { item_id:     "a".to_owned(),
                                                text:        "A haunted lighthouse.".to_owned(),
                                                model:       "local-extractive".to_owned(),
                                                created_at:  1_700_000_000,
                                                token_count: 3, })
          .unwrap();

    let reply = json(&server.get_summary(id("a")).await);

    assert_eq!(reply,
               serde_json::json!({ "text": "A haunted lighthouse.",
                                   "model": "local-extractive",
                                   "created_at": 1_700_000_000 }));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn summarizing_refuses_unless_the_download_and_spend_scope_is_on() {
    let (server, dir) = server(false, "summarize_refuse");

    let reply = json(&server.summarize_item(id("a")).await);

    assert!(reply["error"].as_str()
                          .unwrap()
                          .contains("scope is switched off"));
    assert_eq!(json(&server.get_summary(id("a")).await),
               serde_json::Value::Null);
    let _ = std::fs::remove_dir_all(&dir);
}

fn create_collection(name: &str) -> Parameters<CreateCollectionRequest> {
    Parameters(CreateCollectionRequest { name: name.to_owned(), })
}

fn add_to_collection(collection: &str, id: &str) -> Parameters<AddToCollectionRequest> {
    Parameters(AddToCollectionRequest { collection: collection.to_owned(),
                                        id:         id.to_owned(), })
}

#[tokio::test]
async fn the_tools_that_need_the_app_refuse_unless_their_scope_is_on() {
    let (server, dir) = server(false, "app_refuse");

    let replies = [json(&server.create_collection(create_collection("Solo")).await),
                   json(&server.add_to_collection(add_to_collection("Solo", "a"))
                               .await),
                   json(&server.download_item(id("a")).await)];

    for reply in replies {
        assert!(reply["error"].as_str()
                              .unwrap()
                              .contains("scope is switched off"));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn the_tools_that_need_the_app_say_so_when_it_is_not_running() {
    let (server, dir) = server(true, "app_absent");

    let reply = json(&server.download_item(id("a")).await);

    assert!(reply["error"].as_str().unwrap().contains("not running"));
    let log = audit(&dir);
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].tool, "download_item");
    assert!(!log[0].denied);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn the_tools_that_need_the_app_reply_with_what_the_app_answers() {
    let (server, dir) = server(true, "app_answers");
    let (requests, incoming) = std::sync::mpsc::channel();
    let _listener = IpcListener::start(&IpcEndpoint::at(&dir), requests).unwrap();
    std::thread::spawn(move || {
        while let Ok(request) = incoming.recv() {
            let data = serde_json::json!({ "asked": request.request.tool().name() });
            request.reply.send(AppReply::Done { data }).ok();
        }
    });

    let reply = json(&server.create_collection(create_collection("Solo")).await);

    assert_eq!(reply, serde_json::json!({ "asked": "create_collection" }));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_error_is_valid_json_whatever_the_message_holds() {
    let reply = error_json("a \"quoted\" message\nwith a newline");

    assert_eq!(json(&reply)["error"],
               "a \"quoted\" message\nwith a newline");
}

#[tokio::test]
async fn with_access_off_every_tool_is_refused_and_logged() {
    let (server, dir) = server_with("off", |_| {});

    let listed = json(&server.list_items().await);
    let tagged = json(&server.set_tags(set_tags("a", &["solo"])).await);

    assert!(listed["error"].as_str().unwrap().contains("switched off"));
    assert!(tagged["error"].as_str().unwrap().contains("switched off"));
    let log = audit(&dir);
    assert_eq!(log.len(), 2);
    assert!(log.iter().all(|entry| entry.denied));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_call_that_goes_ahead_is_logged_with_its_tool_scope_and_summary() {
    let (server, dir) = server(false, "log_allowed");

    server.get_item(id("a")).await;
    server.set_tags(set_tags("a", &["solo"])).await;

    let log = audit(&dir);
    assert_eq!(log[1].tool, "get_item");
    assert_eq!(log[1].summary, "a");
    assert_eq!(log[1].scope, McpScope::Read);
    assert!(!log[1].denied);
    assert_eq!(log[0].tool, "set_tags");
    assert_eq!(log[0].scope, McpScope::Change);
    assert!(log[0].denied, "the change scope is off");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_tool_switched_off_by_itself_is_refused_inside_a_scope_that_is_on() {
    let (server, dir) = server_with("tool_off", |settings| {
        settings.save_enabled(true);
        settings.save_tool(McpTool::GetSummary, false);
    });

    let summary = json(&server.get_summary(id("a")).await);
    let item = json(&server.get_item(id("a")).await);

    assert!(summary["error"].as_str()
                            .unwrap()
                            .contains("tool is switched off"));
    assert_eq!(item["id"], "a");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn a_change_in_the_settings_applies_to_the_next_call() {
    let (server, dir) = server(false, "live");
    assert!(json(&server.set_tags(set_tags("a", &["solo"])).await)["error"].is_string());

    McpSettings::load_from(&dir.join("mcp.toml")).save_scope(McpScope::Change, true);

    assert_eq!(json(&server.set_tags(set_tags("a", &["solo"])).await),
               serde_json::json!({ "ok": true }));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn searching_uses_the_search_box_query_language() {
    let (server, dir) = server(false, "search");

    let by_publisher = json(&server.search_library(Parameters(SearchLibraryRequest {
                                                       query: "publisher:\"free league\"".to_owned(),
                                                   })).await);
    let none = json(&server.search_library(Parameters(SearchLibraryRequest {
                                                query: "publisher:paizo".to_owned(),
                                            })).await);

    assert_eq!(by_publisher.as_array().unwrap().len(), 2);
    assert!(none.as_array().unwrap().is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn the_library_stats_add_up_the_index() {
    let (server, dir) = server(false, "stats");

    let stats = json(&server.get_library_stats().await);

    assert_eq!(stats["titles"], 2);
    assert_eq!(stats["publishers"], 1);
    assert_eq!(stats["by_publisher"][0]["titles"], 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn collections_list_empty_before_any_have_been_loaded() {
    let (server, dir) = server(false, "collections");

    assert_eq!(json(&server.list_collections().await),
               serde_json::json!([]));
    let _ = std::fs::remove_dir_all(&dir);
}
