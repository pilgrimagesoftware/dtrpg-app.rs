//! "Report a Feature/Bug" window: a kind selector (Bug/Feature Request), a
//! subject, and a description field, with read-only diagnostic context
//! (version, commit, build date, target) appended automatically, an option to
//! attach the application log, and screenshots to name in the report.
//! Submission files the issue through `gh` when it is installed and signed in,
//! and otherwise opens a pre-filled GitHub "new issue" page in the browser;
//! see `dtrpg_core::report`.
//!
//! Its own window rather than a modal dialog, matching Knot's "Report an
//! Issue…" window (see `openspec/specs/bug-reporting` in the Knot repo).

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use dtrpg_core::platform::{Platform, PlatformIntegration as _};
use dtrpg_core::report::{self, Outcome, Report, SystemGh};
use gpui::prelude::*;
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, MouseButton, Render, Size, TitlebarOptions,
    Window, WindowBounds, WindowHandle, WindowOptions, div, point, px,
};
use gpui_kit::component::Disableable as _;
use gpui_kit::component::Root;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::radio::Radio;
use gpui_kit::component::scroll::ScrollableElement;
use rust_i18n::t;

use crate::build_info;
use crate::data::theme::LibriTheme;
use crate::ui::widgets::{selectable_text, suppress_selection_outside_text};

const WINDOW_WIDTH: f32 = 440.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportKind {
    Bug,
    Feature,
}

impl ReportKind {
    /// Prefix applied to the composed issue title — the kind is encoded as
    /// plain text because applying a label needs triage permission.
    fn title_prefix(self) -> &'static str {
        match self {
            ReportKind::Bug => "[Bug]",
            ReportKind::Feature => "[Feature Request]",
        }
    }
}

/// Where a submission stands, shown under the form.
#[derive(Clone, Default, PartialEq, Eq)]
enum Status {
    #[default]
    Idle,
    Submitting,
    /// `gh` could not file the issue, with its message.
    Failed(String),
}

/// Root view for the "Report a Feature/Bug" window.
pub struct ReportFeatureBugWindowView {
    kind:         Rc<RefCell<ReportKind>>,
    subject:      Entity<InputState>,
    description:  Entity<TextareaState>,
    attach_log:   Rc<RefCell<bool>>,
    screenshots:  Rc<RefCell<Vec<PathBuf>>>,
    status:       Rc<RefCell<Status>>,
    focus_handle: FocusHandle,
}

impl ReportFeatureBugWindowView {
    /// Constructs the window's root view and focuses it immediately so
    /// Escape closes the window without an extra click first.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        let subject = cx.new(|cx| {
            InputState::new(window, cx).placeholder(t!("report.subject_placeholder").to_string())
        });
        let description = cx.new(|cx| {
                                TextareaState::new(window, cx)
                .rows(6)
                .placeholder(t!("report.description_placeholder").to_string())
                            });
        Self { kind: Rc::new(RefCell::new(ReportKind::Bug)),
               subject,
               description,
               attach_log: Rc::default(),
               screenshots: Rc::default(),
               status: Rc::default(),
               focus_handle }
    }
}

