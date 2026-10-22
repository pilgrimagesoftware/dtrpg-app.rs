//! SQLite operations for where each downloaded file is kept.
//!
//! A file's location is the name of the storage location it was put in, written
//! when it was downloaded, so it stays true when the placement rules change
//! later.

use std::collections::HashMap;

use rusqlite::{Connection, params};

use super::{IndexError, LibraryIndex};

/// Replaces what is recorded for `item_id` with `locations`, pairs of a file's
/// name and the name of the location it is in.
fn set(conn: &mut Connection, item_id: &str, locations: &[(String, String)])
       -> rusqlite::Result<()> {
    let transaction = conn.transaction()?;
    transaction.execute("DELETE FROM file_locations WHERE item_id = ?1",
                        params![item_id])?;
    for (file_name, location) in locations {
        transaction.execute(
                            "INSERT INTO file_locations (item_id, file_name, location)
             VALUES (?1, ?2, ?3)",
                            params![item_id, file_name, location],
        )?;
    }
    transaction.commit()
}

/// Forgets where `item_id`'s files were.
fn clear(conn: &Connection, item_id: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM file_locations WHERE item_id = ?1",
                 params![item_id])?;
    Ok(())
}

/// The names of the locations each item has files in, by item id, without
/// repeats and in name order. Items with no recorded file are left out.
fn by_item(conn: &Connection) -> rusqlite::Result<HashMap<String, Vec<String>>> {
    let mut statement = conn.prepare(
                                     "SELECT DISTINCT item_id, location FROM file_locations
         ORDER BY item_id, location",
    )?;
    let rows = statement.query_map([], |row| {
                            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                        })?;
    let mut by_item: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        let (item_id, location) = row?;
        by_item.entry(item_id).or_default().push(location);
    }
    Ok(by_item)
}

/// The ids of downloaded items that have no recorded file location: those
/// downloaded before locations were recorded.
fn downloaded_without_location(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement = conn.prepare(
                                     "SELECT id FROM items
         WHERE status = 'downloaded'
         AND id NOT IN (SELECT item_id FROM file_locations)",
    )?;
    statement.query_map([], |row| row.get(0))?.collect()
}

impl LibraryIndex {
    /// Replaces what is recorded about where `item_id`'s files are kept with
    /// `locations`, pairs of a file's name and its storage location's name.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the write fails.
    pub fn set_file_locations(&mut self, item_id: &str, locations: &[(String, String)])
                              -> Result<(), IndexError> {
        Ok(set(&mut self.conn, item_id, locations)?)
    }

    /// Forgets where `item_id`'s files are kept, for when they are removed.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the write fails.
    pub fn clear_file_locations(&self, item_id: &str) -> Result<(), IndexError> {
        Ok(clear(&self.conn, item_id)?)
    }

    /// The names of the storage locations each item has files in, by item id.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the query fails.
    pub fn file_locations_by_item(&self) -> Result<HashMap<String, Vec<String>>, IndexError> {
        Ok(by_item(&self.conn)?)
    }

    /// The ids of downloaded items with no recorded file location.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError::Sqlite`] if the query fails.
    pub fn downloaded_items_without_location(&self) -> Result<Vec<String>, IndexError> {
        Ok(downloaded_without_location(&self.conn)?)
    }
}
