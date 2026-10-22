//! The Agent & MCP page: whether agents may use Libri, what they may do, which
//! tools they have, how to connect one, and what they have done.

use dtrpg_core::data::mcp_access::{McpScope, McpTool};
use gpui::prelude::*;
use gpui::{ClipboardItem, Entity, SharedString, div, px};
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use gpui_kit::component::tag::Tag;
use gpui_kit::component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings_native_view::switch_row;
use crate::ui::widgets::{selectable_note, selectable_text, small_caps_text};
use crate::view_models::agent_display::{
    AGENT_CONFIG_JSON, AgentPageData, exposed_tools, scope_label, summary_label, time_label,
    tool_description,
};

/// How a scope reads: its title and note, and how risky it is.
struct ScopeText {
    scope: McpScope,
    title: &'static str,
    note:  &'static str,
    risk:  &'static str,
}

const SCOPES: [ScopeText; 3] = [ScopeText { scope: McpScope::Read,
                                            title: "agent.scope_read_title",
                                            note:  "agent.scope_read_note",
                                            risk:  "agent.risk_low", },
                                ScopeText { scope: McpScope::Change,
                                            title: "agent.scope_change_title",
                                            note:  "agent.scope_change_note",
                                            risk:  "agent.risk_medium", },
                                ScopeText { scope: McpScope::DownloadAndSpend,
                                            title: "agent.scope_spend_title",
                                            note:  "agent.scope_spend_note",
                                            risk:  "agent.risk_high", }];

/// The risk badge beside a scope's title: green, amber or red, so a wider
/// scope reads as the riskier choice.
fn risk_badge(scope: McpScope, label: String) -> Tag {
    let tag = match scope {
        McpScope::Read => Tag::success(),
        McpScope::Change => Tag::warning(),
        McpScope::DownloadAndSpend => Tag::danger(),
    };
    tag.outline().child(label)
}

/// A section heading in the page's small-caps label style.
fn heading(text: String, colors: &ColorTokens, label_font: &str) -> impl IntoElement {
    div().font_family(label_font.to_string())
         .text_sm()
         .text_color(colors.text_primary)
         .child(small_caps_text(text))
}

/// The three scope switches, each with its risk badge.
fn scope_rows(entity: &Entity<SettingsController>, data: &AgentPageData, colors: &ColorTokens)
              -> impl IntoElement {
    div().flex()
         .flex_col()
         .gap(px(16.0))
         .children(SCOPES.iter().map(|text| {
                                    let scope = text.scope;
                                    let entity = entity.clone();
                                    let checked = match scope {
                                        McpScope::Read => data.settings.read,
                                        McpScope::Change => data.settings.change,
                                        McpScope::DownloadAndSpend => {
                                            data.settings.download_and_spend
                                        }
                                    };
                                    div().flex()
                  .flex_col()
                  .gap(px(4.0))
                  .child(div().flex()
                              .items_center()
                              .justify_between()
                              .child(div().flex()
                                          .items_center()
                                          .gap(px(8.0))
                                          .child(div().text_sm()
                                                      .font_weight(gpui::FontWeight::SEMIBOLD)
                                                      .text_color(colors.text_primary)
                                                      .child(t!(text.title).to_string()))
                                          .child(risk_badge(scope, t!(text.risk).to_string())))
                              .child(Switch::new(SharedString::from(format!("agent-scope-{scope}")))
                                         .checked(checked)
                                         .on_click(move |checked, _, cx| {
                                             let checked = *checked;
                                             entity.update(cx, |ctrl, cx| {
                                                       ctrl.set_mcp_scope(scope, checked, cx);
                                                   });
                                         })))
                  .child(selectable_note(SharedString::from(format!("agent-scope-note-{scope}")),
                                         t!(text.note).to_string(),
                                         colors))
                                }))
}

/// The tools: each with its scope, what it does, and a switch.
fn tools_table(entity: &Entity<SettingsController>, data: &AgentPageData, colors: &ColorTokens)
               -> impl IntoElement {
    let head = |label: &str| TableHead::new().child(small_caps_text(t!(label).to_string()));
    let header = TableHeader::new().child(TableRow::new().child(head("agent.col_tool"))
                                                         .child(head("agent.col_scope"))
                                                         .child(head("agent.col_what"))
                                                         .child(head("agent.col_on")));
    let rows = McpTool::ALL.into_iter().map(|tool| {
        let entity = entity.clone();
        let on = !data.settings.disabled_tools.contains(tool.name());
        let tooltip = t!("agent.tool_toggle_tooltip", tool = tool.name()).to_string();
        TableRow::new().child(TableCell::new().child(div().font_family("monospace")
                                                          .text_sm()
                                                          .text_color(colors.text_primary)
                                                          .child(tool.name())))
                       .child(TableCell::new().child(Tag::secondary().outline()
                                                                     .child(scope_label(tool.scope()))))
                       .child(TableCell::new().child(div().text_sm()
                                                          .text_color(colors.text_secondary)
                                                          .child(tool_description(tool))))
                       .child(TableCell::new().child(
            div().id(SharedString::from(format!("agent-tool-{}", tool.name())))
                 .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
                 .child(Switch::new(SharedString::from(format!("agent-tool-switch-{}",
                                                               tool.name()))).checked(on)
                                                                              .on_click(move |checked, _, cx| {
                                                                                  let checked = *checked;
                                                                                  entity.update(cx, |ctrl, cx| {
                                                                                            ctrl.set_mcp_tool(tool, checked, cx);
                                                                                        });
                                                                              })),
        ))
    });
    Table::new().child(header)
                .child(TableBody::new().children(rows))
}