impl Focusable for ReportFeatureBugWindowView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Opens the "Report a Feature/Bug" window. Callers are responsible for
/// tracking the returned handle and bringing an already-open window forward
/// instead of calling this again — see
/// `LibraryRootView::show_report_feature_bug`.
#[allow(clippy::expect_used)]
pub fn open_report_feature_bug_window(cx: &mut App) -> WindowHandle<Root> {
    let window_size = Size { width:  px(WINDOW_WIDTH),
                             height: px(WINDOW_HEIGHT), };
    let (window, _view) = gpui_kit::open_window(
        WindowOptions {
            // Same chrome pattern as `open_settings_window`: hide the
            // native title text but keep the macOS traffic lights, and
            // disable minimize/resize since the content scrolls its own
            // overflow.
            titlebar: Some(TitlebarOptions {
                appears_transparent: true,
                traffic_light_position: Some(point(px(12.0), px(6.0))),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            is_resizable: false,
            is_minimizable: false,
            ..Default::default()
        },
        cx,
        move |window, cx| cx.new(|cx| ReportFeatureBugWindowView::new(window, cx)),
    )
    .expect("failed to open report feature/bug window");
    // `gpui_kit::open_window` wraps the view in a `Root`, so the window's
    // root entity type is always `Root` — see `open_settings_window`'s
    // matching comment.
    window.downcast::<Root>()
          .expect("open_window's root entity is always Root")
}

/// `shared-report-feature-bug`'s submission-validation rule: Report is
/// reachable only once both fields carry more than whitespace.
fn fields_filled(subject: &str, description: &str) -> bool {
    !subject.trim().is_empty() && !description.trim().is_empty()
}

impl Render for ReportFeatureBugWindowView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.global::<LibriTheme>().colors.clone();
        let current_kind = *self.kind.borrow();
        let enabled = fields_filled(&self.subject.read(cx).value(),
                                    &self.description.read(cx).value());

        let label = |text: String| {
            div().text_xs()
                 .text_color(colors.text_secondary)
                 .child(text)
        };

        let kind_row = {
            let kind_bug = self.kind.clone();
            let kind_feature = self.kind.clone();
            div()
                .flex()
                .gap(px(16.0))
                .child(
                    Radio::new("report-kind-bug")
                        .label(t!("report.kind_bug").to_string())
                        .checked(current_kind == ReportKind::Bug)
                        .on_click(move |_, window, _cx| {
                            *kind_bug.borrow_mut() = ReportKind::Bug;
                            window.refresh();
                        }),
                )
                .child(
                    Radio::new("report-kind-feature")
                        .label(t!("report.kind_feature").to_string())
                        .checked(current_kind == ReportKind::Feature)
                        .on_click(move |_, window, _cx| {
                            *kind_feature.borrow_mut() = ReportKind::Feature;
                            window.refresh();
                        }),
                )
        };

        let diagnostics =
            div().flex()
                 .flex_col()
                 .gap_1()
                 .p_2()
                 .rounded(px(6.0))
                 .bg(colors.surface_alt)
                 .child(selectable_text("report-diagnostics-version",
                                        t!("report.diagnostics_version",
                                           version = env!("CARGO_PKG_VERSION")).to_string()))
                 .child(selectable_text("report-diagnostics-build",
                                        t!("report.diagnostics_build",
                                           date = build_info::BUILD_DATE,
                                           commit = build_info::GIT_HASH).to_string()))
                 .child(selectable_text("report-diagnostics-platform",
                                        t!("report.diagnostics_platform",
                                           target = build_info::TARGET).to_string()));

        let status = self.status.borrow().clone();
        let submitting = status == Status::Submitting;
        let buttons = {
            let inputs = (self.subject.clone(), self.description.clone());
            let choices = (self.kind.clone(), self.attach_log.clone(), self.screenshots.clone());
            let status_cell = self.status.clone();
            div().flex()
                 .justify_end()
                 .gap_2()
                 .child(Button::new("report-cancel").label(t!("report.cancel").to_string())
                                                    .ghost()
                                                    .on_click(|_, window, _cx| {
                                                        window.remove_window();
                                                    }))
                 .child(Button::new("report-submit").label(t!("report.submit").to_string())
                                                    .primary()
                                                    .disabled(!enabled || submitting)
                                                    .on_click(move |_, window, cx| {
                                                        let subject =
                                                            inputs.0.read(cx).value().to_string();
                                                        let description =
                                                            inputs.1.read(cx).value().to_string();
                                                        if !fields_filled(&subject, &description) {
                                                            return;
                                                        }
                                                        let kind = *choices.0.borrow();
                                                        let attach_log = *choices.1.borrow();
                                                        let screenshots =
                                                            choices.2.borrow().clone();
                                                        *status_cell.borrow_mut() =
                                                            Status::Submitting;
                                                        window.refresh();
                                                        start_submit(
                                window,
                                cx,
                                Request {
                                    title: format!("{} {subject}", kind.title_prefix()),
                                    description,
                                    attach_log,
                                    screenshots,
                                },
                                status_cell.clone(),
                            );
                                                    }))
        };

        let options = {
            let attach_log = self.attach_log.clone();
            let screenshots = self.screenshots.clone();
            let chosen = screenshots.borrow().len();
            let log_on = *attach_log.borrow();
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    Checkbox::new("report-attach-log")
                        .label(t!("report.attach_log").to_string())
                        .checked(log_on)
                        .on_click(move |_, window, _cx| {
                            let on = !*attach_log.borrow();
                            *attach_log.borrow_mut() = on;
                            window.refresh();
                        }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            Button::new("report-add-screenshots")
                                .outline()
                                .label(t!("report.attach_screenshots").to_string())
                                .on_click(move |_, window, cx| {
                                    pick_screenshots(window, cx, screenshots.clone());
                                }),
                        )
                        .children((chosen > 0).then(|| {
                            div()
                                .text_xs()
                                .text_color(colors.text_secondary)
                                .child(t!("report.screenshots_chosen", count = chosen).to_string())
                        })),
                )
        };

        let status_line = match status {
            Status::Idle => None,
            Status::Submitting => {
                Some((t!("report.submitting").to_string(), colors.text_secondary))
            }
            Status::Failed(error) => {
                Some((t!("report.failed", error = error).to_string(), colors.warning_text))
            }
        };

        // Reserves vertical clearance for the macOS traffic light buttons
        // (see `open_report_feature_bug_window`'s `appears_transparent`) and
        // renders the window title — same pattern as the settings window's
        // `settings-drag-region`.
        let drag_region = div().id("report-drag-region")
                               .h(px(28.0))
                               .flex_none()
                               .flex()
                               .items_center()
                               .justify_center()
                               .text_base()
                               .line_height(px(28.0))
                               .font_weight(gpui::FontWeight::BOLD)
                               .text_color(colors.text_primary)
                               .child(t!("report.title").to_string())
                               .on_mouse_down(MouseButton::Left, |_, window, _| {
                                   window.start_window_move();
                               });

        div().id("report-window-root")
             .track_focus(&self.focus_handle)
             .on_mouse_down(MouseButton::Left, suppress_selection_outside_text)
             .on_key_down(move |event, window, _cx| {
                 if event.keystroke.key == "escape" {
                     window.remove_window();
                 }
             })
             .size_full()
             .bg(colors.surface)
             .flex()
             .flex_col()
             .child(drag_region)
             // `overflow_y_scrollbar()` wraps its element in `Scrollable`,
             // whose `render` only copies the `size` style field onto its
             // own outer wrapper — sizing utilities set here would be
             // silently dropped. The flex/scroll container lives on the
             // plain outer div instead; the inner div just needs
             // `size_full()` to fill it, which `Scrollable` already applies
             // on its own.
             .child(div().flex_1()
                         .min_h_0()
                         .child(div().flex()
                                     .flex_col()
                                     .size_full()
                                     .overflow_y_scrollbar()
                                     .gap_2()
                                     .px_4()
                                     .py_2()
                                     .child(kind_row)
                                     .child(label(t!("report.subject_label").to_string()))
                                     .child(Input::new(&self.subject).w_full())
                                     .child(label(t!("report.description_label").to_string()))
                                     .child(Textarea::new(&self.description).w_full().h(px(120.0)))
                                     .child(options)
                                     .child(diagnostics)
                                     .children(status_line.map(|(text, color)| {
                                                              div().text_xs()
                                                                   .text_color(color)
                                                                   .child(text)
                                                          }))))
             .child(div().px_4().pb_4().child(buttons))
    }
}

