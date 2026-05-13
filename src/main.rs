//! DriveThruRPG Rust desktop application entry point.
//!
//! This binary boots the GPUI desktop shell with stubbed library data.

mod app;
mod services;
mod ui;
mod view_models;

fn main() {
    ui::library::launch();
}
