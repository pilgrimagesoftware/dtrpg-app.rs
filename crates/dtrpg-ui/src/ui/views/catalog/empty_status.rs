use std::sync::Arc;

use dtrpg_core::data::enums::GroupBy;
use dtrpg_core::data::enums::*;
use dtrpg_core::util::filter::SidebarFilter;
use gpui::prelude::*;
use gpui::{
    AnyElement, Entity, FontWeight, Hsla, IntoElement, ParentElement, SharedString, Styled, div, px,
};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{ContextMenuExt, PopupMenuItem};
use gpui_kit::component::spinner::Spinner;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{Sizable, Size};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;

/// The translation key of the line under "No titles match", which says what
/// the user can change: the search, the section, or both.
pub(super) fn no_matches_hint_key(has_search: bool, has_section: bool) -> &'static str {
    match (has_search, has_section) {
        (true, true) => "catalog.try_clear_search_and_section",
        (true, false) => "catalog.try_clear_search",
        (false, _) => "catalog.try_different_section",
    }
}

/// Renders the screen shown when the catalog has titles but none match the
/// search and section: what happened, what to change, and a button that goes
/// back to all titles when a section other than that one is chosen.
pub(super) fn render_no_matches_state(search_query: &str, filter: &SidebarFilter,
                                      entity: Entity<LibraryController>, text_color: Hsla)
                                      -> impl IntoElement + 'static {
    let has_section = *filter != SidebarFilter::AllTitles;
    let hint = t!(no_matches_hint_key(!search_query.is_empty(), has_section));

    div().flex()
         .flex_col()
         .items_center()
         .justify_center()
         .h_full()
         .gap(px(12.0))
         .child(div().text_color(text_color)
                     .child(Icon::new(IconName::Search).with_size(Size::Large)))
         .child(div().text_sm()
                     .text_color(text_color)
                     .child(t!("catalog.no_titles_match")))
         .child(div().text_xs().text_color(text_color).child(hint))
         .when(has_section, |column| {
             column.child(Button::new("show-all-titles")
                              .ghost()
                              .outline()
                              .label(t!("catalog.show_all_titles").to_string())
                              .on_click(move |_, _, cx| {
                                  entity.update(cx, |ctrl, cx| {
                                            ctrl.set_filter(SidebarFilter::AllTitles, cx);
                                        });
                              }))
         })
}

pub(super) fn render_library_empty_state(text_color: Hsla) -> impl IntoElement + 'static {
    div().flex()
         .flex_col()
         .items_center()
         .justify_center()
         .h_full()
         .gap(px(12.0))
         .child(div().text_2xl().text_color(text_color).child("⊘"))
         .child(div().text_sm()
                     .text_color(text_color)
                     .child(t!("catalog.library_empty")))
}

// ── Group header
// ──────────────────────────────────────────────────────────────

pub(super) fn render_group_header(title: &str, kind: GroupBy, count: usize,
                                  colors: &ColorTokens, entity: Entity<LibraryController>)
                                  -> AnyElement {
    let text_primary = colors.text_primary;
    let text_tertiary = colors.text_tertiary;
    let title = title.to_string();
    let publisher = title.clone();
    let header = div().id(SharedString::from(format!("group-header-{title}")))
                      .flex()
                      .items_center()
                      .gap(px(8.0))
                      .py(px(10.0))
                      .child(div().text_sm()
                                  .font_weight(FontWeight::SEMIBOLD)
                                  .text_color(text_primary)
                                  .child(title))
                      .child(div().text_xs()
                                  .text_color(text_tertiary)
                                  .child(count.to_string()));
    // "Download all" is for a publisher. A tag has nothing to offer here yet,
    // so its header has no menu rather than an empty one.
    if kind != GroupBy::Publisher {
        return header.into_any_element();
    }
    header.context_menu(move |menu, _, _| {
              let entity = entity.clone();
              let publisher = publisher.clone();
              menu.item(PopupMenuItem::new(t!("catalog.publisher_download_all")).on_click(
                  move |_, _, cx| {
                      entity.update(cx, |ctrl, cx| {
                                ctrl.request_download_all_for_publisher(&publisher, cx)
                            });
                  },
              ))
          })
          .into_any_element()
}

