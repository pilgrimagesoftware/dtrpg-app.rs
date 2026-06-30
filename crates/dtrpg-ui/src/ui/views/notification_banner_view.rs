//! Notification banner view: renders persistent auth-related notices below the toolbar.

use gpui::{AnyElement, Entity, IntoElement, ParentElement, Styled, div, px};
use gpui_component::alert::Alert;
use gpui_component::button::{Button, ButtonVariants};

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

    let border = colors.border;

    div()
        .flex_none()
        .flex()
        .flex_col()
        .border_b_1()
        .border_color(border)
        .children(notices.into_iter().map(|notice| {
            let (message, action_label) = notice_strings(&notice);
            let kind = notice.kind;

            let auth_entity_action = auth_entity.clone();
            let auth_entity_dismiss = auth_entity.clone();
            let settings_entity = settings_entity.clone();

            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    Alert::warning(format!("notice-alert-{kind:?}"), message)
                        .banner()
                        .flex_1()
                        .on_close(move |_, _, cx| {
                            auth_entity_dismiss.update(cx, |ctrl, cx| {
                                ctrl.dismiss_notice(kind, cx);
                            });
                        }),
                )
                .child(
                    Button::new(format!("notice-action-{kind:?}"))
                        .ghost()
                        .label(action_label)
                        .mr(px(8.0))
                        .on_click(move |_, _, cx| {
                            settings_entity.update(cx, |ctrl, cx| ctrl.open(cx));
                            auth_entity_action.update(cx, |ctrl, cx| {
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
