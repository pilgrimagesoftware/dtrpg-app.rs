use std::future::Future;
use std::sync::mpsc;
use std::time::Duration;

use super::*;

/// Answers each request that arrives on `requests` with its tool's name.
fn answer_in_background(requests: mpsc::Receiver<Incoming>) {
    std::thread::spawn(move || {
        while let Ok(incoming) = requests.recv() {
            let data = serde_json::json!({ "tool": incoming.request.tool().name() });
            incoming.reply.send(AppReply::Done { data }).ok();
        }
    });
}

fn run<T>(future: impl Future<Output = T>) -> T {
    crate::runtime::handle().unwrap().block_on(future)
}

#[test]
fn a_request_is_answered_by_the_app() {
    let folder = tempfile::tempdir().unwrap();
    let endpoint = IpcEndpoint::at(folder.path());
    let (sender, receiver) = mpsc::channel();
    let _listener = IpcListener::start(&endpoint, sender).unwrap();
    answer_in_background(receiver);

    let reply = run(send_request(&endpoint,
                                 AppRequest::CreateCollection { name: "Solo".to_owned(), }));

    assert_eq!(reply.unwrap(),
               AppReply::Done { data: serde_json::json!({ "tool": "create_collection" }), });
}

#[test]
fn a_request_with_the_wrong_token_is_refused() {
    let folder = tempfile::tempdir().unwrap();
    let endpoint = IpcEndpoint::at(folder.path());
    let (sender, receiver) = mpsc::channel();
    let _listener = IpcListener::start(&endpoint, sender).unwrap();
    answer_in_background(receiver);
    std::fs::write(endpoint.token_path(), "not the token").unwrap();

    let reply = run(send_request(&endpoint,
                                 AppRequest::DownloadItem { item_id: "1".to_owned(), }));

    assert!(matches!(reply.unwrap(), AppReply::Failed { .. }));
}

#[test]
fn a_rotated_token_works_at_once_and_the_old_one_stops_working() {
    let folder = tempfile::tempdir().unwrap();
    let endpoint = IpcEndpoint::at(folder.path());
    let (sender, receiver) = mpsc::channel();
    let listener = IpcListener::start(&endpoint, sender).unwrap();
    answer_in_background(receiver);
    let stale = std::fs::read_to_string(endpoint.token_path()).unwrap();

    let fresh = listener.rotate_token().unwrap();

    assert_ne!(fresh, stale);
    assert_eq!(std::fs::read_to_string(endpoint.token_path()).unwrap(),
               fresh);
    // The agent reads the token file on every request, so it keeps working.
    let reply = run(send_request(&endpoint,
                                 AppRequest::CreateCollection { name: "Solo".to_owned(), }));
    assert!(matches!(reply.unwrap(), AppReply::Done { .. }));
    // A copy of the old token, still in hand somewhere, no longer answers.
    std::fs::write(endpoint.token_path(), &stale).unwrap();
    let stale_reply = run(send_request(&endpoint,
                                       AppRequest::CreateCollection { name: "Solo".to_owned(), }));
    assert!(matches!(stale_reply.unwrap(), AppReply::Failed { .. }));
}

#[test]
fn the_bind_display_names_where_the_socket_is() {
    let folder = tempfile::tempdir().unwrap();
    let endpoint = IpcEndpoint::at(folder.path());

    let display = endpoint.bind_display();

    #[cfg(unix)]
    assert!(display.ends_with("agent.sock"), "{display}");
    #[cfg(not(unix))]
    assert!(display.starts_with("libri-agent-"), "{display}");
}

#[test]
fn nothing_listening_means_the_app_is_not_running() {
    let folder = tempfile::tempdir().unwrap();
    let endpoint = IpcEndpoint::at(folder.path());

    let reply = run(send_request(&endpoint,
                                 AppRequest::DownloadItem { item_id: "1".to_owned(), }));

    assert!(matches!(reply, Err(IpcError::AppNotRunning)));
}

#[test]
fn the_listener_stops_when_it_is_dropped() {
    let folder = tempfile::tempdir().unwrap();
    let endpoint = IpcEndpoint::at(folder.path());
    let (sender, receiver) = mpsc::channel();
    let listener = IpcListener::start(&endpoint, sender).unwrap();
    answer_in_background(receiver);
    drop(listener);
    std::thread::sleep(Duration::from_millis(100));

    let reply = run(send_request(&endpoint,
                                 AppRequest::DownloadItem { item_id: "1".to_owned(), }));

    assert!(reply.is_err() || matches!(reply, Ok(AppReply::Failed { .. })));
}

#[test]
fn requests_name_the_tool_that_gates_them() {
    use crate::data::mcp_access::McpScope;

    let add = AppRequest::AddToCollection { collection: "1".to_owned(),
                                            item_id:    "2".to_owned(), };
    assert_eq!(add.tool().scope(), McpScope::Change);
    assert_eq!(AppRequest::DownloadItem { item_id: "2".to_owned(), }.tool()
                                                                    .scope(),
               McpScope::DownloadAndSpend);
}
