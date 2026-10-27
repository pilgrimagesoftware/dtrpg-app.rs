//! Filing a bug or feature report: the issue body, an optional log tail and
//! named screenshots, and delivery through `gh` or a pre-filled browser page.
//!
//! GitHub has no API for attaching a file to an issue, so a log is attached as
//! the tail of its text in the body, and screenshots are named in the body and
//! revealed in the file manager for the user to drag onto the page. Everything
//! here is blocking; callers run [`submit`] and [`read_log`] off the UI thread.
//! The body is English on purpose: it is read by whoever triages the issue.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::constants::{
    GH_FALLBACK_PATHS, REPORT_LOG_TAIL_BYTES, REPORT_LOG_TAIL_LINES, REPORT_MAX_SCREENSHOTS,
    REPORT_REPO, REPORT_SCREENSHOT_EXTENSIONS,
};
use crate::logging::platform_log_dir;

/// What the user filed, plus the build diagnostics that accompany it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Report {
    /// Issue title, already carrying its kind prefix.
    pub title:       String,
    /// The user's description.
    pub description: String,
    /// Version, commit, build date and target, one per line.
    pub diagnostics: String,
    /// Tail of the application log, when the user attached it.
    pub log_tail:    Option<String>,
    /// Screenshots the user chose, by path. Named in the body, never read.
    pub screenshots: Vec<PathBuf>,
}

/// How a submission ended.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// Filed through `gh`, at this URL.
    Filed(String),
    /// `gh` was usable but filing failed, with why.
    FileFailed(String),
    /// `gh` was not usable; the compose page opened in the browser.
    BrowserOpened,
    /// `gh` was not usable and the browser could not be opened.
    BrowserFailed,
}

/// Runs `gh`, returning its stdout on success or its stderr on failure.
pub trait GhRunner {
    /// Whether `gh` is installed and signed in.
    fn ready(&self) -> bool;
    /// Runs `gh` with `args`, feeding `stdin` when given.
    fn run(&self, args: &[&str], stdin: Option<&str>) -> Result<String, String>;
}

/// The real `gh`, found on the PATH or in a Homebrew location a Finder-launched
/// app's PATH lacks.
pub struct SystemGh(Option<PathBuf>);

impl SystemGh {
    /// Looks for `gh`.
    pub fn locate() -> Self {
        let on_path =
            std::env::var_os("PATH").into_iter()
                                    .flat_map(|paths| {
                                        std::env::split_paths(&paths).collect::<Vec<_>>()
                                    })
                                    .map(|dir| dir.join("gh"));
        let fallbacks = GH_FALLBACK_PATHS.iter().map(PathBuf::from);
        Self(on_path.chain(fallbacks).find(|path| path.is_file()))
    }
}

impl GhRunner for SystemGh {
    fn ready(&self) -> bool {
        self.run(&["auth", "status"], None).is_ok()
    }

    fn run(&self, args: &[&str], stdin: Option<&str>) -> Result<String, String> {
        let program = self.0.as_deref().ok_or("gh is not installed")?;
        let mut child = Command::new(program).args(args)
                                             .stdin(Stdio::piped())
                                             .stdout(Stdio::piped())
                                             .stderr(Stdio::piped())
                                             .spawn()
                                             .map_err(|error| error.to_string())?;
        if let Some(mut pipe) = child.stdin.take() {
            // Dropping the pipe closes stdin, which `--body-file -` waits on.
            pipe.write_all(stdin.unwrap_or_default().as_bytes())
                .map_err(|error| error.to_string())?;
        }
        let output = child.wait_with_output()
                          .map_err(|error| error.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        }
        else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
        }
    }
}

/// Files `report` through `gh` when it is ready, otherwise hands the compose
/// page to `open_url`. Screenshots end up one drag from the issue: a filed
/// issue's page opens, and either way `reveal` shows each file.
pub fn submit(report: &Report, gh: &impl GhRunner, open_url: impl Fn(&str) -> bool,
              reveal: impl Fn(&Path) -> bool)
              -> Outcome {
    if gh.ready() {
        let args = ["issue",
                    "create",
                    "--repo",
                    REPORT_REPO,
                    "--title",
                    &report.title,
                    "--body-file",
                    "-"];
        return match gh.run(&args, Some(&issue_body(report))) {
            Ok(url) => {
                if !report.screenshots.is_empty() {
                    open_url(&url);
                    report.screenshots.iter().for_each(|path| {
                                                 reveal(path);
                                             });
                }
                Outcome::Filed(url)
            }
            Err(error) => Outcome::FileFailed(error),
        };
    }
    if open_url(&compose_url(report)) {
        report.screenshots.iter().for_each(|path| {
                                     reveal(path);
                                 });
        Outcome::BrowserOpened
    }
    else {
        Outcome::BrowserFailed
    }
}

