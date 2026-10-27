use std::path::Path;
use std::sync::Arc;

use dtrpg_core::data::library::LibraryItem;
use gpui::{
    AnyElement, App, Entity, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px,
};
use gpui_kit::component::Disableable;
use gpui_kit::component::IconName;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::tooltip::Tooltip;
use rust_i18n::t;

use super::contents_tab::*;
use super::cover::*;
use super::item_tier::*;
use super::other_details::*;
use crate::controllers::inspector::InspectorController;
use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;
use crate::ui::widgets::selectable_text;
use crate::util::datetime::{format_absolute, format_relative};
use crate::view_models::inspector::{InspectorPreviewState, InspectorSummaryState, InspectorTab};

/// Renders a stateful div showing a relative date label, with a tooltip that
/// reveals the absolute human-readable date/time on hover.
///
/// `slot` disambiguates the element id (e.g. `"added"`, `"updated"`) so
/// multiple date rows on the same item don't collide.
pub(super) fn render_relative_date_value(item_id: &str, slot: &str, ts: i64,
                                         value_font_family: &str)
                                         -> AnyElement {
    let relative = format_relative(ts);
    let absolute = format_absolute(ts);
    let id = SharedString::from(format!("detail-{slot}-{item_id}"));
    div().id(id)
         .child(selectable_text(SharedString::from(format!("detail-{slot}-{item_id}-text")),
                                relative).font_family(value_font_family.to_string()))
         .tooltip(move |window, cx| Tooltip::new(absolute.clone()).build(window, cx))
         .into_any_element()
}

pub(super) fn inspector_tab_label(tab: InspectorTab) -> String {
    match tab {
        InspectorTab::Overview => t!("detail.inspector_overview_tab").to_string(),
        InspectorTab::Files => t!("detail.inspector_files_tab").to_string(),
        InspectorTab::Contents => t!("detail.inspector_contents_tab").to_string(),
        InspectorTab::Summary => t!("detail.inspector_summary_tab").to_string(),
        InspectorTab::Cover => t!("detail.inspector_cover_tab").to_string(),
        InspectorTab::OtherDetails => t!("detail.inspector_other_details_tab").to_string(),
    }
}

/// Renders the row of inspector inspector, plus, when `pop_out` is given, a
/// control at the right end that moves the inspector into its own window.
/// Renders the row of inspector tabs. The Files tab carries a badge with the
/// number of files.
pub(super) fn render_inspector_tab_bar(item: &LibraryItem, entity: Entity<LibraryController>,
                                       active: InspectorTab, colors: &ColorTokens)
                                       -> AnyElement {
    let tabs = [InspectorTab::Overview,
                InspectorTab::Files,
                InspectorTab::Contents,
                InspectorTab::Summary,
                InspectorTab::Cover,
                InspectorTab::OtherDetails];
    let file_count = item.files.len();
    div().id("inspector-tabs")
         .flex()
         .items_center()
         .gap(px(4.0))
         .border_b_1()
         .border_color(colors.border)
         .px(px(16.0))
         .children(tabs.into_iter().map(|tab| {
             let click_entity = entity.clone();
             let click_id = Arc::clone(&item.id);
             let active_color = if tab == active { colors.accent } else { colors.border };
             let badge = (tab == InspectorTab::Files && file_count > 0).then(|| {
                 div().px(px(6.0))
                      .rounded_full()
                      .bg(colors.surface_alt)
                      .text_xs()
                      .text_color(colors.text_secondary)
                      .child(file_count.to_string())
             });
             div().id(SharedString::from(format!("inspector-tab-{tab:?}")))
                 .flex()
                 .items_center()
                 .gap(px(6.0))
                 .px(px(10.0))
                 .py(px(8.0))
                 .text_xs()
                 .text_color(if tab == active { colors.text_primary } else { colors.text_secondary })
                 .border_b_2()
                 .border_color(active_color)
                 .cursor_pointer()
                 .on_click(move |_, _, cx| {
                     click_entity.update(cx, |controller, cx| {
                         controller.set_inspector_tab(Arc::clone(&click_id), tab, cx);
                     });
                 })
                 .child(inspector_tab_label(tab))
                 .children(badge)
         }))
         .into_any_element()
}

pub(super) struct InspectorRenderContext<'a> {
    pub(super) storage_root_path: &'a Path,
    pub(super) entity:            Entity<LibraryController>,
    pub(super) inspector:         &'a Entity<InspectorController>,
    pub(super) colors:            &'a ColorTokens,
    pub(super) window:            &'a mut Window,
    pub(super) cx:                &'a mut App,
}

pub(super) fn render_inspector_preview(state: &InspectorPreviewState, entry_id: &Arc<str>,
                                       entity: Entity<LibraryController>, colors: &ColorTokens)
                                       -> Option<AnyElement> {
    let body: AnyElement = match state {
        InspectorPreviewState::None => return None,
        InspectorPreviewState::Loading(name) => {
            div().text_sm()
                 .text_color(colors.text_secondary)
                 .child(t!("detail.inspector_preview_loading", name = name.clone()).to_string())
                 .into_any_element()
        }
        InspectorPreviewState::Unavailable(name) => {
            div().text_sm()
                 .text_color(colors.text_secondary)
                 .child(t!("detail.inspector_preview_unavailable", name = name.clone()).to_string())
                 .into_any_element()
        }
        InspectorPreviewState::Text { name, contents } => {
            div().flex()
                 .flex_col()
                 .gap(px(6.0))
                 .child(div().text_xs()
                             .text_color(colors.text_tertiary)
                             .child(name.clone()))
                 .child(div().id("inspector-preview-body")
                             .max_h(px(240.0))
                             .overflow_y_scroll()
                             .rounded_md()
                             .bg(colors.surface_alt)
                             .p(px(8.0))
                             .text_xs()
                             .font_family("monospace".to_string())
                             .text_color(colors.text_primary)
                             .child(contents.clone()))
                 .into_any_element()
        }
    };
    Some(div()
             .id("inspector-preview-panel")
             .flex()
             .flex_col()
             .gap(px(8.0))
             .rounded_md()
             .border_1()
             .border_color(colors.border)
             .p(px(10.0))
             .child(body)
             .child(Button::new("inspector-preview-close")
                        .ghost()
                        .child(t!("detail.inspector_preview_close").to_string())
                        .on_click({
                            let close_entity = entity.clone();
                            let close_id = Arc::clone(entry_id);
                            move |_, _, cx| {
                                close_entity.update(cx, |controller, cx| {
                                    controller.clear_inspector_preview(Arc::clone(&close_id), cx);
                                });
                            }
                        }))
             .into_any_element())
}

