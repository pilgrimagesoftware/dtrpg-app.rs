//! DriveThruRPG Rust desktop baseline application entry point.
//!
//! This binary boots a GPUI window with baseline shell state and stubbed services
//! defined by the `implement-rust-library-baseline-ui-stubs` OpenSpec change.

mod app;
mod services;
mod view_models;

use app::shell::{AppCommand, AppShell, AppShellState, SessionPresentationState};
use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};
use services::stub::{StubLibraryService, StubMode};
use view_models::library::{LibraryPaneState, LibraryViewModel};

/// Root GPUI view for the Rust baseline desktop shell.
struct BaselineDesktopRoot {
    shell: AppShell,
}

impl BaselineDesktopRoot {
    /// Creates baseline shell state with deterministic stub data.
    fn new() -> Self {
        let service = StubLibraryService::new(StubMode::Seeded);
        let library = LibraryViewModel::new(Box::new(service));

        let mut shell = AppShell::new(
            AppShellState {
                session: SessionPresentationState::SignedIn,
                library: LibraryPaneState::Loading,
                selected_item_id: None,
                status_message: "Loading your library…".to_string(),
            },
            library,
        );

        shell.dispatch(AppCommand::LoadLibrary);

        if let Some(first) = shell.first_item_id() {
            shell.dispatch(AppCommand::SelectLibraryItem(first));
        }

        Self { shell }
    }

    fn render_lines(&self) -> Vec<String> {
        let state = self.shell.state();
        let mut lines = vec![
            "DriveThruRPG – Baseline Rust UI (Stubbed)".to_string(),
            format!("Session: {:?}", state.session),
            format!("Library pane: {:?}", state.library),
            format!("Status: {}", state.status_message),
            "".to_string(),
            "Library items:".to_string(),
        ];

        for item in self.shell.items() {
            lines.push(format!("• {} — {}", item.title, item.publisher));
        }

        lines.push("".to_string());

        if let Some(selected) = self.shell.selected_item() {
            lines.push("Selected item detail:".to_string());
            lines.push(format!("Title: {}", selected.title));
            lines.push(format!("Publisher: {}", selected.publisher));
            lines.push(format!("Summary: {}", selected.summary));
        } else {
            lines.push("Selected item detail: none".to_string());
        }

        lines
    }
}

impl Render for BaselineDesktopRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let lines = self.render_lines();

        div()
            .size_full()
            .bg(rgb(0x1f2937))
            .text_color(rgb(0xf9fafb))
            .p_4()
            .flex()
            .flex_col()
            .gap_1()
            .children(lines.into_iter().map(|line| div().child(line)))
    }
}

/// Starts the baseline GPUI desktop application in stubbed mode.
fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(960.0), px(680.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| BaselineDesktopRoot::new()),
        )
        .expect("failed to open baseline desktop window");
    });
}
