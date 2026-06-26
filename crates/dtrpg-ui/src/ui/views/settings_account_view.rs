//! Account settings section: identity display, log-out, and API key reset.

use gpui::{AnyElement, div, px, Entity, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled};

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;

/// Renders the Account settings section.
///
/// `is_authenticated` controls which branch is shown: authenticated identity +
/// log-out actions, or a "not signed in" prompt.
pub fn render_account_section(
    is_authenticated: bool,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> AnyElement {
    if is_authenticated {
        render_authenticated(entity, colors).into_any_element()
    } else {
        render_unauthenticated(colors).into_any_element()
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

fn render_unauthenticated(colors: &ColorTokens) -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let border = colors.border;

    div()
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
        // ── Authenticate prompt ───────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .child(
                            "Sign in with your DriveThruRPG API key to access your library.",
                        ),
                ),
        )
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
