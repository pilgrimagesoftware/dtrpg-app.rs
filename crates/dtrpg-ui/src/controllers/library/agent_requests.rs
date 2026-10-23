//! Doing what an AI agent asks through the MCP server.
//!
//! The server holds no credentials, so it passes the three requests that need
//! the person's DriveThruRPG session (create a collection, add an item to
//! one, download an item) to the app over a local socket. The app listens only
//! while agent access is on. The server has already checked the scope and the
//! tool's switch and written the audit entry; the app checks again, so a
//! request that reaches the socket by another way is refused the same.

use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use dtrpg_core::data::collection::CollectionEntry;
use dtrpg_core::data::constants::AGENT_REQUEST_POLL_MS;
use dtrpg_core::data::mcp_access::{
    AppReply, AppRequest, Incoming, IpcEndpoint, IpcListener, McpSettings,
};
use serde_json::json;

use super::*;

/// The listener, and the requests it has received and nobody has answered.
pub(super) struct AgentLink {
    /// The listener; kept so the token can be rotated while access is on.
    /// Dropping it stops the listener.
    listener: IpcListener,
    incoming: Receiver<Incoming>,
}

impl AgentLink {
    /// Starts listening, or logs why it could not and returns `None`.
    fn start() -> Option<Self> {
        let (sender, incoming) = channel();
        match IpcListener::start(&IpcEndpoint::production(), sender) {
            Ok(listener) => Some(Self { listener, incoming }),
            Err(error) => {
                tracing::warn!(%error, "could not start listening for agent requests");
                None
            }
        }
    }
}

impl LibraryController {
    /// Starts or stops listening for an agent's requests, to match whether
    /// agent access is on.
    pub fn set_agent_access(&mut self, enabled: bool) {
        match (enabled, self.agent_link.is_some()) {
            (true, false) => self.agent_link = AgentLink::start(),
            (false, true) => self.agent_link = None,
            _ => {}
        }
    }

    /// Writes a new agent token, so a copy of the old one stops working.
    /// While the listener is running its in-memory copy is updated too;
    /// otherwise only the file is rewritten, which the next start replaces.
    pub fn rotate_agent_token(&mut self) {
        let result = match &self.agent_link {
            Some(link) => link.listener.rotate_token().map(|_| ()),
            None => IpcEndpoint::production().rotate_token().map(|_| ()),
        };
        if let Err(error) = result {
            tracing::warn!(%error, "could not rotate the agent token");
        }
    }

    /// Starts a loop that answers the agent's requests as they arrive, for
    /// the lifetime of the controller. The requests come from another thread,
    /// and are handled here because the collection and download code is the
    /// UI thread's. With agent access off there is nothing to look at.
    pub(super) fn start_agent_request_pump(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, async_cx| {
              loop {
                  async_cx.background_executor()
                          .timer(Duration::from_millis(AGENT_REQUEST_POLL_MS))
                          .await;
                  if this.update(async_cx, |ctrl, cx| ctrl.answer_agent_requests(cx))
                         .is_err()
                  {
                      break;
                  }
              }
          })
          .detach();
    }

    /// Answers every request that has arrived.
    fn answer_agent_requests(&mut self, cx: &mut Context<Self>) {
        let waiting: Vec<Incoming> = self.agent_link
                                         .as_ref()
                                         .map(|link| link.incoming.try_iter().collect())
                                         .unwrap_or_default();
        for incoming in waiting {
            self.answer_agent_request(incoming, cx);
        }
    }

    /// Checks the request against the settings, then does it. The reply goes
    /// back to the server, now or when the work finishes.
    fn answer_agent_request(&mut self, incoming: Incoming, cx: &mut Context<Self>) {
        let Incoming { request, reply } = incoming;
        if let Err(denial) = McpSettings::load().check(request.tool()) {
            reply.send(AppReply::failed(denial.message())).ok();
            return;
        }
        match request {
            AppRequest::CreateCollection { name } => {
                let name = name.trim().to_owned();
                if name.is_empty() {
                    reply.send(AppReply::failed("a collection needs a name"))
                         .ok();
                    return;
                }
                self.create_collection_then(name,
                                            move |result| {
                                                reply.send(created_reply(result)).ok();
                                            },
                                            cx);
            }
            AppRequest::AddToCollection { collection,
                                          item_id, } => {
                reply.send(self.agent_add_to_collection(&collection, &item_id, cx))
                     .ok();
            }
            AppRequest::DownloadItem { item_id } => {
                reply.send(self.agent_download(&item_id, cx)).ok();
            }
        }
    }

    /// Puts the item in the collection named or numbered `collection`.
    pub(super) fn agent_add_to_collection(&mut self, collection: &str, item_id: &str,
                                          cx: &mut Context<Self>)
                                          -> AppReply {
        let wanted = collection.trim().to_lowercase();
        let collections = self.collections();
        let Some(found) =
            collections.iter()
                       .find(|entry| {
                           entry.id.to_string() == wanted || entry.name.to_lowercase() == wanted
                       })
        else {
            return AppReply::failed(format!("no collection is named or numbered {collection:?}"));
        };
        let Some(item) = self.item_by_id(item_id)
        else {
            return AppReply::failed(format!("no item has the id {item_id:?}"));
        };
        if found.member_ids.contains(&item.order_product_id) {
            return AppReply::Done { data: json!({ "added": false,
                                                  "message": "the item is already in the collection" }), };
        }
        self.add_item_to_collection(found.id, item.order_product_id, item.product_id, cx);
        AppReply::Done { data: json!({ "added": true,
                                       "collection": found.name,
                                       "message": "Libri is adding the item; it tells the person if DriveThruRPG refuses" }), }
    }

    /// Starts downloading the item's files that are not downloaded yet.
    pub(super) fn agent_download(&mut self, item_id: &str, cx: &mut Context<Self>) -> AppReply {
        let Some(item) = self.item_by_id(item_id)
        else {
            return AppReply::failed(format!("no item has the id {item_id:?}"));
        };
        if item.status == ItemStatus::Downloaded {
            return AppReply::Done { data: json!({ "started": false,
                                                  "message": "the item is already downloaded" }), };
        }
        self.request_download(item_id, item.title.to_string(), cx);
        if self.pending_download.is_some() {
            return AppReply::Done { data: json!({ "started": false,
                                                  "message": "there is not enough free disk space; Libri asked the person to confirm" }), };
        }
        AppReply::Done { data: json!({ "started": true,
                                       "message": "the download has started and carries on in the background" }), }
    }
}

/// The reply to a request to create a collection.
pub(super) fn created_reply(result: Result<CollectionEntry, String>) -> AppReply {
    match result {
        Ok(entry) => AppReply::Done { data: json!({ "id": entry.id, "name": entry.name }), },
        Err(message) => AppReply::failed(message),
    }
}
