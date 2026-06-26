//! Account settings section: identity display, log-out, and API key reset.
//!
//! `is_authenticated` is stubbed as `true` until `secure-credential-storage` wires
//! real auth state into `SettingsSnapshot`.

use gpui::{AnyElement, div, px, Entity, IntoElement, ParentElement, Styled};

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;

/// Renders the Account settings section.
///
/// `is_authenticated` controls which branch is shown: authenticated identity +
/// log-out actions, or a "not signed in" prompt. Pass the value from
/// `SettingsSnapshot::is_authenticated` once that field exists; for now it is
/// stubbed to `true`.
pub fn render_account_section(
    _entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> AnyElement {
    // Stubbed until `secure-credential-storage` supplies real auth state.
    let is_authenticated = true;

    if is_authenticated {
        render_authenticated(colors).into_any_element()
    } else {
        render_unauthenticated(colors).into_any_element()
    }
}

// ── Authenticated state ───────────────────────────────────────────────────────

fn render_authenticated(colors: &ColorTokens) -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
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
                .child(
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .child(
                            "Log Out and Reset API Key are available once the credential store \
                             is connected (see secure-credential-storage).",
                        ),
                )
                .child(render_action_button("Log Out", accent, accent_on))
                .child(render_action_button("Reset API Key", accent, accent_on)),
        )
}

// ── Unauthenticated state ─────────────────────────────────────────────────────

fn render_unauthenticated(colors: &ColorTokens) -> impl IntoElement + 'static {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let border = colors.border;
    let accent = colors.accent;
    let accent_on = colors.accent_on;

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
                )
                .child(render_action_button("Sign In with API Key…", accent, accent_on)),
        )
}

fn render_action_button(
    label: &'static str,
    accent: gpui::Hsla,
    accent_on: gpui::Hsla,
) -> impl IntoElement + 'static {
    div()
        .h(px(34.0))
        .px(px(16.0))
        .rounded(px(8.0))
        .bg(accent)
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(accent_on)
                .child(label),
        )
}