// ── Status glyph
// ──────────────────────────────────────────────────────────────

/// Renders the download-status glyph for `id`.
///
/// While `activity` is not idle the glyph is replaced by a spinner with its
/// own tooltip rather than showing a downloaded/cloud state that is about to
/// change or may be stale: [`ItemActivity::Downloading`] when one of the
/// entry's files is queued or downloading, [`ItemActivity::Verifying`] while
/// a file-presence check (`verify-downloaded-status-against-disk`) is in
/// flight for it.
pub(super) fn render_status(id: &Arc<str>, status: ItemStatus, offloaded: bool,
                            activity: ItemActivity, colors: &ColorTokens)
                            -> AnyElement {
    match activity {
        ItemActivity::Downloading => {
            return render_downloading_indicator(id, colors).into_any_element();
        }
        ItemActivity::Verifying => {
            return render_verifying_indicator(id, colors).into_any_element();
        }
        ItemActivity::Idle => {}
    }
    let accent = colors.accent;
    let text_tertiary = colors.text_tertiary;
    let (glyph, tooltip_text) = match status {
        ItemStatus::Downloaded if offloaded => {
            (div().text_xs()
                  .text_color(colors.warning_text)
                  .child("☁")
                  .into_any_element(),
             t!("detail.tooltip_offloaded_download").to_string())
        }
        ItemStatus::Downloaded => (div().size(px(7.0))
                                        .rounded_full()
                                        .bg(accent)
                                        .into_any_element(),
                                   t!("detail.status_on_device").to_string()),
        ItemStatus::Cloud => (div().text_xs()
                                   .text_color(text_tertiary)
                                   .child("☁")
                                   .into_any_element(),
                              t!("detail.status_in_cloud").to_string()),
    };
    div().id(SharedString::from(format!("status-{id}")))
         .flex_none()
         .child(glyph)
         .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
         .into_any_element()
}

/// Renders an accent-colored spinner with a "Downloading…" tooltip for a
/// catalog entry with a file queued or downloading (see
/// `LibraryController::is_downloading`). Accent-colored, unlike the muted
/// verifying spinner, so a download in progress stands out in the catalog.
pub(super) fn render_downloading_indicator(id: &Arc<str>, colors: &ColorTokens)
                                           -> impl IntoElement + 'static + use<> {
    let elem_id: Arc<str> = Arc::from(format!("downloading-{id}"));
    div().id(elem_id)
         .flex_none()
         .child(Spinner::new().with_size(Size::XSmall).color(colors.accent))
         .tooltip(|window, cx| {
             Tooltip::new(t!("detail.item_status_downloading").to_string()).build(window, cx)
         })
}

/// Renders a small tinted spinner with a "checking download status" tooltip
/// for a catalog entry whose file-presence verification is currently in
/// flight (`LibraryController::is_verifying_downloads`, see
/// `verify-downloaded-status-against-disk`). Distinct from
/// [`render_checking_indicator`]'s untinted spinner (a network-bound
/// availability check) so the two kinds of in-flight check are never
/// visually conflated.
pub(super) fn render_verifying_indicator(id: &Arc<str>, colors: &ColorTokens)
                                         -> impl IntoElement + 'static + use<> {
    let elem_id: Arc<str> = Arc::from(format!("verifying-{id}"));
    div().id(elem_id)
         .flex_none()
         .child(Spinner::new().with_size(Size::XSmall)
                              .color(colors.text_tertiary))
         .tooltip(|window, cx| {
             Tooltip::new(t!("detail.tooltip_verifying_download").to_string()).build(window, cx)
         })
}

#[cfg(test)]
mod tests {
    use super::no_matches_hint_key;

    #[test]
    fn the_hint_names_whatever_the_user_can_change() {
        assert_eq!(no_matches_hint_key(true, true),
                   "catalog.try_clear_search_and_section");
        assert_eq!(no_matches_hint_key(true, false), "catalog.try_clear_search");
        assert_eq!(no_matches_hint_key(false, true),
                   "catalog.try_different_section");
    }

    #[test]
    fn with_no_search_and_no_section_the_hint_still_points_somewhere() {
        assert_eq!(no_matches_hint_key(false, false),
                   "catalog.try_different_section");
    }
}