/// The config an agent needs, in a box, with a button that copies it.
fn connect_block(colors: &ColorTokens, mono_font: &str) -> impl IntoElement {
    div().flex()
         .flex_col()
         .gap(px(8.0))
         .child(selectable_note("agent-connect-note", t!("agent.connect_note").to_string(), colors))
         .child(div().p(px(12.0))
                     .rounded(px(6.0))
                     .border_1()
                     .border_color(colors.border)
                     .bg(colors.surface_alt)
                     .font_family(mono_font.to_string())
                     .child(selectable_text("agent-config", AGENT_CONFIG_JSON).text_xs()
                                                                               .text_color(colors.text_primary)))
         .child(div().flex().child(Button::new("agent-copy-config")
                                       .outline()
                                       .label(t!("agent.copy_config").to_string())
                                       .on_click(|_, _, cx| {
                                           cx.write_to_clipboard(ClipboardItem::new_string(AGENT_CONFIG_JSON.to_owned()));
                                       })))
}

/// The latest calls, newest first.
fn activity_rows(data: &AgentPageData, colors: &ColorTokens) -> impl IntoElement {
    if data.activity.is_empty() {
        return div().text_xs()
                    .text_color(colors.text_secondary)
                    .child(t!("agent.activity_empty").to_string())
                    .into_any_element();
    }
    div().flex()
         .flex_col()
         .gap(px(6.0))
         .children(data.activity.iter().map(|entry| {
                                           div().flex()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child(div().text_xs()
                                                            .text_color(colors.text_tertiary)
                                                            .child(time_label(entry)))
                                                .child(Tag::secondary().child(entry.tool.clone()))
                                                .child(div().flex_1()
                                                            .truncate()
                                                            .text_sm()
                                                            .text_color(if entry.denied {
                                                                            colors.error
                                                                        }
                                                                        else {
                                                                            colors.text_primary
                                                                        })
                                                            .child(summary_label(entry)))
                                                .child(risk_badge(entry.scope,
                                                                  scope_label(entry.scope)))
                                       }))
         .into_any_element()
}

/// Renders the Agent & MCP page.
pub fn render_agent_section(entity: Entity<SettingsController>, data: AgentPageData,
                            colors: &ColorTokens, label_font: &str, mono_font: &str)
                            -> impl IntoElement + 'static + use<> {
    let enable_entity = entity.clone();
    let refresh_entity = entity.clone();
    let enable_note = if data.settings.enabled {
        t!("agent.enable_note_on",
           exposed = exposed_tools(&data.settings),
           all = McpTool::ALL.len()).to_string()
    }
    else {
        t!("agent.enable_note_off").to_string()
    };
    div().flex()
         .flex_col()
         .gap(px(24.0))
         .p(px(24.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(t!("agent.title").to_string()))
         .child(selectable_note("agent-intro", t!("agent.intro").to_string(), colors))
         .child(switch_row("agent-enabled",
                           t!("agent.enable_title").to_string(),
                           enable_note,
                           data.settings.enabled,
                           colors,
                           move |checked, cx| {
                               enable_entity.update(cx, |ctrl, cx| {
                                                ctrl.set_mcp_enabled(checked, cx);
                                            });
                           }))
         .child(div().h(px(1.0)).bg(colors.border))
         .child(heading(t!("agent.scopes_heading").to_string(), colors, label_font))
         .child(scope_rows(&entity, &data, colors))
         .child(heading(t!("agent.tools_heading").to_string(), colors, label_font))
         .child(tools_table(&entity, &data, colors))
         .child(heading(t!("agent.connect_heading").to_string(), colors, label_font))
         .child(connect_block(colors, mono_font))
         .child(div().flex()
                     .items_center()
                     .justify_between()
                     .child(heading(t!("agent.activity_heading").to_string(), colors, label_font))
                     .child(Button::new("agent-refresh-activity")
                                .ghost()
                                .label(t!("agent.activity_refresh").to_string())
                                .on_click(move |_, _, cx| {
                                    refresh_entity.update(cx, |ctrl, cx| {
                                                      ctrl.refresh_mcp_activity(cx);
                                                  });
                                })))
         .child(activity_rows(&data, colors))
}
