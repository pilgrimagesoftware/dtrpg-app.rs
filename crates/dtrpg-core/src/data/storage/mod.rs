//! Storage root configuration, placement rules, and capacity helpers.

mod config;
mod item_path;
mod paths;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_settings;

pub mod placement;
pub mod roots;
mod usage;

pub use config::{DEFAULT_MAX_CONCURRENT_DOWNLOADS, DEFAULT_ROOT_ID, StorageConfig, StorageError};
pub use item_path::resolved_file_path;
pub use paths::{
    available_space_mb, available_space_mb_at, collection_dir, publisher_dir,
    sanitize_path_component, validate_writable,
};
pub use roots::{RootHealth, RootKind};
pub use usage::{locations_of_titles, titles_per_root};
