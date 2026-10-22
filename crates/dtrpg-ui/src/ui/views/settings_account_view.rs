//! Account settings section: who is signed in and the session's details, with
//! log out; or the email and password sign-in form. Laid out like the other
//! settings pages: a heading, then bordered cards of rows.

use std::sync::Arc;

use gpui::{
    AnyElement, Entity, Image, ImageFormat, ImageSource, IntoElement, ParentElement, Styled, div,
    px,
};
use gpui_kit::component::avatar::Avatar;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Disableable, Sizable};
use rust_i18n::t;

use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::theme::ColorTokens;
use crate::ui::views::settings_card::{card, card_row, section_heading};
use crate::ui::widgets::selectable_text;
use crate::util::datetime::format_absolute;

/// Renders the Account settings section.
#[allow(clippy::too_many_arguments)]
pub fn render_account_section(auth: &AuthStateSnapshot, entity: Entity<SettingsController>,
                              colors: &ColorTokens, email_input: Option<Entity<InputState>>,
                              password_input: Option<Entity<InputState>>,
                              sign_in_in_progress: bool, sign_in_enabled: bool,
                              sign_in_error: Option<String>, mono_font_family: &str)
                              -> AnyElement {
    let page = div().flex()
                    .flex_col()
                    .gap(px(24.0))
                    .p(px(24.0))
                    .child(div().text_sm()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(colors.text_primary)
                                .child(t!("settings.account_title")));
    if auth.is_logged_in {
        page.child(identity_card(auth, entity, colors))
            .child(session_card(auth, colors, mono_font_family))
            .into_any_element()
    }
    else {
        page.child(sign_in_card(entity,
                                colors,
                                SignInForm { email_input,
                                             password_input,
                                             in_progress: sign_in_in_progress,
                                             enabled: sign_in_enabled,
                                             error: sign_in_error }))
            .into_any_element()
    }
}

// ── Signed in
// ─────────────────────────────────────────────────────────────────

/// Who is signed in: the avatar, the account's name and a log out button.
fn identity_card(auth: &AuthStateSnapshot, entity: Entity<SettingsController>,
                 colors: &ColorTokens)
                 -> impl IntoElement + 'static {
    let name = auth.email
                   .clone()
                   .or_else(|| auth.api_key_hint.clone())
                   .unwrap_or_else(|| t!("settings.default_account_name").to_string());
    card(colors).child(card_row(true, colors)
                           .child(render_avatar_circle(auth, colors))
                           .child(div().flex_1()
                                       .min_w_0()
                                       .flex()
                                       .flex_col()
                                       .gap(px(2.0))
                                       .child(selectable_text("settings-account-email", name)
                                                  .text_sm()
                                                  .font_weight(gpui::FontWeight::SEMIBOLD)
                                                  .text_color(colors.text_primary)
                                                  .truncate())
                                       .child(div().text_xs()
                                                   .text_color(colors.text_secondary)
                                                   .child(t!("settings.account_signed_in"))))
                           .child(Button::new("logout-btn").danger()
                                                           .label(t!("settings.log_out_button"))
                                                           .on_click(move |_, _, cx| {
                                                               entity.update(cx, |ctrl, cx| ctrl.logout(cx));
                                                           })))
}

/// The session's details: the API key, when the token was acquired and when it
/// expires, each a row with its name on the left and its value on the right.
fn session_card(auth: &AuthStateSnapshot, colors: &ColorTokens, mono_font_family: &str)
                -> impl IntoElement + 'static {
    let mut rows: Vec<(&'static str, String, String)> = Vec::new();
    if let Some(hint) = &auth.api_key_hint {
        rows.push(("settings-account-api-key",
                   t!("settings.api_key_label").to_string(),
                   hint.clone()));
    }
    if let Some(status) = auth.token_status {
        rows.push(("settings-account-token-acquired",
                   t!("settings.token_acquired_label").to_string(),
                   format_absolute(status.acquired_at)));
        if let Some(expires_at) = status.expires_at {
            rows.push(("settings-account-token-expires",
                       t!("settings.token_expires_label").to_string(),
                       format_absolute(expires_at)));
        }
    }
    let card_body = rows.into_iter().enumerate().fold(card(colors), |card, (index, (id, label, value))| {
        card.child(card_row(index == 0, colors).child(div().flex_1()
                                                           .text_sm()
                                                           .font_weight(gpui::FontWeight::SEMIBOLD)
                                                           .text_color(colors.text_primary)
                                                           .child(label))
                                               .child(selectable_text(id, value).text_xs()
                                                                                .font_family(mono_font_family.to_string())
                                                                                .text_color(colors.text_secondary)))
    });
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.account_session_title").to_string(), colors))
         .child(card_body)
}

