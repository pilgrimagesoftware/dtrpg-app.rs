//! Counting the pages of a PDF on disk.

use std::path::Path;

/// Returns `path`'s PDF page count, or `None` if the file can't be opened or
/// parsed as a PDF.
#[must_use]
pub fn pdf_page_count(path: &Path) -> Option<u32> {
    let doc = lopdf::Document::load(path).ok()?;
    u32::try_from(doc.get_pages().len()).ok()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_file_has_no_count() {
        assert_eq!(pdf_page_count(Path::new("/nonexistent/file.pdf")), None);
    }

    #[test]
    fn a_file_that_is_not_a_pdf_has_no_count() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.pdf");
        std::fs::write(&path, b"not a pdf").unwrap();

        assert_eq!(pdf_page_count(&path), None);
    }
}
