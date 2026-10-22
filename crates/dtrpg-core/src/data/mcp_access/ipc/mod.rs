//! How the MCP server asks the running app to act for an agent.
//!
//! Creating a collection and downloading a file call DriveThruRPG with the
//! person's session. The MCP server is a separate process that holds no
//! credentials, so it cannot do them itself: it sends the app a typed
//! [`AppRequest`] over a local socket and the app answers with an
//! [`AppReply`]. The app only listens while agent access is on.
//!
//! The socket is reachable by other programs on the machine, so every request
//! also carries a token that only the person's own account can read (see
//! [`IpcEndpoint`]).

mod client;
mod endpoint;
mod message;
mod server;
// A failed unwrap is how a test fails.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests;

pub use client::{IpcError, send_request};
pub use endpoint::IpcEndpoint;
pub use message::{AppReply, AppRequest};
pub use server::{Incoming, IpcListener};