/// Renders the Files tab's "Verify on disk" button, which checks the entry's
/// files against disk and corrects what the catalog says about them.
fn render_verify_on_disk(item_id: &Arc<str>, verifying: bool, entity: Entity<LibraryController>)
                         -> AnyElement {
    let id = Arc::clone(item_id);
    div().px(px(20.0))
         .pb(px(16.0))
         .child(Button::new("inspector-verify-on-disk").ghost()
                                                       .outline()
                                                       .icon(IconName::RefreshCw)
                                                       .label(t!("detail.verify_on_disk_button").to_string())
                                                       .disabled(verifying)
                                                       .loading(verifying)
                                                       .on_click(move |_, _, cx| {
                                                           entity.update(cx, |controller, cx| {
                                                                     controller.verify_item_on_disk(&id, cx);
                                                                 });
                                                       }))
         .into_any_element()
}

/// Renders the Summary tab: the stored summary with the model that wrote it,
/// when, and its length, or a line saying there is none.
fn render_inspector_summary(state: &InspectorSummaryState, colors: &ColorTokens) -> AnyElement {
    let body: AnyElement = match state {
        InspectorSummaryState::Loading => {
            div().text_sm()
                 .text_color(colors.text_secondary)
                 .child(t!("detail.inspector_summary_loading").to_string())
                 .into_any_element()
        }
        InspectorSummaryState::Missing => {
            div().text_sm()
                 .text_color(colors.text_secondary)
                 .child(t!("detail.inspector_summary_empty").to_string())
                 .into_any_element()
        }
        InspectorSummaryState::Loaded(summary) => {
            div().flex()
                 .flex_col()
                 .gap(px(8.0))
                 .child(selectable_text("inspector-summary-text", summary.text.clone())
                            .text_sm()
                            .text_color(colors.text_primary))
                 .child(div().flex()
                             .items_center()
                             .gap(px(8.0))
                             .text_xs()
                             .text_color(colors.text_tertiary)
                             .child(summary.model.clone())
                             .child(format_absolute(summary.created_at))
                             .child(t!("detail.inspector_summary_tokens",
                                       count = summary.token_count).to_string()))
                 .into_any_element()
        }
    };
    div().flex()
         .flex_col()
         .gap(px(12.0))
         .p(px(20.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(inspector_tab_label(InspectorTab::Summary)))
         .child(body)
         .into_any_element()
}

pub(super) fn render_secondary_inspector_tab(item: &LibraryItem, tab: InspectorTab,
                                             context: InspectorRenderContext<'_>)
                                             -> AnyElement {
    let contents = context.entity
                          .read(context.cx)
                          .inspector_contents_for(&item.id);
    let content: AnyElement = match tab {
        InspectorTab::Files => {
            let verifying = context.entity
                                   .read(context.cx)
                                   .is_verifying_downloads(&item.id);
            // Padded like the other tabs, so the list and the file's details
            // do not touch the inspector's edge.
            div().flex()
                 .flex_col()
                 .gap(px(12.0))
                 .p(px(16.0))
                 .child(render_item_tier(item,
                                         context.storage_root_path,
                                         context.entity.clone(),
                                         context.inspector,
                                         context.colors,
                                         context.window,
                                         context.cx))
                 .child(render_verify_on_disk(&item.id, verifying, context.entity.clone()))
                 .into_any_element()
        }
        InspectorTab::Contents => {
            let _filter_input = context.entity
                                       .read(context.cx)
                                       .inspector_filter_input_for(&item.id);
            render_inspector_contents(&contents,
                                      &item.id,
                                      context.entity.clone(),
                                      context.colors,
                                      context.cx)
        }
        InspectorTab::Summary => {
            let state = context.entity
                               .read(context.cx)
                               .inspector_summary_for(&item.id);
            render_inspector_summary(&state, context.colors)
        }
        InspectorTab::Cover => {
            render_cover_tab(item, context.entity.clone(), context.colors, context.cx)
        }
        InspectorTab::OtherDetails => render_other_details_tab(item, context.colors, context.cx),
        InspectorTab::Overview => div().into_any_element(),
    };

    content
}

#[cfg(test)]
mod tests {

    use crate::ui::views::detail_panel::metadata_table::value_or_dash;

    #[test]
    fn value_or_dash_passes_through_non_empty_value() {
        assert_eq!(value_or_dash("Pathfinder"), "Pathfinder");
    }

    #[test]
    fn value_or_dash_falls_back_to_em_dash_on_empty_string() {
        assert_eq!(value_or_dash(""), "\u{2014}");
    }

    #[test]
    fn value_or_dash_falls_back_to_em_dash_on_whitespace_only() {
        assert_eq!(value_or_dash("   "), "\u{2014}");
    }
}
