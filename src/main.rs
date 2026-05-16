//! DriveThruRPG Rust desktop application entry point.
//!
//! This binary boots the GPUI desktop shell with SDK-backed library data.

mod app;
mod services;
mod ui;
mod view_models;

fn main() {
    ui::library::launch();
}
