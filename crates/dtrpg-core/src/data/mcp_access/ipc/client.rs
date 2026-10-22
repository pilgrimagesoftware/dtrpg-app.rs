//! The MCP server's side: asking the app.

use std::time::Duration;

use interprocess::local_socket::tokio::{Stream, prelude::*};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use super::endpoint::IpcEndpoint;
use super::message::{AppReply, AppRequest, Envelope};

/// How long the server waits for the app's answer. A download is only
/// started, not finished, so the app answers quickly.
const REPLY_TIMEOUT: Duration = Duration::from_secs(30);

/// Why a request did not get an answer.
#[derive(Debug, Error)]
pub enum IpcError {
    /// Nothing is listening, which means Libri is not running or agent access
    /// is off.
    #[error("Libri is not running, or agent access is off in its settings")]
    AppNotRunning,
    /// The app did not answer in time.
    #[error("Libri did not answer in time")]
    TimedOut,
    /// The conversation broke.
    #[error("could not talk to Libri: {0}")]
    Io(#[from] std::io::Error),
    /// The answer was not understood.
    #[error("Libri's answer was not understood: {0}")]
    BadReply(#[from] serde_json::Error),
}

/// Sends `request` to the app at `endpoint` and waits for its answer.
///
/// # Errors
///
/// Returns [`IpcError::AppNotRunning`] when the app is not listening, and
/// another [`IpcError`] when the conversation fails.
pub async fn send_request(endpoint: &IpcEndpoint, request: AppRequest)
                          -> Result<AppReply, IpcError> {
    let token = endpoint.read_token().map_err(not_running)?;
    let stream = Stream::connect(endpoint.name()?).await
                                                  .map_err(not_running)?;
    let (reader, mut writer) = stream.split();

    let mut line = serde_json::to_string(&Envelope { token, request })?;
    line.push('\n');
    writer.write_all(line.as_bytes()).await?;

    let mut reply = String::new();
    tokio::time::timeout(REPLY_TIMEOUT, BufReader::new(reader).read_line(&mut reply))
        .await
        .map_err(|_| IpcError::TimedOut)??;
    Ok(serde_json::from_str(&reply)?)
}

/// A failure to reach the app, which is the same as the app not running.
fn not_running(error: std::io::Error) -> IpcError {
    match error.kind() {
        std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused => {
            IpcError::AppNotRunning
        }
        _ => IpcError::Io(error),
    }
}
