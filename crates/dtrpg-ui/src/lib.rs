//! DriveThruRPG Rust desktop application entry point.
//!
//! This binary boots the GPUI desktop shell with SDK-backed library data.

pub mod ui {
    pub mod app;
    pub mod library;
    pub mod views;
    pub mod windows;
}
pub mod models;
pub mod data;
pub mod controllers;
pub mod view_models;
