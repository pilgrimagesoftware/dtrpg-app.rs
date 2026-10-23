//! Where the socket and its token are.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use interprocess::local_socket::Name;

use crate::data::paths::app_data_dir;

/// The socket the app listens on and the token file that goes with it.
///
/// On macOS and Linux the socket is a file in the app's data folder; on
/// Windows it is a named pipe. Windows lets other accounts connect to a pipe,
/// and a file's permissions are only a first line of defence anywhere, so the
/// app also writes a random token to a file only the person's account can read,
/// and refuses requests that do not carry it.
#[derive(Clone, Debug)]
pub struct IpcEndpoint {
    folder: PathBuf,
}

impl IpcEndpoint {
    /// The endpoint in the app's own data folder.
    #[must_use]
    pub fn production() -> Self {
        Self::at(app_data_dir())
    }

    /// An endpoint whose files are in `folder`.
    #[must_use]
    pub fn at(folder: impl Into<PathBuf>) -> Self {
        Self { folder: folder.into(), }
    }

    /// The file holding the token a request must carry.
    #[must_use]
    pub fn token_path(&self) -> PathBuf {
        self.folder.join("agent.token")
    }

    /// The socket file on macOS and Linux.
    #[cfg(unix)]
    pub(super) fn socket_path(&self) -> PathBuf {
        self.folder.join("agent.sock")
    }

    /// The name to listen on or connect to.
    pub(super) fn name(&self) -> io::Result<Name<'static>> {
        #[cfg(unix)]
        {
            use interprocess::local_socket::{GenericFilePath, ToFsName};

            self.socket_path().to_fs_name::<GenericFilePath>()
        }
        #[cfg(not(unix))]
        {
            use interprocess::local_socket::{GenericNamespaced, ToNsName};

            self.pipe_name().to_ns_name::<GenericNamespaced>()
        }
    }

    /// The pipe name on Windows: one per folder, so tests and a second
    /// profile do not meet.
    #[cfg(not(unix))]
    fn pipe_name(&self) -> String {
        let suffix: String = self.folder
                                 .to_string_lossy()
                                 .chars()
                                 .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
                                 .collect();
        format!("libri-agent-{suffix}")
    }

    /// What the agent settings page says the server is bound to. This is
    /// never a network address: the socket is local, and loopback-only by
    /// construction.
    #[must_use]
    pub fn bind_display(&self) -> String {
        #[cfg(unix)]
        {
            self.socket_path().display().to_string()
        }
        #[cfg(not(unix))]
        {
            self.pipe_name()
        }
    }

    /// Writes a new random token and returns it. The app calls this when the
    /// listener starts; the settings page calls it when the person rotates
    /// the token so an old copy of the file stops working.
    pub fn rotate_token(&self) -> io::Result<String> {
        fs::create_dir_all(&self.folder)?;
        let token = format!("{}{}",
                            uuid::Uuid::new_v4().simple(),
                            uuid::Uuid::new_v4().simple());
        write_private(&self.token_path(), &token)?;
        Ok(token)
    }

    /// The token the app wrote, for a request to carry.
    pub(super) fn read_token(&self) -> io::Result<String> {
        Ok(fs::read_to_string(self.token_path())?.trim().to_owned())
    }
}

/// Writes `text` to `path` so that only the person's own account can read it.
fn write_private(path: &Path, text: &str) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = fs::OpenOptions::new().write(true)
                                             .create(true)
                                             .truncate(true)
                                             .mode(0o600)
                                             .open(path)?;
        file.write_all(text.as_bytes())
    }
    #[cfg(not(unix))]
    {
        // The data folder is in the person's own profile, which other
        // accounts cannot read.
        fs::write(path, text)
    }
}
