//! DriveThruRPG Rust desktop application entry point.
//!
//! This binary boots the GPUI desktop shell with SDK-backed library data.

pub mod ui {
    pub mod actions;
    pub mod app;
    pub mod library;
    pub mod views;
    pub mod windows;
}
pub mod controllers;
pub mod credentials;
pub mod data;
pub mod models;
pub mod services;
pub mod util;
pub mod view_models;
