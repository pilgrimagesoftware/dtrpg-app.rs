//! Account settings section: identity display, log-out, and API key sign-in form.

use gpui::{AnyElement, div, px, Entity, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled};
use gpui_component::input::{Input, InputState};

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;

/// Renders the Account settings section.
///
/// `is_authenticated` controls which branch is shown: authenticated identity +
/// log-out actions, or the API key sign-in form.
pub fn render_account_section(
    is_authenticated: bool,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
    api_key_input: Option<Entity<InputState>>,
    sign_in_in_progress: bool,
    sign_in_error: Option<String>,
) -> AnyElement {
    if is_authenticated {
        render_authenticated(entity, colors).into_any_element()
    } else {
        render_unauthenticated(entity, colors, api_key_input, sign_in_in_progress, sign_in_error).into_any_element()
    }
}

// ── Authenticated state ───────────────────────────────────────────────────────

fn render_authenticated(entity: Entity<SettingsController>, colors: &ColorTokens) -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;
    let accent = colors.accent;
    let accent_on = colors.accent_on;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        // ── Identity row ──────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child("Account"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child("Signed in to DriveThruRPG"),
                ),
        )
        // ── Divider ───────────────────────────────────────────────────────
        .child(div().h(px(1.0)).bg(border))
        // ── Actions ───────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(render_logout_button(entity, accent, accent_on)),
        )
}

// ── Unauthenticated state ─────────────────────────────────────────────────────

fn render_unauthenticated(
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
    api_key_input: Option<Entity<InputState>>,
    sign_in_in_progress: bool,
    sign_in_error: Option<String>,
) -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let error_color = colors.error;
    let border = colors.border;
    let accent = colors.accent;
    let accent_on = colors.accent_on;
    let disabled_bg = colors.hover;

    let mut form = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        // ── Status row ────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child("Account"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child("Not signed in to DriveThruRPG"),
                ),
        )
        // ── Divider ───────────────────────────────────────────────────────
        .child(div().h(px(1.0)).bg(border))
        // ── Sign-in form ──────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .child("Sign in with your DriveThruRPG API key to access your library."),
                ),
        );

    if let Some(input_state) = api_key_input {
        let entity_for_btn = entity.clone();
        let btn_bg = if sign_in_in_progress { disabled_bg } else { accent };
        let btn_label = if sign_in_in_progress { "Signing In..." } else { "Sign In" };

        let mut form_section = div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                Input::new(&input_state)
                    .appearance(true)
                    .into_element(),
            );

        if let Some(err) = sign_in_error {
            form_section = form_section.child(
                div()
                    .text_xs()
                    .text_color(error_color)
                    .child(err),
            );
        }

        form_section = form_section.child(
            div()
                .id("sign-in-btn")
                .h(px(34.0))
                .px(px(16.0))
                .rounded(px(8.0))
                .bg(btn_bg)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .on_click(move |_event, _window, cx| {
                    if !sign_in_in_progress {
                        entity_for_btn.update(cx, |ctrl, cx| ctrl.sign_in(cx));
                    }
                })
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(accent_on)
                        .child(btn_label),
                ),
        );

        form = form.child(form_section);
    }

    form
}

fn render_logout_button(
    entity: Entity<SettingsController>,
    accent: gpui::Hsla,
    accent_on: gpui::Hsla,
) -> impl IntoElement + 'static {
    div()
        .id("logout-btn")
        .h(px(34.0))
        .px(px(16.0))
        .rounded(px(8.0))
        .bg(accent)
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .on_click(move |_event, _window, cx| {
            entity.update(cx, |ctrl, cx| ctrl.request_logout(cx));
        })
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(accent_on)
                .child("Log Out"),
        )
}
