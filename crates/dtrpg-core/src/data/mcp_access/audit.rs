//! The log of every call an agent made, kept as one JSON object per line so
//! the server can append without reading the file.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::McpScope;
use crate::data::paths::app_data_dir;

/// The most entries the log keeps. Older ones are dropped when it grows past
/// this.
const MAX_ENTRIES: usize = 1000;

/// One call.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuditEntry {
    /// When the call was made, in Unix seconds.
    pub time_secs: i64,
    /// The tool's name.
    pub tool:      String,
    /// What the call was about, in a few words. Never holds credentials.
    pub summary:   String,
    /// The scope the tool needs.
    pub scope:     McpScope,
    /// Whether the call was refused.
    pub denied:    bool,
}

/// The audit log file.
pub struct AuditLog {
    path: PathBuf,
}

impl AuditLog {
    /// The log in the app's data folder.
    #[must_use]
    pub fn open() -> Self {
        Self::at(app_data_dir().join("mcp_audit.jsonl"))
    }

    /// The log at `path`.
    #[must_use]
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Adds `entry` to the end of the log, and drops the oldest entries if the
    /// log has grown too long. A failure is logged and otherwise ignored: a
    /// call is not worth refusing because its record could not be written.
    pub fn append(&self, entry: &AuditEntry) {
        if let Err(error) = self.try_append(entry) {
            tracing::warn!(%error, path = %self.path.display(), "could not write the audit log");
        }
    }

    fn try_append(&self, entry: &AuditEntry) -> std::io::Result<()> {
        if let Some(dir) = self.path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let line = serde_json::to_string(entry).map_err(std::io::Error::other)?;
        let mut file = OpenOptions::new().create(true)
                                         .append(true)
                                         .open(&self.path)?;
        writeln!(file, "{line}")?;
        drop(file);
        self.trim()
    }

    /// Keeps the newest [`MAX_ENTRIES`] lines.
    fn trim(&self) -> std::io::Result<()> {
        let text = std::fs::read_to_string(&self.path)?;
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() <= MAX_ENTRIES {
            return Ok(());
        }
        let kept = lines[lines.len() - MAX_ENTRIES..].join("\n");
        std::fs::write(&self.path, kept + "\n")
    }

    /// The newest `limit` entries, newest first. A line that cannot be read is
    /// skipped, and a missing log is empty.
    #[must_use]
    pub fn recent(&self, limit: usize) -> Vec<AuditEntry> {
        read_entries(&self.path).into_iter()
                                .rev()
                                .take(limit)
                                .collect()
    }
}

fn read_entries(path: &Path) -> Vec<AuditEntry> {
    std::fs::read_to_string(path).map(|text| {
                                     text.lines()
                                         .filter_map(|line| serde_json::from_str(line).ok())
                                         .collect()
                                 })
                                 .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn entry(n: i64, denied: bool) -> AuditEntry {
        AuditEntry { time_secs: n,
                     tool: "list_items".to_owned(),
                     summary: format!("call {n}"),
                     scope: McpScope::Read,
                     denied }
    }

    #[test]
    fn a_missing_log_is_empty() {
        let dir = tempfile::tempdir().unwrap();
        assert!(AuditLog::at(dir.path().join("none.jsonl")).recent(10)
                                                           .is_empty());
    }

    #[test]
    fn entries_come_back_newest_first_with_what_was_written() {
        let dir = tempfile::tempdir().unwrap();
        let log = AuditLog::at(dir.path().join("a").join("log.jsonl"));

        log.append(&entry(1, false));
        log.append(&entry(2, true));

        assert_eq!(log.recent(10), [entry(2, true), entry(1, false)]);
        assert_eq!(log.recent(1), [entry(2, true)]);
    }

    #[test]
    fn the_log_keeps_only_the_newest_entries() {
        let dir = tempfile::tempdir().unwrap();
        let log = AuditLog::at(dir.path().join("log.jsonl"));

        for n in 0..i64::try_from(MAX_ENTRIES).unwrap() + 5 {
            log.append(&entry(n, false));
        }

        let all = log.recent(usize::MAX);
        assert_eq!(all.len(), MAX_ENTRIES);
        assert_eq!(all[0].time_secs, i64::try_from(MAX_ENTRIES).unwrap() + 4);
    }

    #[test]
    fn a_damaged_line_is_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("log.jsonl");
        let log = AuditLog::at(&path);
        log.append(&entry(1, false));
        std::fs::write(&path,
                       std::fs::read_to_string(&path).unwrap() + "not json\n").unwrap();
        log.append(&entry(2, false));

        assert_eq!(log.recent(10).len(), 2);
    }
}
