//! Login view: API key input form with sign-in button and auth state feedback.

use gpui::{
    div, px, rgb, AppContext, Context, Entity, IntoElement,
    ParentElement, Render, Styled, Window,
};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::Disableable;

use crate::controllers::login::{LoginController, LoginState};
use crate::data::events::LoginStateChanged;
use crate::ui::app::open_library_window;

/// Renders the login form window contents.
pub struct LoginView {
    login: Entity<LoginController>,
    input: Entity<InputState>,
}

impl LoginView {
    /// Creates the login view, wires input events to the controller, and subscribes
    /// to `LoginStateChanged::Succeeded` to transition to the library window.
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        login: Entity<LoginController>,
    ) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .masked(true)
                .placeholder("DriveThruRPG API key")
        });

        // Update the draft in the controller whenever the text changes.
        cx.subscribe(&input, |this, state, event: &InputEvent, cx| {
            if let InputEvent::Change = event {
                let value = state.read(cx).value().to_string();
                this.login.update(cx, |ctrl, cx| ctrl.set_api_key(value, cx));
            }
        })
        .detach();

        // On press-enter in the input, attempt submit.
        let login_enter = login.clone();
        cx.subscribe(&input, move |_this, _state, event: &InputEvent, cx| {
            if let InputEvent::PressEnter { .. } = event {
                login_enter.update(cx, |ctrl, cx| ctrl.submit(cx));
            }
        })
        .detach();

        // When login succeeds, open the library window and close this one.
        cx.subscribe(&login, |_this, _ctrl, event: &LoginStateChanged, cx| {
            if let LoginStateChanged::Succeeded(tokens) = event {
                open_library_window(tokens.clone(), cx);
                cx.with_window(cx.entity_id(), |window, _cx| window.remove_window());
            }
        })
        .detach();

        // Re-render on any login state change.
        cx.subscribe(&login, |_this, _ctrl, _event: &LoginStateChanged, cx| {
            cx.notify();
        })
        .detach();

        Self { login, input }
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ctrl = self.login.read(cx);
        let draft_empty = ctrl.api_key_draft().trim().is_empty();
        let in_progress = matches!(ctrl.state(), LoginState::InProgress);
        let error_msg = if let LoginState::Error(msg) = ctrl.state() {
            Some(msg.clone())
        } else {
            None
        };
        let login = self.login.clone();

        let surface = rgb(0x1C1C1E);
        let text_primary = rgb(0xF2F2F7);
        let text_secondary = rgb(0xAEAEB2);
        let text_error = rgb(0xFF453A);
        let border = rgb(0x38383A);

        let button = {
            let login_btn = login.clone();
            Button::new("sign-in")
                .label("Sign In")
                .primary()
                .loading(in_progress)
                .disabled(draft_empty || in_progress)
                .on_click(move |_event, _window, cx| {
                    login_btn.update(cx, |ctrl, cx| ctrl.submit(cx));
                })
        };

        div()
            .size_full()
            .bg(surface)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(360.0))
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .p(px(24.0))
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(border)
                    // Title
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(text_primary)
                            .child("Sign In to DriveThruRPG"),
                    )
                    // Subtitle
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary)
                            .child("Enter your API key to access your library."),
                    )
                    // API key input
                    .child(Input::new(&self.input))
                    // Error message
                    .child(if let Some(msg) = error_msg {
                        div()
                            .text_xs()
                            .text_color(text_error)
                            .child(msg)
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    // Sign-in button
                    .child(button),
            )
    }
}
