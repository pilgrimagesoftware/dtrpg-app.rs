//! The app's side: listening for requests.

use std::io;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};

use interprocess::local_socket::tokio::prelude::*;
use interprocess::local_socket::{ListenerOptions, tokio::Stream};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use super::endpoint::IpcEndpoint;
use super::message::{AppReply, AppRequest, Envelope};
use crate::runtime;

/// The longest a request line may be, so a stray program cannot make the app
/// hold an endless line in memory.
const MAX_REQUEST_BYTES: u64 = 16 * 1024;

/// A request that has arrived, and the way to answer it.
#[derive(Debug)]
pub struct Incoming {
    /// What is asked.
    pub request: AppRequest,
    /// Sends the answer back to the server.
    pub reply:   oneshot::Sender<AppReply>,
}

/// The app's listener. Requests arrive on the channel given to
/// [`IpcListener::start`]; dropping the listener stops it.
#[derive(Debug)]
pub struct IpcListener {
    endpoint: IpcEndpoint,
    token:    Arc<Mutex<String>>,
    task:     JoinHandle<()>,
}

impl IpcListener {
    /// Starts listening at `endpoint` and sends each request that carries the
    /// right token to `requests`.
    ///
    /// # Errors
    ///
    /// Returns an error if the shared runtime, the token file or the socket
    /// cannot be set up.
    pub fn start(endpoint: &IpcEndpoint, requests: Sender<Incoming>) -> io::Result<Self> {
        let runtime = runtime::handle()?;
        let token = endpoint.rotate_token()?;
        // A socket file left by a run that crashed would stop the bind.
        #[cfg(unix)]
        match std::fs::remove_file(endpoint.socket_path()) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
            _ => {}
        }
        // Binding registers the socket with the runtime, so it needs the
        // runtime's context even though this is not an async function.
        let _context = runtime.enter();
        let listener = ListenerOptions::new().name(endpoint.name()?)
                                             .create_tokio()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(endpoint.socket_path(),
                                     std::fs::Permissions::from_mode(0o600))?;
        }
        let token = Arc::new(Mutex::new(token));
        let token_for_task = token.clone();
        let task = runtime.spawn(async move {
                       loop {
                           match listener.accept().await {
                               Ok(stream) => {
                                   tokio::spawn(serve(stream,
                                                      token_for_task.clone(),
                                                      requests.clone()));
                               }
                               Err(error) => {
                                   tracing::warn!(%error, "could not accept an agent request");
                               }
                           }
                       }
                   });
        Ok(Self { endpoint: endpoint.clone(),
                  token,
                  task })
    }

    /// Writes a new token and starts accepting only requests that carry it.
    /// A copy of the old token, however it was kept, stops working. The
    /// agent reads the token from the file on each request, so a running
    /// agent needs nothing more.
    ///
    /// # Errors
    ///
    /// Returns an error if the token file cannot be written. The old token
    /// stays in force.
    pub fn rotate_token(&self) -> io::Result<String> {
        let fresh = self.endpoint.rotate_token()?;
        *self.token.lock().unwrap_or_else(PoisonError::into_inner) = fresh.clone();
        Ok(fresh)
    }
}

impl Drop for IpcListener {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// Reads one request from `stream`, hands it to the app and writes back the
/// answer.
async fn serve(stream: Stream, token: Arc<Mutex<String>>, requests: Sender<Incoming>) {
    let (reader, mut writer) = stream.split();
    let mut line = String::new();
    let read = BufReader::new(reader.take(MAX_REQUEST_BYTES)).read_line(&mut line)
                                                             .await;
    let reply = match read {
        Ok(_) => answer(&line, &token, &requests).await,
        Err(error) => AppReply::failed(format!("could not read the request: {error}")),
    };
    let Ok(mut text) = serde_json::to_string(&reply)
    else {
        return;
    };
    text.push('\n');
    if let Err(error) = writer.write_all(text.as_bytes()).await {
        tracing::debug!(%error, "could not answer an agent request");
    }
}

/// The answer to the request in `line`: the app's, or a refusal.
async fn answer(line: &str, token: &Arc<Mutex<String>>, requests: &Sender<Incoming>) -> AppReply {
    let Ok(envelope) = serde_json::from_str::<Envelope>(line)
    else {
        return AppReply::failed("not a request Libri understands");
    };
    // Read the current token without holding the lock across the await
    // below, so a rotation cannot stall the request being answered.
    let current = token.lock().unwrap_or_else(PoisonError::into_inner).clone();
    if envelope.token != current {
        return AppReply::failed("the request did not carry Libri's token");
    }
    let (reply, answer) = oneshot::channel();
    if requests.send(Incoming { request: envelope.request,
                                reply })
               .is_err()
    {
        return AppReply::failed("Libri is shutting down");
    }
    answer.await
          .unwrap_or_else(|_| AppReply::failed("Libri did not answer"))
}
