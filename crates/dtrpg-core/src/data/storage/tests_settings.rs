//! Unit tests for the storage settings that are plain on/off or number fields.

#![allow(clippy::unwrap_used)] // Test fixtures intentionally use unwrap for concise assertions.

use std::path::PathBuf;

use super::config::*;
use crate::data::constants::RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS;

#[test]
fn max_concurrent_downloads_defaults_to_three() {
    let cfg = StorageConfig { override_path:                None,
                              max_concurrent_downloads:     DEFAULT_MAX_CONCURRENT_DOWNLOADS,
                              create_collections:           false,
                              check_free_space:             true,
                              recently_updated_window_days: RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS,
                              extra_roots:                  Vec::new(),
                              rules:                        Vec::new(),
                              next_root_id:                 1,
                              default_root_id:              0, };
    assert_eq!(cfg.max_concurrent_downloads(), 3);
}

#[test]
fn missing_max_concurrent_downloads_field_deserializes_to_default() {
    let file: StorageConfigFile = toml::from_str("").unwrap();
    assert_eq!(file.max_concurrent_downloads,
               DEFAULT_MAX_CONCURRENT_DOWNLOADS);
}

#[test]
fn create_collections_defaults_to_false() {
    let cfg = StorageConfig { override_path:                None,
                              max_concurrent_downloads:     DEFAULT_MAX_CONCURRENT_DOWNLOADS,
                              create_collections:           false,
                              check_free_space:             true,
                              recently_updated_window_days: RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS,
                              extra_roots:                  Vec::new(),
                              rules:                        Vec::new(),
                              next_root_id:                 1,
                              default_root_id:              0, };
    assert!(!cfg.create_collections());
}

#[test]
fn missing_create_collections_field_deserializes_to_default() {
    let file: StorageConfigFile = toml::from_str("").unwrap();
    assert!(!file.create_collections);
}

#[test]
fn create_collections_round_trips_through_toml_serialization() {
    let cfg = StorageConfigFile { root_path:                    None,
                                  max_concurrent_downloads:     DEFAULT_MAX_CONCURRENT_DOWNLOADS,
                                  create_collections:           true,
                                  check_free_space:             true,
                                  recently_updated_window_days:
                                      RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS,
                                  extra_roots:                  Vec::new(),
                                  rules:                        Vec::new(),
                                  next_root_id:                 1,
                                  default_root_id:              0, };
    let text = toml::to_string(&cfg).unwrap();
    let reloaded: StorageConfigFile = toml::from_str(&text).unwrap();
    assert!(reloaded.create_collections);
}

#[test]
fn free_space_is_checked_unless_turned_off() {
    let file: StorageConfigFile = toml::from_str("max_concurrent_downloads = 3").unwrap();

    assert!(file.check_free_space);
    assert!(StorageConfig::for_test(PathBuf::from("/tmp/dtrpg")).check_free_space());
}

#[test]
fn free_space_check_can_be_stored_off() {
    let file: StorageConfigFile =
        toml::from_str("max_concurrent_downloads = 3\ncheck_free_space = false").unwrap();

    assert!(!file.check_free_space);
}