/// Renders a 56×56 avatar circle: Gravatar image if available, initial letter
/// otherwise.
fn render_avatar_circle(auth: &AuthStateSnapshot, _colors: &ColorTokens) -> AnyElement {
    const AVATAR_SIZE: gpui::Pixels = px(56.);

    if let Some(bytes) = &auth.avatar_bytes {
        let format = if bytes.starts_with(b"\x89PNG") {
            ImageFormat::Png
        }
        else {
            ImageFormat::Jpeg
        };
        let image = Arc::new(Image::from_bytes(format, bytes.as_ref().clone()));
        // flex_none() keeps this from being resized by an ancestor flex
        // layout's basis/shrink computation - without it, Avatar's own
        // explicit pixel sizing can still get squished in a tight flex
        // context (observed in the title-bar account button).
        return div().flex_none()
                    .child(Avatar::new().with_size(gpui_kit::component::Size::Size(AVATAR_SIZE))
                                        .rounded_full()
                                        .src(ImageSource::Image(image)))
                    .into_any_element();
    }

    let name = auth.email
                   .clone()
                   .or_else(|| auth.display_initial.map(|c| c.to_string()))
                   .unwrap_or_else(|| "D".to_string());

    div().flex_none()
         .child(Avatar::new().with_size(gpui_kit::component::Size::Size(AVATAR_SIZE))
                             .rounded_full()
                             .name(name))
         .into_any_element()
}

// ── Signed out
// ────────────────────────────────────────────────────────────────

/// What the sign-in form needs.
struct SignInForm {
    email_input:    Option<Entity<InputState>>,
    password_input: Option<Entity<InputState>>,
    in_progress:    bool,
    enabled:        bool,
    error:          Option<String>,
}

/// The sign-in form: a note on what signing in is for, the email and password
/// fields, why the last attempt failed if it did, and the sign-in button.
fn sign_in_card(entity: Entity<SettingsController>, colors: &ColorTokens, form: SignInForm)
                -> impl IntoElement + 'static {
    let can_click = form.enabled && !form.in_progress;
    let button_label = if form.in_progress {
        t!("settings.sign_in_in_progress")
    }
    else {
        t!("settings.sign_in_button")
    };
    let mut card_body = card(colors).child(card_row(true, colors).child(
        div().flex_1()
             .flex()
             .flex_col()
             .gap(px(2.0))
             .child(div().text_sm()
                         .font_weight(gpui::FontWeight::SEMIBOLD)
                         .text_color(colors.text_primary)
                         .child(t!("settings.not_signed_in")))
             .child(div().text_xs()
                         .text_color(colors.text_secondary)
                         .child(t!("settings.sign_in_prompt"))),
    ));
    for input in [form.email_input, form.password_input].into_iter()
                                                        .flatten()
    {
        card_body = card_body.child(card_row(false, colors).child(
            div().flex_1().child(Input::new(&input).appearance(true).disabled(form.in_progress)),
        ));
    }
    if let Some(error) = form.error {
        card_body = card_body.child(card_row(false, colors).child(
            selectable_text("settings-sign-in-error", error).text_xs().text_color(colors.error),
        ));
    }
    card_body = card_body.child(card_row(false, colors).justify_end().child(
        Button::new("sign-in-btn").primary()
                                  .label(button_label)
                                  .disabled(!can_click)
                                  .on_click(move |_, _, cx| {
                                      entity.update(cx, |ctrl, cx| ctrl.sign_in(cx));
                                  }),
    ));
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.sign_in_title").to_string(), colors))
         .child(card_body)
}

#[cfg(test)]
mod tests {
    use rust_i18n::t;

    #[test]
    fn the_texts_of_the_settings_pages_laid_out_as_cards_resolve() {
        for key in ["settings.account_session_title",
                    "settings.account_signed_in",
                    "settings.sign_in_title",
                    "settings.file_openers_section",
                    "settings.file_opener_add",
                    "settings.appearance_typography_title",
                    "settings.appearance_text_size_note"]
        {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }
}
