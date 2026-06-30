//! Logging initialisation: WARN+ to stderr and to a rolling daily log file.
//!
//! The log directory follows platform conventions:
//! - macOS:   `~/Library/Logs/com.pilgrimagesoftware.dtrpg/`
//! - Linux:   `$XDG_DATA_HOME/com.pilgrimagesoftware.dtrpg/logs/`
//! - Windows: `%APPDATA%\com.pilgrimagesoftware.dtrpg\logs\`
//!
//! The default filter level is `WARN`. Set `RUST_LOG` to override (e.g.
//! `RUST_LOG=debug cargo run` for verbose output during development).

use std::path::PathBuf;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

/// Initialises the global tracing subscriber with console and file layers.
///
/// Returns a [`WorkerGuard`] that **must be kept alive** for the lifetime of
/// the process; dropping it early will stop the background flush thread and
/// may lose buffered log lines.
///
/// If the log directory cannot be created the function falls back to
/// console-only logging and returns `None`.
pub fn init() -> Option<WorkerGuard> {
    let log_dir = platform_log_dir();

    if let Err(e) = std::fs::create_dir_all(&log_dir) {
        // Can't create log dir — set up console-only and continue.
        init_console_only();
        eprintln!(
            "warning: could not create log directory {}: {e}",
            log_dir.display()
        );
        return None;
    }

    let file_appender = tracing_appender::rolling::daily(&log_dir, "libri.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stderr).compact())
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    Some(guard)
}

fn init_console_only() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stderr).compact())
        .init();
}

fn platform_log_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir()
            .map(|h| h.join("Library/Logs/com.pilgrimagesoftware.dtrpg"))
            .unwrap_or_else(|| PathBuf::from("/tmp/com.pilgrimagesoftware.dtrpg"))
    }
    #[cfg(not(target_os = "macos"))]
    {
        dirs::data_local_dir()
            .map(|d| d.join("com.pilgrimagesoftware.dtrpg/logs"))
            .unwrap_or_else(|| PathBuf::from("/tmp/com.pilgrimagesoftware.dtrpg/logs"))
    }
}
