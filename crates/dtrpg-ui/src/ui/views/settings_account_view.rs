//! Account settings section: identity display, log-out, and email+password
//! sign-in form.

use std::sync::Arc;

use gpui::{
    AnyElement, Entity, Image, ImageFormat, ImageSource, InteractiveElement, IntoElement,
    ParentElement, StatefulInteractiveElement, Styled, div, px,
};
use gpui_component::Sizable;
use gpui_component::avatar::Avatar;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputState};
use rust_i18n::t;

use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::theme::ColorTokens;
use crate::ui::widgets::selectable_text;

/// Renders the Account settings section.
#[allow(clippy::too_many_arguments)]
pub fn render_account_section(auth: &AuthStateSnapshot, entity: Entity<SettingsController>,
                              colors: &ColorTokens, email_input: Option<Entity<InputState>>,
                              password_input: Option<Entity<InputState>>,
                              sign_in_in_progress: bool, sign_in_enabled: bool,
                              sign_in_error: Option<String>, mono_font_family: &str)
                              -> AnyElement {
    if auth.is_logged_in {
        render_authenticated(auth, entity, colors, mono_font_family).into_any_element()
    }
    else {
        render_unauthenticated(entity,
                               colors,
                               email_input,
                               password_input,
                               sign_in_in_progress,
                               sign_in_enabled,
                               sign_in_error).into_any_element()
    }
}

// ── Authenticated state
// ───────────────────────────────────────────────────────

fn render_authenticated(auth: &AuthStateSnapshot, entity: Entity<SettingsController>,
                        colors: &ColorTokens, mono_font_family: &str)
                        -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let border = colors.border;

    let avatar = render_avatar_circle(auth, colors);
    let email_text = auth.email
                         .clone()
                         .or_else(|| auth.api_key_hint.clone())
                         .unwrap_or_else(|| t!("settings.default_account_name").to_string());

    div().flex()
         .flex_col()
         .gap(px(24.0))
         .p(px(24.0))
         // ── Identity row ──────────────────────────────────────────────────
         .child(
                div().flex().items_center().child(
        div().flex()
             .items_center()
             .gap(px(16.0))
             .flex_1()
             .min_w_0()
             .child(avatar)
             .child({
                 let mut col = div().flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .child(div().text_sm()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(text_primary)
                                                .child(t!("settings.account_title")));

                 // Email
                 col = col.child(
                            div()
                                .flex()
                                .items_baseline()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(colors.text_secondary)
                                        .child(t!("settings.email_label")),
                                )
                                .child(
                                    selectable_text("settings-account-email", email_text.clone())
                                        .text_xs()
                                        .font_family(mono_font_family.to_string())
                                        .text_color(colors.text_tertiary),
                                ),
                        );

                 // API Key
                 if let Some(hint) = &auth.api_key_hint {
                     col = col.child(
                                div()
                                    .flex()
                                    .items_baseline()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(colors.text_secondary)
                                            .child(t!("settings.api_key_label")),
                                    )
                                    .child(
                                        selectable_text("settings-account-api-key", hint.clone())
                                            .text_xs()
                                            .font_family(mono_font_family.to_string())
                                            .text_color(colors.text_tertiary),
                                    ),
                            );
                 }
                 col
             }),
    ),
    )
         // ── Divider ───────────────────────────────────────────────────────
         .child(div().h(px(1.0)).bg(border))
         // ── Actions ───────────────────────────────────────────────────────
         .child(div().flex()
                     .flex_col()
                     .gap(px(12.0))
                     .child(render_logout_button(entity)))
}

/// Renders a 56×56 avatar circle: Gravatar image if available, initial letter
/// otherwise.
fn render_avatar_circle(auth: &AuthStateSnapshot, _colors: &ColorTokens) -> AnyElement {
    let avatar = Avatar::new().with_size(gpui_component::Size::Size(px(56.)))
                              .rounded_full();

    if let Some(bytes) = &auth.avatar_bytes {
        let format = if bytes.starts_with(b"\x89PNG") {
            ImageFormat::Png
        }
        else {
            ImageFormat::Jpeg
        };
        let image = Arc::new(Image::from_bytes(format, bytes.as_ref().clone()));
        return avatar.src(ImageSource::Image(image)).into_any_element();
    }

    let name = auth.email
                   .clone()
                   .or_else(|| auth.display_initial.map(|c| c.to_string()))
                   .unwrap_or_else(|| "D".to_string());

    avatar.name(name).into_any_element()
}

// ── Unauthenticated state
// ─────────────────────────────────────────────────────

fn render_unauthenticated(entity: Entity<SettingsController>, colors: &ColorTokens,
                          email_input: Option<Entity<InputState>>,
                          password_input: Option<Entity<InputState>>, sign_in_in_progress: bool,
                          sign_in_enabled: bool, sign_in_error: Option<String>)
                          -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let error_color = colors.error;
    let border = colors.border;
    let accent = colors.accent;
    let accent_on = colors.accent_on;
    let disabled_bg = colors.hover;

    let mut form = div().flex()
                        .flex_col()
                        .gap(px(24.0))
                        .p(px(24.0))
                        .child(div().flex()
                                    .flex_col()
                                    .gap(px(6.0))
                                    .child(div().text_sm()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(text_primary)
                                                .child(t!("settings.account_title")))
                                    .child(div().text_sm()
                                                .text_color(text_secondary)
                                                .child(t!("settings.not_signed_in"))))
                        .child(div().h(px(1.0)).bg(border))
                        .child(div().flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(div().text_xs()
                                                .text_color(text_tertiary)
                                                .child(t!("settings.sign_in_prompt"))));

    if let (Some(email_state), Some(pw_state)) = (email_input, password_input) {
        let entity_for_btn = entity.clone();
        let can_click = sign_in_enabled && !sign_in_in_progress;
        let btn_bg = if can_click { accent } else { disabled_bg };
        let btn_text = if can_click { accent_on } else { text_secondary };
        let btn_label = if sign_in_in_progress {
            t!("settings.sign_in_in_progress")
        }
        else {
            t!("settings.sign_in_button")
        };

        let mut form_section = div().flex()
                                    .flex_col()
                                    .gap(px(10.0))
                                    .child(Input::new(&email_state).appearance(true)
                                                                   .disabled(sign_in_in_progress)
                                                                   .into_element())
                                    .child(Input::new(&pw_state).appearance(true)
                                                                .disabled(sign_in_in_progress)
                                                                .into_element());

        if let Some(err) = sign_in_error {
            form_section = form_section.child(
                selectable_text("settings-sign-in-error", err)
                    .text_xs()
                    .text_color(error_color),
            );
        }

        form_section =
            form_section.child(div().id("sign-in-btn")
                                    .h(px(34.0))
                                    .px(px(16.0))
                                    .rounded(px(8.0))
                                    .bg(btn_bg)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .on_click(move |_event, _window, cx| {
                                        if can_click {
                                            entity_for_btn.update(cx, |ctrl, cx| ctrl.sign_in(cx));
                                        }
                                    })
                                    .child(div().text_sm()
                                                .font_weight(gpui::FontWeight::MEDIUM)
                                                .text_color(btn_text)
                                                .child(btn_label)));

        form = form.child(form_section);
    }

    form
}

fn render_logout_button(entity: Entity<SettingsController>) -> impl IntoElement + 'static {
    Button::new("logout-btn").danger()
                             .label(t!("settings.log_out_button"))
                             .on_click(move |_, _, cx| {
                                 entity.update(cx, |ctrl, cx| ctrl.logout(cx));
                             })
}
