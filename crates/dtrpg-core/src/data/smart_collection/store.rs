//! Keeping the user's smart collections in a file.
//!
//! Backed by `{app_preferences_dir}/smart_collections.toml`.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::model::SmartCollection;
use crate::data::constants::SMART_COLLECTIONS_FILE;
use crate::data::paths::app_preferences_dir;

/// Why smart collections could not be read or written.
#[derive(Debug, Error)]
pub enum SmartCollectionError {
    /// The file exists but could not be read.
    #[error("could not read {path}: {source}")]
    Read {
        /// The file.
        path:   PathBuf,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
    /// The file is not valid smart collection data.
    #[error("could not parse {path}: {source}")]
    Parse {
        /// The file.
        path:   PathBuf,
        /// What went wrong.
        #[source]
        source: toml::de::Error,
    },
    /// The collections could not be turned into text.
    #[error("could not encode the smart collections: {0}")]
    Encode(#[from] toml::ser::Error),
    /// The file could not be written.
    #[error("could not write {path}: {source}")]
    Write {
        /// The file.
        path:   PathBuf,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
}

/// How the collections look in the file.
#[derive(Default, Deserialize, Serialize)]
struct StoreFile {
    /// Every smart collection, in the order they were added.
    #[serde(default)]
    collections: Vec<SmartCollection>,
}

/// The user's smart collections, kept in a file that every change rewrites.
pub struct SmartCollectionStore {
    /// Where the collections are kept.
    path:        PathBuf,
    /// The collections, in the order they were added.
    collections: Vec<SmartCollection>,
}

impl SmartCollectionStore {
    /// Opens the store in the preferences directory.
    ///
    /// A file that cannot be read is set aside as `smart_collections.toml.bak`
    /// and the store starts empty, so the next change does not overwrite what
    /// the user had.
    #[must_use]
    pub fn load() -> Self {
        let path = app_preferences_dir().join(SMART_COLLECTIONS_FILE);
        Self::load_or_set_aside(path)
    }

    /// Opens the store at `path`, as [`load`](Self::load) does.
    #[must_use]
    pub fn load_or_set_aside(path: PathBuf) -> Self {
        match Self::open(&path) {
            Ok(store) => store,
            Err(error) => {
                tracing::warn!(%error, "smart collections could not be loaded; starting empty");
                let _ = std::fs::rename(&path, path.with_extension("toml.bak"));
                Self { path,
                       collections: Vec::new() }
            }
        }
    }

    /// Opens the store at `path`. A file that does not exist is an empty store.
    ///
    /// # Errors
    ///
    /// Returns [`SmartCollectionError::Read`] or
    /// [`SmartCollectionError::Parse`] if the file exists but cannot be
    /// used.
    pub fn open(path: &Path) -> Result<Self, SmartCollectionError> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(source) => {
                return Err(SmartCollectionError::Read { path: path.to_owned(),
                                                        source });
            }
        };
        let file: StoreFile = toml::from_str(&text).map_err(|source| {
                                                       SmartCollectionError::Parse { path:
                                                                                         path.to_owned(),
                                                                                     source }
                                                   })?;
        Ok(Self { path:        path.to_owned(),
                  collections: file.collections, })
    }

    /// Every smart collection, in the order they were added.
    #[must_use]
    pub fn all(&self) -> &[SmartCollection] {
        &self.collections
    }

    /// The smart collection with `id`, if there is one.
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&SmartCollection> {
        self.collections
            .iter()
            .find(|collection| collection.id == id)
    }

    /// Adds `collection`, giving it a new id, and saves. Returns the id.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written; the collection is
    /// still added in memory.
    pub fn add(&mut self, mut collection: SmartCollection) -> Result<u64, SmartCollectionError> {
        let id = self.collections
                     .iter()
                     .map(|existing| existing.id)
                     .max()
                     .map_or(1, |highest| highest + 1);
        collection.id = id;
        self.collections.push(collection);
        self.save()?;
        Ok(id)
    }

    /// Replaces the collection that has `collection`'s id, and saves. Does
    /// nothing if there is no such collection.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn update(&mut self, collection: SmartCollection) -> Result<(), SmartCollectionError> {
        let Some(existing) = self.collections
                                 .iter_mut()
                                 .find(|existing| existing.id == collection.id)
        else {
            return Ok(());
        };
        *existing = collection;
        self.save()
    }

    /// Removes the collection with `id`, and saves.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn remove(&mut self, id: u64) -> Result<(), SmartCollectionError> {
        self.collections.retain(|collection| collection.id != id);
        self.save()
    }

    /// Writes every collection to the file.
    fn save(&self) -> Result<(), SmartCollectionError> {
        let file = StoreFile { collections: self.collections.clone(), };
        let text = toml::to_string(&file)?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| {
                                               SmartCollectionError::Write { path:
                                                                                 parent.to_owned(),
                                                                             source }
                                           })?;
        }
        std::fs::write(&self.path, text).map_err(|source| {
                                            SmartCollectionError::Write { path: self.path.clone(),
                                                                          source }
                                        })
    }
}

#[cfg(test)]
mod tests;
