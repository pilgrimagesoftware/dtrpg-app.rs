//! Tests that the app does what an agent asks for, and says no when it
//! cannot. The socket and the settings check are tested where they live, in
//! `dtrpg-core` and `libri-mcp`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dtrpg_core::data::collection::entry::CollectionEntry;
use dtrpg_core::data::mcp_access::AppReply;
use gpui::TestAppContext;

use super::super::agent_requests::created_reply;
use super::super::*;
use super::support::*;

/// The id of the collection the fixture holds.
const COLLECTION_ID: u64 = 7;

/// A controller holding the items `first` and `second`, and a collection
/// named "Solo Play" that has no members.
fn setup(cx: &mut TestAppContext) -> Fixture {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               vec![item_with_files("first", 11, 1),
                                    item_with_files("second", 12, 1)]);
    fixture.controller.update(cx, |ctrl, _| {
                          ctrl.library
                              .push_collection(CollectionEntry { id:         COLLECTION_ID,
                                                                 name:       "Solo Play".into(),
                                                                 member_ids: Arc::from([]),
                                                                 created_at: 0, });
                      });
    fixture
}

/// What `reply` tells the agent, or the reason it was refused.
fn failure(reply: &AppReply) -> Option<&str> {
    match reply {
        AppReply::Failed { message } => Some(message),
        AppReply::Done { .. } => None,
    }
}

#[gpui::test]
fn an_item_is_added_to_a_collection_found_by_name_ignoring_case(cx: &mut TestAppContext) {
    let fixture = setup(cx);

    let (reply, members) =
        fixture.controller.update(cx, |ctrl, cx| {
                              let reply = ctrl.agent_add_to_collection("solo PLAY", "first", cx);
                              let members = ctrl.collections()[0].member_ids.clone();
                              (reply, members)
                          });

    assert!(failure(&reply).is_none(), "{reply:?}");
    assert!(members.contains(&11));
}

#[gpui::test]
fn an_item_is_added_to_a_collection_found_by_id(cx: &mut TestAppContext) {
    let fixture = setup(cx);

    let reply =
        fixture.controller.update(cx, |ctrl, cx| {
                              ctrl.agent_add_to_collection(&COLLECTION_ID.to_string(), "second", cx)
                          });

    assert!(failure(&reply).is_none(), "{reply:?}");
}

#[gpui::test]
fn adding_to_a_collection_that_is_not_there_or_an_item_that_is_not_there_fails(cx: &mut TestAppContext)
{
    let fixture = setup(cx);

    let (no_collection, no_item) =
        fixture.controller.update(cx, |ctrl, cx| {
                              (ctrl.agent_add_to_collection("Nothing", "first", cx),
                               ctrl.agent_add_to_collection("Solo Play", "missing", cx))
                          });

    assert!(failure(&no_collection).unwrap().contains("no collection"));
    assert!(failure(&no_item).unwrap().contains("no item"));
}

#[gpui::test]
fn downloading_an_item_that_is_not_there_fails(cx: &mut TestAppContext) {
    let fixture = setup(cx);

    let reply = fixture.controller
                       .update(cx, |ctrl, cx| ctrl.agent_download("missing", cx));

    assert!(failure(&reply).unwrap().contains("no item"));
}

#[test]
fn a_created_collection_is_reported_with_its_id_and_name() {
    let entry = CollectionEntry { id:         9,
                                  name:       "Horror".into(),
                                  member_ids: Arc::from([]),
                                  created_at: 0, };

    let reply = created_reply(Ok(entry));
    let refused = created_reply(Err("no".to_owned()));

    assert_eq!(reply,
               AppReply::Done { data: serde_json::json!({ "id": 9, "name": "Horror" }), });
    assert_eq!(failure(&refused), Some("no"));
}
