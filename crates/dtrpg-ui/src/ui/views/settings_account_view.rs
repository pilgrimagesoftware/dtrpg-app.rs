//! Account settings section: identity display, log-out, and API key sign-in form.

use std::sync::Arc;

use gpui::{AnyElement, div, img, px, Entity, Image, ImageFormat, ImageSource, InteractiveElement,
    IntoElement, ObjectFit, ParentElement, StatefulInteractiveElement, Styled, StyledImage};
use gpui_component::input::{Input, InputState};

use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::theme::ColorTokens;

/// Renders the Account settings section.
pub fn render_account_section(
    is_authenticated: bool,
    auth: &AuthStateSnapshot,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
    api_key_input: Option<Entity<InputState>>,
    sign_in_in_progress: bool,
    sign_in_error: Option<String>,
) -> AnyElement {
    if is_authenticated {
        render_authenticated(auth, entity, colors).into_any_element()
    } else {
        render_unauthenticated(entity, colors, api_key_input, sign_in_in_progress, sign_in_error).into_any_element()
    }
}

// ── Authenticated state ───────────────────────────────────────────────────────

fn render_authenticated(auth: &AuthStateSnapshot, entity: Entity<SettingsController>, colors: &ColorTokens) -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;
    let accent = colors.accent;
    let accent_on = colors.accent_on;

    let avatar = render_avatar_circle(auth, colors);
    let email_text = auth.email.clone().unwrap_or_else(|| "DriveThruRPG Account".to_string());

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        // ── Identity row ──────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(16.0))
                .child(avatar)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
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
                                .child(email_text),
                        ),
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

/// Renders a 56×56 avatar circle: Gravatar image if available, initial letter otherwise.
fn render_avatar_circle(auth: &AuthStateSnapshot, colors: &ColorTokens) -> AnyElement {
    let size = px(56.0);

    if let Some(bytes) = &auth.avatar_bytes {
        let format = if bytes.starts_with(b"\x89PNG") { ImageFormat::Png } else { ImageFormat::Jpeg };
        let image = Arc::new(Image::from_bytes(format, bytes.as_ref().clone()));
        return div()
            .size(size)
            .rounded_full()
            .overflow_hidden()
            .child(
                img(ImageSource::Image(image))
                    .size(size)
                    .object_fit(ObjectFit::Cover),
            )
            .into_any_element();
    }

    let initial = auth.display_initial
        .map(|c| c.to_string())
        .unwrap_or_else(|| "?".to_string());

    div()
        .size(size)
        .rounded_full()
        .bg(colors.accent_soft)
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .text_xl()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(colors.accent)
                .child(initial),
        )
        .into_any_element()
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
        .child(div().h(px(1.0)).bg(border))
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
