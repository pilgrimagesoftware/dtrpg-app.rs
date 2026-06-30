//! Notification banner view: renders persistent auth-related notices below the toolbar.

use gpui::{div, px, AnyElement, Entity, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled};

use crate::controllers::auth_state::AuthStateController;
use crate::controllers::settings::SettingsController;
use crate::data::notification::{Notice, NoticeKind};
use crate::data::theme::ColorTokens;

/// Renders the notification banner column.
///
/// Returns an invisible empty element when `notices` is empty so the caller
/// does not need to conditionalize the insertion point.
pub fn render_notification_banner(
    notices: Vec<Notice>,
    auth_entity: Entity<AuthStateController>,
    settings_entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> AnyElement {
    if notices.is_empty() {
        return div().into_any_element();
    }

    let warning_bg = colors.warning_bg;
    let warning_text = colors.warning_text;
    let border_strong = colors.border_strong;
    let text_secondary = colors.text_secondary;

    div()
        .flex_none()
        .flex()
        .flex_col()
        .bg(warning_bg)
        .border_b_1()
        .border_color(border_strong)
        .children(notices.into_iter().map(|notice| {
            let (message, action_label) = notice_strings(&notice);
            let kind = notice.kind;

            let auth_entity_action = auth_entity.clone();
            let auth_entity_dismiss = auth_entity.clone();
            let settings_entity = settings_entity.clone();

            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .px(px(16.0))
                .py(px(8.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(warning_text)
                        .child("⚠"),
                )
                .child(
                    div()
                        .flex_1()
                        .text_sm()
                        .text_color(warning_text)
                        .child(message),
                )
                .child(
                    div()
                        .id(format!("notice-action-{kind:?}"))
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded(px(5.0))
                        .text_xs()
                        .text_color(warning_text)
                        .border_1()
                        .border_color(warning_text)
                        .cursor_pointer()
                        .child(action_label)
                        .on_click(move |_, _, cx| {
                            settings_entity.update(cx, |ctrl, cx| {
                                ctrl.open(cx);
                            });
                            auth_entity_action.update(cx, |ctrl, cx| {
                                ctrl.dismiss_notice(kind, cx);
                            });
                        }),
                )
                .child(
                    div()
                        .id(format!("notice-dismiss-{kind:?}"))
                        .px(px(6.0))
                        .py(px(4.0))
                        .rounded(px(5.0))
                        .text_xs()
                        .text_color(text_secondary)
                        .cursor_pointer()
                        .child("×")
                        .on_click(move |_, _, cx| {
                            auth_entity_dismiss.update(cx, |ctrl, cx| {
                                ctrl.dismiss_notice(kind, cx);
                            });
                        }),
                )
        }))
        .into_any_element()
}

fn notice_strings(notice: &Notice) -> (&'static str, &'static str) {
    match notice.kind {
        NoticeKind::NotSignedIn => ("Not signed in to DriveThruRPG", "Set Up Account"),
        NoticeKind::SessionExpired => (
            "Session expired — sign in again to refresh your library",
            "Sign In Again",
        ),
    }
}