/// What the Report button gathered, ready to hand to a background thread.
struct Request {
    title:       String,
    description: String,
    attach_log:  bool,
    screenshots: Vec<PathBuf>,
}

/// Files the report off the UI thread (`gh` and the log read both block) and
/// closes the window, or shows why `gh` could not file it and keeps it open.
fn start_submit(window: &mut Window, cx: &mut App, request: Request, status: Rc<RefCell<Status>>) {
    window.to_async(cx)
          .spawn(async move |cx| {
              let outcome = cx.background_executor()
                              .spawn(async move { file_report(request) })
                              .await;
              cx.update(|window, _cx| {
                    if let Outcome::FileFailed(error) = outcome {
                        *status.borrow_mut() = Status::Failed(error);
                        window.refresh();
                    }
                    else {
                        window.remove_window();
                    }
                })
                .ok();
          })
          .detach();
}

/// Builds the report and submits it. Blocks.
fn file_report(request: Request) -> Outcome {
    let diagnostics = format!("**Version:** {}\n**Commit:** {}\n**Build Date:** {}\n**Target:** {}",
                              env!("CARGO_PKG_VERSION"),
                              build_info::GIT_HASH,
                              build_info::BUILD_DATE,
                              build_info::TARGET);
    let log_tail =
        request.attach_log
               .then(|| report::read_log().unwrap_or_else(|| "(no log found)".to_owned()));
    let report = Report { title: request.title,
                          description: request.description,
                          diagnostics,
                          log_tail,
                          screenshots: request.screenshots };
    report::submit(&report,
                   &SystemGh::locate(),
                   |url| open::that(url).is_ok(),
                   |path| Platform.reveal(path).is_ok())
}

/// Lets the user pick image files and adds them to `screenshots`. Async, for
/// the reason given on `settings_file_openers_view::pick_app_and_begin_add`.
fn pick_screenshots(window: &mut Window, cx: &mut App, screenshots: Rc<RefCell<Vec<PathBuf>>>) {
    window.to_async(cx)
          .spawn(async move |cx| {
              let picked = rfd::AsyncFileDialog::new()
                .add_filter(
                    t!("report.screenshot_filter").to_string(),
                    dtrpg_core::constants::REPORT_SCREENSHOT_EXTENSIONS,
                )
                .pick_files()
                .await
                .unwrap_or_default();
              report::add_screenshots(&mut screenshots.borrow_mut(),
                                      picked.into_iter().map(|handle| handle.path().to_path_buf()));
              cx.update(|window, _cx| window.refresh()).ok();
          })
          .detach();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_is_disabled_until_both_fields_are_non_whitespace() {
        assert!(!fields_filled("", ""));
        assert!(!fields_filled("   ", "a description"));
        assert!(!fields_filled("a subject", "   "));
        assert!(!fields_filled("", "a description"));
        assert!(fields_filled("a subject", "a description"));
    }

    #[test]
    fn each_kind_has_its_own_title_prefix() {
        assert_eq!(ReportKind::Bug.title_prefix(), "[Bug]");
        assert_eq!(ReportKind::Feature.title_prefix(), "[Feature Request]");
    }
}
