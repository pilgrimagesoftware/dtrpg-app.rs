//! The Summaries section of the Agent & MCP page: how summaries are written and
//! how much of the month's allowance they have used.

use dtrpg_core::data::summary_settings::{SummaryProviderKind, SummarySettingsFile};
use gpui::prelude::*;
use gpui::{AnyElement, Entity, SharedString, div, px};
use gpui_kit::component::button::Button;
use gpui_kit::component::input::Input;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings_card::{card, card_row, section_heading};
use crate::ui::views::settings_native_view::switch_row;
use crate::view_models::summary_display::{
    SummaryPage, ceiling_choices, ceiling_note, grouped, key_hint, model_choices,
};

/// Renders the Summaries heading and its card: provider, model, monthly token
/// ceiling and the switch for summarizing on download.
pub fn render_summaries(entity: &Entity<SettingsController>, settings: &SummarySettingsFile,
                        page: &SummaryPage, colors: &ColorTokens)
                        -> impl IntoElement + 'static + use<> {
    let auto_entity = entity.clone();
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("agent.summaries_heading").to_string(), colors))
         .child(card(colors).child(provider_row(entity, settings.provider, page, colors))
                            .child(model_row(entity, &settings.model, colors))
                            .child(ceiling_row(entity,
                                               settings.monthly_token_ceiling,
                                               page.tokens_used,
                                               colors))
                            .child(card_row(false, colors).child(div().flex_1().child(switch_row(
                                "summary-auto",
                                t!("agent.summary_auto_title").to_string(),
                                t!("agent.summary_auto_note").to_string(),
                                settings.auto_summarize_on_download,
                                colors,
                                move |checked, cx| {
                                    auto_entity.update(cx, |ctrl, cx| {
                                                    ctrl.set_summary_auto(checked, cx);
                                                });
                                },
                            )))))
}

/// A row of the card: a title and a note on the left, a control on the right.
fn setting_row(first: bool, title: String, note: String, control: AnyElement,
               colors: &ColorTokens)
               -> impl IntoElement {
    card_row(first, colors).child(div().flex_1()
                                       .min_w_0()
                                       .flex()
                                       .flex_col()
                                       .child(div().text_sm()
                                                   .font_weight(gpui::FontWeight::SEMIBOLD)
                                                   .text_color(colors.text_primary)
                                                   .child(title))
                                       .child(div().text_xs()
                                                   .text_color(colors.text_secondary)
                                                   .child(note)))
                           .child(control)
}

/// The name of a provider, as the menu and the button show it.
fn provider_label(provider: SummaryProviderKind) -> String {
    match provider {
        SummaryProviderKind::Anthropic => t!("agent.summary_provider_anthropic").to_string(),
        SummaryProviderKind::Local => t!("agent.summary_provider_local").to_string(),
    }
}

/// The width of the API key field.
const KEY_FIELD_WIDTH: f32 = 220.0;

fn provider_row(entity: &Entity<SettingsController>, current: SummaryProviderKind,
                page: &SummaryPage, colors: &ColorTokens)
                -> impl IntoElement {
    let entity = entity.clone();
    let menu = Button::new("summary-provider").outline()
                                              .label(provider_label(current))
                                              .dropdown_caret(true)
                                              .dropdown_menu(move |menu, _, _| {
                                                  [SummaryProviderKind::Anthropic,
                                                   SummaryProviderKind::Local].into_iter()
                                                                              .fold(menu, |menu, provider| {
                                                      let entity = entity.clone();
                                                      menu.item(PopupMenuItem::new(provider_label(provider))
                                                                    .checked(provider == current)
                                                                    .on_click(move |_, _, cx| {
                                                                        entity.update(cx, |ctrl, cx| {
                                                                                  ctrl.set_summary_provider(provider, cx);
                                                                              });
                                                                    }))
                                                  })
                                              });
    // The key is only asked for when Anthropic writes the summaries. It sits to
    // the left of the chooser, and the hint under the title says where the key
    // comes from.
    let anthropic = current == SummaryProviderKind::Anthropic;
    let key_input = page.key_input.as_ref().filter(|_| anthropic);
    let note = if anthropic {
        format!("{} {}",
                t!("agent.summary_provider_note"),
                key_hint(page.key))
    }
    else {
        t!("agent.summary_provider_note").to_string()
    };
    let control =
        div().flex()
             .items_center()
             .gap(px(8.0))
             .children(key_input.map(|input| div().w(px(KEY_FIELD_WIDTH)).child(Input::new(input))))
             .child(menu);
    setting_row(true,
                t!("agent.summary_provider_title").to_string(),
                note,
                control.into_any_element(),
                colors)
}

fn model_row(entity: &Entity<SettingsController>, current: &str, colors: &ColorTokens)
             -> impl IntoElement {
    let entity = entity.clone();
    let choices = model_choices(current);
    let selected = current.to_owned();
    let menu = Button::new("summary-model").outline()
                                           .label(SharedString::from(current.to_owned()))
                                           .dropdown_caret(true)
                                           .dropdown_menu(move |menu, _, _| {
                                               choices.iter().fold(menu, |menu, model| {
                                                   let entity = entity.clone();
                                                   let chosen = model.clone();
                                                   menu.item(PopupMenuItem::new(model.clone())
                                                                 .checked(*model == selected)
                                                                 .on_click(move |_, _, cx| {
                                                                     let chosen = chosen.clone();
                                                                     entity.update(cx, |ctrl, cx| {
                                                                               ctrl.set_summary_model(chosen, cx);
                                                                           });
                                                                 }))
                                               })
                                           });
    setting_row(false,
                t!("agent.summary_model_title").to_string(),
                t!("agent.summary_model_note").to_string(),
                menu.into_any_element(),
                colors)
}

fn ceiling_row(entity: &Entity<SettingsController>, current: u64, used: u64,
               colors: &ColorTokens)
               -> impl IntoElement {
    let entity = entity.clone();
    let choices = ceiling_choices(current);
    let menu = Button::new("summary-ceiling").outline()
                                             .label(grouped(current))
                                             .dropdown_caret(true)
                                             .dropdown_menu(move |menu, _, _| {
                                                 choices.iter().fold(menu, |menu, ceiling| {
                                                     let entity = entity.clone();
                                                     let chosen = *ceiling;
                                                     menu.item(PopupMenuItem::new(grouped(chosen))
                                                                   .checked(chosen == current)
                                                                   .on_click(move |_, _, cx| {
                                                                       entity.update(cx, |ctrl, cx| {
                                                                                 ctrl.set_summary_ceiling(chosen, cx);
                                                                             });
                                                                   }))
                                                 })
                                             });
    setting_row(false,
                t!("agent.summary_ceiling_title").to_string(),
                ceiling_note(used, current),
                menu.into_any_element(),
                colors)
}