/// The description, the diagnostics fenced so GitHub keeps their lines, then
/// the log tail in a collapsed block and the screenshot names.
pub fn issue_body(report: &Report) -> String {
    body(report,
         report.log_tail
               .as_deref()
               .map(|tail| format!("<details>\n<summary>Application log</summary>\n\n````\n{tail}\n````\n\n</details>\n")))
}

/// [`issue_body`] for the browser, which names the log file rather than
/// carrying it: a compose URL that long is refused.
fn compose_body(report: &Report) -> String {
    body(report,
         report.log_tail.as_ref().map(|_| {
                                     format!("Application log: attach `{}` by hand.\n",
                                             platform_log_dir().display())
                                 }))
}

fn body(report: &Report, log: Option<String>) -> String {
    let mut text = format!("{}\n\n---\n{}\n",
                           report.description.trim(),
                           report.diagnostics.trim_end());
    if let Some(log) = log {
        text.push_str(&format!("\n### Logs\n\n{log}"));
    }
    if !report.screenshots.is_empty() {
        let names: String = report.screenshots
                                  .iter()
                                  .map(|path| format!("- `{}`\n", file_name(path)))
                                  .collect();
        text.push_str(&format!(
            "\n### Screenshots\n\nThe reporter adds these to the issue:\n\n{names}"
        ));
    }
    text
}

/// The repository's new-issue page with the title and body pre-filled.
pub fn compose_url(report: &Report) -> String {
    format!("https://github.com/{REPORT_REPO}/issues/new?title={}&body={}",
            percent_encode(&report.title),
            percent_encode(&compose_body(report)))
}

/// Percent-encodes everything but RFC 3986's unreserved characters.
pub fn percent_encode(value: &str) -> String {
    value.bytes()
         .map(|byte| {
             if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                 char::from(byte).to_string()
             }
             else {
                 format!("%{byte:02X}")
             }
         })
         .collect()
}

/// A screenshot's file name. Not its path, which names the home directory.
pub fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(|| path.display().to_string(),
                                 |name| name.to_string_lossy().into_owned())
}

/// Appends the image files among `picked` to `chosen`, up to
/// [`REPORT_MAX_SCREENSHOTS`], returning how many were skipped (not an image,
/// or past the cap). A file already chosen is neither added twice nor counted.
pub fn add_screenshots(chosen: &mut Vec<PathBuf>, picked: impl IntoIterator<Item = PathBuf>)
                       -> usize {
    let mut skipped = 0;
    for path in picked {
        if chosen.contains(&path) {
            continue;
        }
        let is_image =
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| {
                    REPORT_SCREENSHOT_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
                });
        if is_image && chosen.len() < REPORT_MAX_SCREENSHOTS {
            chosen.push(path);
        }
        else {
            skipped += 1;
        }
    }
    skipped
}

/// The tail of the newest application log, or `None` when there is none to
/// read. Blocks on the file system.
pub fn read_log() -> Option<String> {
    let newest = std::fs::read_dir(platform_log_dir()).ok()?
                                                      .filter_map(Result::ok)
                                                      .filter(|entry| {
                                                          entry.file_name()
                                                               .to_string_lossy()
                                                               .starts_with("libri.log")
                                                      })
                                                      .max_by_key(|entry| {
                                                          entry.metadata()
                                                               .and_then(|meta| meta.modified())
                                                               .ok()
                                                      })?;
    let bytes = read_tail(&newest.path()).ok()?;
    Some(tail_lines(&bytes, REPORT_LOG_TAIL_LINES))
}

/// The last [`REPORT_LOG_TAIL_BYTES`] of the file, dropping a leading partial
/// line.
fn read_tail(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let start = file.metadata()?
                    .len()
                    .saturating_sub(u64::try_from(REPORT_LOG_TAIL_BYTES).unwrap_or(u64::MAX));
    file.seek(SeekFrom::Start(start))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    if start > 0
       && let Some(newline) = bytes.iter().position(|&byte| byte == b'\n')
    {
        bytes.drain(..=newline);
    }
    Ok(bytes)
}

/// The last `lines` lines of `bytes`; invalid UTF-8 is replaced, not refused.
pub fn tail_lines(bytes: &[u8], lines: usize) -> String {
    let text = String::from_utf8_lossy(bytes);
    let kept: Vec<&str> = text.trim_end_matches('\n')
                              .rsplit('\n')
                              .take(lines)
                              .collect();
    kept.into_iter().rev().collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests;
