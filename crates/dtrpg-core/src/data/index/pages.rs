//! SQLite operations for the page count of each downloaded file.

use std::collections::HashMap;

use rusqlite::{Connection, params};

use super::{IndexError, LibraryIndex};
use crate::catalog_scan;

/// Replaces what is recorded for `item_id` with `pages`, pairs of a file's name
/// and its page count.
fn set(conn: &mut Connection, item_id: &str, pages: &[(String, u32)]) -> rusqlite::Result<()> {
    let transaction = conn.transaction()?;
    transaction.execute("DELETE FROM file_pages WHERE item_id = ?1",
                        params![item_id])?;
    for (file_name, count) in pages {
        transaction.execute(
                            "INSERT INTO file_pages (item_id, file_name, pages)
             VALUES (?1, ?2, ?3)",
                            params![item_id, file_name, count],
        )?;
    }
    transaction.commit()
}

/// Forgets the page counts of `item_id`'s files.
pub(super) fn clear(conn: &Connection, item_id: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM file_pages WHERE item_id = ?1",
                 params![item_id])?;
    Ok(())
}

/// The pages in each item's downloaded files, by item id. Items with no
/// recorded file are left out.
#[track_caller]
fn by_item(conn: &Connection) -> rusqlite::Result<HashMap<String, u32>> {
    catalog_scan!();
    let mut statement =
        conn.prepare("SELECT item_id, SUM(pages) FROM file_pages GROUP BY item_id")?;
    let rows = statement.query_map([], |row| {
                            Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
                        })?;
    rows.collect()
}

/// The pages in `item_id`'s downloaded files, or `None` if none are recorded.
fn of_item(conn: &Connection, item_id: &str) -> rusqlite::Result<Option<u32>> {
    conn.query_row("SELECT SUM(pages) FROM file_pages WHERE item_id = ?1",
                   params![item_id],
                   |row| row.get(0))
}

/// The pages in all the downloaded files.
#[track_caller]
fn total(conn: &Connection) -> rusqlite::Result<u64> {
    catalog_scan!();
    let pages: i64 = conn.query_row("SELECT COALESCE(SUM(pages), 0) FROM file_pages",
                                    [],
                                    |row| row.get(0))?;
    Ok(u64::try_from(pages).unwrap_or_default())
}

/// The ids of downloaded items whose files have not had their pages counted.
#[track_caller]
fn downloaded_without_pages(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    catalog_scan!();
    let mut statement = conn.prepare(
                                     "SELECT id FROM items
         WHERE status = 'downloaded'
         AND id NOT IN (SELECT item_id FROM file_pages)",
    )?;
    statement.query_map([], |row| row.get(0))?.collect()
}

impl LibraryIndex {
    /// Replaces the page counts recorded for `item_id` with `pages`, pairs of a
    /// file's name and its page count.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the write fails.
    pub fn set_file_pages(&mut self, item_id: &str, pages: &[(String, u32)])
                          -> Result<(), IndexError> {
        Ok(set(&mut self.conn, item_id, pages)?)
    }

    /// The pages in each item's downloaded files, by item id.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the query fails.
    #[track_caller]
    pub fn file_pages_by_item(&self) -> Result<HashMap<String, u32>, IndexError> {
        Ok(by_item(&self.conn)?)
    }

    /// The pages in `item_id`'s downloaded files, or `None` if none are
    /// recorded.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the query fails.
    pub fn file_pages_of(&self, item_id: &str) -> Result<Option<u32>, IndexError> {
        Ok(of_item(&self.conn, item_id)?)
    }

    /// The pages in all the downloaded files.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the query fails.
    #[track_caller]
    pub fn total_pages(&self) -> Result<u64, IndexError> {
        Ok(total(&self.conn)?)
    }

    /// The ids of downloaded items whose files have not had their pages
    /// counted.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the query fails.
    #[track_caller]
    pub fn downloaded_items_without_pages(&self) -> Result<Vec<String>, IndexError> {
        Ok(downloaded_without_pages(&self.conn)?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn names(pairs: &[(&str, u32)]) -> Vec<(String, u32)> {
        pairs.iter()
             .map(|(name, count)| ((*name).to_owned(), *count))
             .collect()
    }

    #[test]
    fn the_total_adds_every_file_and_is_zero_when_none_are_recorded() {
        let mut index = LibraryIndex::open_in_memory().unwrap();
        assert_eq!(index.total_pages().unwrap(), 0);

        index.set_file_pages("a", &names(&[("one.pdf", 100), ("two.pdf", 20)]))
             .unwrap();
        index.set_file_pages("b", &names(&[("three.pdf", 4)]))
             .unwrap();

        assert_eq!(index.total_pages().unwrap(), 124);
    }

    #[test]
    fn an_item_s_pages_are_the_sum_of_its_files() {
        let mut index = LibraryIndex::open_in_memory().unwrap();
        index.set_file_pages("a", &names(&[("one.pdf", 100), ("two.pdf", 20)]))
             .unwrap();

        assert_eq!(index.file_pages_by_item().unwrap().get("a"), Some(&120));
    }

    #[test]
    fn recording_again_replaces_what_was_recorded() {
        let mut index = LibraryIndex::open_in_memory().unwrap();
        index.set_file_pages("a", &names(&[("one.pdf", 100)]))
             .unwrap();
        index.set_file_pages("a", &names(&[("one.pdf", 7)]))
             .unwrap();

        assert_eq!(index.total_pages().unwrap(), 7);
    }

    #[test]
    fn clearing_forgets_the_item_s_pages() {
        let mut index = LibraryIndex::open_in_memory().unwrap();
        index.set_file_pages("a", &names(&[("one.pdf", 100)]))
             .unwrap();
        index.clear_file_locations("a").unwrap();

        assert_eq!(index.total_pages().unwrap(), 0);
    }
}
