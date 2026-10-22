//! MCP server binary that exposes tools to interact with the local library and
//! archive contents.
//!
//! What an agent may do is decided in Libri's settings, not here: access is off
//! until it is switched on, then limited to the scopes and tools chosen there,
//! and every call, allowed or refused, is written to the audit log the settings
//! window shows. The server reads those settings on each call, so a change
//! applies to the next call. It holds no credentials and talks to nothing but
//! the local library index.

mod requests;
mod server;
mod tools_change;
mod tools_read;

use dtrpg_core::api::LibraryApi;
use dtrpg_core::data::paths::cache_dir;
use dtrpg_core::services::sdk::{RustSdkCollectionsService, RustSdkLibraryService};
use rmcp::ServiceExt;

use crate::server::{Access, LibriMcpServer};

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), rmcp::RmcpError> {
    let api = LibraryApi::new(Box::new(RustSdkLibraryService::unauthenticated()),
                              Box::new(RustSdkCollectionsService::unauthenticated()),
                              cache_dir());

    let service = LibriMcpServer::new(api, Access::production()).serve(rmcp::transport::stdio())
                                                                .await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests;
