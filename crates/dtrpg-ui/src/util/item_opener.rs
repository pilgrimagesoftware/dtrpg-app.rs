//! Service for opening downloaded catalog items in the system's default application.

use std::path::Path;
use thiserror::Error;

/// Errors that can occur when attempting to open an item.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum OpenError {
    /// The file was not found at the specified path.
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// No default application is configured for this file type.
    #[error("No default application configured for this file type")]
    NoDefaultApp,

    /// The operating system failed to open the file.
    #[error("Failed to open file: {0}")]
    OsFailed(String),

    /// Multiple files exist and user selection is required.
    #[error("Multiple files require selection")]
    MultipleFilesRequireSelection,
}

/// Service for opening files in the system's default application.
pub struct ItemOpener;

impl ItemOpener {
    /// Opens the file at the given path using the system's default application.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file does not exist (`OpenError::FileNotFound`)
    /// - No default application is configured (`OpenError::NoDefaultApp`)
    /// - The OS fails to open the file (`OpenError::OsFailed`)
    pub fn open(path: &Path) -> Result<(), OpenError> {
        // Check if the file exists first
        if !path.exists() {
            return Err(OpenError::FileNotFound(
                path.display().to_string(),
            ));
        }

        // Attempt to open the file with the system's default application
        open::that(path).map_err(|e| {
            let error_msg = e.to_string();

            // Try to classify the error
            if error_msg.contains("no default application")
                || error_msg.contains("no associated application") {
                OpenError::NoDefaultApp
            } else {
                OpenError::OsFailed(error_msg)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_open_nonexistent_file() {
        let path = Path::new("/tmp/nonexistent_file_that_should_not_exist.txt");
        let result = ItemOpener::open(path);

        assert!(matches!(result, Err(OpenError::FileNotFound(_))));
    }

    #[test]
    fn test_open_existing_file() {
        // Create a temporary file
        let temp_path = std::env::temp_dir().join("dtrpg_test_file.txt");
        let Ok(mut file) = File::create(&temp_path) else {
            // Skip test if we can't create the temp file
            return;
        };
        if file.write_all(b"test content").is_err() {
            // Skip test if we can't write
            let _ = std::fs::remove_file(&temp_path);
            return;
        }
        drop(file);

        // Try to open it - this may succeed or fail depending on the environment
        // We just verify it doesn't return FileNotFound
        let result = ItemOpener::open(&temp_path);

        // Clean up
        let _ = std::fs::remove_file(&temp_path);

        // The result should either be Ok or an error other than FileNotFound
        if let Err(e) = result {
            assert!(
                !matches!(e, OpenError::FileNotFound(_)),
                "Should not return FileNotFound for existing file"
            );
        }
    }

    #[test]
    fn test_error_types_are_cloneable() {
        let err = OpenError::FileNotFound("test.txt".to_string());
        let _cloned = err.clone();
    }

    #[test]
    fn test_error_types_are_comparable() {
        let err1 = OpenError::NoDefaultApp;
        let err2 = OpenError::NoDefaultApp;
        assert_eq!(err1, err2);
    }
}
