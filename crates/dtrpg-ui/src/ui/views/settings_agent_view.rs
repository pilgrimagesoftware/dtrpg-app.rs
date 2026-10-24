//! The Agent & MCP page: whether agents may use Libri, what they may do, which
//! tools they have, how to connect one, and what they have done.

use std::path::PathBuf;

use dtrpg_core::data::mcp_access::{McpScope, McpTool};
use gpui::prelude::*;
use gpui::{AnyElement, ClipboardItem, Entity, Hsla, SharedString, div, px};
use gpui_kit::component::Icon;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use gpui_kit::component::tag::Tag;
use gpui_kit::component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings_native_view::switch_row;
use crate::ui::views::settings_summaries_view::render_summaries;
use crate::ui::widgets::{selectable_note, selectable_text, small_caps_text};
use crate::util::item_opener::ItemOpener;
use crate::view_models::agent_display::{
    AgentPageData, agent_config_json, exposed_tools, scope_label, summary_label, time_label,
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

/// The colors of a scope's badge, as text and background: green for read,
/// amber for change, red for download and spend, so a wider scope reads as the
/// riskier choice. They are fixed, not taken from the theme (as the other state
/// colors are), and lighter on a dark page so they stay readable there.
fn risk_colors(scope: McpScope, colors: &ColorTokens) -> (Hsla, Hsla) {
    let dark = colors.surface.l < 0.5;
    let (light_text, dark_text) = match scope {
        McpScope::Read => (0x3F_7A_44, 0x7F_C2_83),
        McpScope::Change => (0x93_66_12, 0xE6_B6_55),
        McpScope::DownloadAndSpend => (0xB0_3A_30, 0xFF_7A_70),
    };
    let text: Hsla = gpui::rgb(if dark { dark_text } else { light_text }).into();
    (text, Hsla { a: 0.16, ..text })
}

/// The risk badge beside a scope's title, and beside each call in the log.
fn risk_badge(scope: McpScope, label: String, colors: &ColorTokens) -> impl IntoElement {
    let (text, background) = risk_colors(scope, colors);
    div().px(px(6.0))
         .py(px(1.0))
         .rounded(px(4.0))
         .border_1()
         .border_color(Hsla { a: 0.45, ..text })
         .bg(background)
         .text_xs()
         .font_weight(gpui::FontWeight::SEMIBOLD)
         .text_color(text)
         .child(label)
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
                                          .child(risk_badge(scope, t!(text.risk).to_string(), colors)))
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

/// The widths of the tools table's fixed columns, in pixels. The description
/// takes whatever is left.
const TOOL_COLUMN_WIDTH: f32 = 190.0;
const SCOPE_COLUMN_WIDTH: f32 = 92.0;
const ON_COLUMN_WIDTH: f32 = 64.0;

/// The tools: each with its scope, what it does, and a switch.
fn tools_table(entity: &Entity<SettingsController>, data: &AgentPageData, colors: &ColorTokens)
               -> impl IntoElement {
    let head = |label: &str| TableHead::new().child(small_caps_text(t!(label).to_string()));
    let header = TableHeader::new().child(TableRow::new().child(head("agent.col_tool")
                                                                    .w(px(TOOL_COLUMN_WIDTH))
                                                                    .flex_none())
                                                         .child(head("agent.col_scope")
                                                                    .w(px(SCOPE_COLUMN_WIDTH))
                                                                    .flex_none())
                                                         .child(head("agent.col_what"))
                                                         .child(head("agent.col_on")
                                                                    .w(px(ON_COLUMN_WIDTH))
                                                                    .flex_none()));
    let rows = McpTool::ALL.into_iter().map(|tool| {
        let entity = entity.clone();
        let on = !data.settings.disabled_tools.contains(tool.name());
        let tooltip = t!("agent.tool_toggle_tooltip", tool = tool.name()).to_string();
        let (scope_text, _) = risk_colors(tool.scope(), colors);
        TableRow::new().child(TableCell::new().w(px(TOOL_COLUMN_WIDTH))
                                              .flex_none()
                                              .child(div().font_family("monospace")
                                                          .text_sm()
                                                          .text_color(colors.text_primary)
                                                          .child(tool.name())))
                       .child(TableCell::new().w(px(SCOPE_COLUMN_WIDTH))
                                              .flex_none()
                                              .child(div().text_xs()
                                                          .font_weight(gpui::FontWeight::SEMIBOLD)
                                                          .text_color(scope_text)
                                                          .child(scope_label(tool.scope()))))
                       .child(TableCell::new().child(div().min_w_0()
                                                          .text_sm()
                                                          .text_color(colors.text_secondary)
                                                          .child(tool_description(tool))))
                       .child(TableCell::new().w(px(ON_COLUMN_WIDTH))
                                              .flex_none()
                                              .child(
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

/// The config an agent needs, in a box, with an icon button inside it that
/// copies it and a button under the box that rotates the token agents
/// carry.
fn connect_block(command_path: &str, colors: &ColorTokens, mono_font: &str,
                 library: Entity<LibraryController>)
                 -> impl IntoElement {
    let config = agent_config_json(command_path);
    let copied = config.clone();
    div().flex()
         .flex_col()
         .gap(px(8.0))
         .child(selectable_note("agent-connect-note", t!("agent.connect_note").to_string(), colors))
         .child(div().relative()
                     .p(px(12.0))
                     .pr(px(48.0))
                     .rounded(px(6.0))
                     .border_1()
                     .border_color(colors.border)
                     .bg(colors.surface_alt)
                     .font_family(mono_font.to_string())
                     .child(selectable_text("agent-config", config).text_xs()
                                                                   .text_color(colors.text_primary))
                     .child(div().absolute()
                                 .top(px(6.0))
                                 .right(px(6.0))
                                 .child(Button::new("agent-copy-config")
                                            .ghost()
                                            .icon(Icon::empty().path("icons/copy.svg"))
                                            .tooltip(t!("agent.copy_config").to_string())
                                            .on_click(move |_, _, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(copied.clone()));
                                            }))))
         .child(Button::new("agent-rotate-token")
                    .ghost()
                    .label(t!("agent.rotate_token").to_string())
                    .tooltip(t!("agent.rotate_note").to_string())
                    .on_click(move |_, _, cx| {
                        library.update(cx, |ctrl, _| ctrl.rotate_agent_token());
                    }))
}

/// Where the log is kept, so a person can open it.
fn log_location(log_path: &str, colors: &ColorTokens) -> impl IntoElement {
    div().flex()
         .flex_col()
         .gap(px(2.0))
         .child(div().text_xs()
                     .text_color(colors.text_secondary)
                     .child(t!("agent.log_location").to_string()))
         .child(selectable_text("agent-log-path", log_path.to_owned()).text_xs()
                                                                      .text_color(colors.text_primary))
}

/// An info row: a title and a note on the left, a fixed value on the right.
/// Hand-rolled because
/// [`crate::ui::views::settings_card::field_row`]'s 140-pixel field would
/// cut a socket path in half.
fn info_row(id: &str, title: String, note: &'static str, value: AnyElement, colors: &ColorTokens)
            -> AnyElement {
    div().flex()
         .flex_col()
         .gap(px(4.0))
         .child(div().flex()
                     .items_start()
                     .justify_between()
                     .gap(px(12.0))
                     .child(div().text_sm()
                                 .font_weight(gpui::FontWeight::SEMIBOLD)
                                 .text_color(colors.text_primary)
                                 .child(title))
                     .child(value))
         .child(selectable_note(id.to_owned(), t!(note).to_string(), colors))
         .into_any_element()
}

/// The Transport and Bind address rows. Both are facts, not controls: the
/// server runs on stdio plus the local socket, and a socket is reachable
/// only from this machine, so there is nothing to choose and no way to
/// expose it to a network. The mockup's dropdown would be decoration.
fn transport_rows(data: &AgentPageData, colors: &ColorTokens, mono_font: &str) -> AnyElement {
    let transport: AnyElement = div().text_xs()
                                     .text_color(colors.text_primary)
                                     .child(t!("agent.transport_value").to_string())
                                     .into_any_element();
    let bind: AnyElement = selectable_text("agent-bind-value", data.bind_display.clone()).text_xs()
                                                                                          .font_family(mono_font.to_string())
                                                                                          .max_w(px(360.0))
                                                                                          .text_color(colors.text_primary)
                                                                                          .into_any_element();
    div().flex()
         .flex_col()
         .gap(px(16.0))
         .child(info_row("agent-info-transport",
                         t!("agent.transport_title").to_string(),
                         "agent.transport_note",
                         transport,
                         colors))
         .child(info_row("agent-info-bind",
                         t!("agent.bind_title").to_string(),
                         "agent.bind_note",
                         bind,
                         colors))
         .into_any_element()
}

/// Opens the activity log with the system's default app. A log that was
/// never written has nothing to open, so say so in the log rather than
/// opening an empty editor.
fn open_agent_log(log_path: &str) {
    let path = PathBuf::from(log_path);
    if !path.exists() {
        tracing::warn!(path = %path.display(), "the agent log does not exist yet");
        return;
    }
    if let Err(error) = ItemOpener::open(&path) {
        tracing::warn!(%error, "could not open the agent log");
    }
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
                                                                  scope_label(entry.scope),
                                                                  colors))
                                       }))
         .into_any_element()
}

/// Renders the Agent & MCP page.
pub fn render_agent_section(entity: Entity<SettingsController>, data: AgentPageData,
                            colors: &ColorTokens, label_font: &str, mono_font: &str,
                            library: Entity<LibraryController>)
                            -> impl IntoElement + 'static + use<> {
    let enable_entity = entity.clone();
    let refresh_entity = entity.clone();
    let log_path = data.log_path.clone();
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
         .child(render_summaries(&entity, &data.summaries, &data.summary_page, colors))
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
         .child(transport_rows(&data, colors, mono_font))
         .child(div().h(px(1.0)).bg(colors.border))
         .child(heading(t!("agent.scopes_heading").to_string(), colors, label_font))
         .child(scope_rows(&entity, &data, colors))
         .child(heading(t!("agent.tools_heading").to_string(), colors, label_font))
         .child(tools_table(&entity, &data, colors))
         .child(heading(t!("agent.connect_heading").to_string(), colors, label_font))
         .child(connect_block(&data.command_path, colors, mono_font, library))
         .child(div().flex()
                     .items_center()
                     .justify_between()
                     .child(heading(t!("agent.activity_heading").to_string(), colors, label_font))
                     .child(div().flex()
                                 .items_center()
                                 .gap(px(4.0))
                                 .child(Button::new("agent-open-log")
                                            .ghost()
                                            .label(t!("agent.open_log").to_string())
                                            .on_click(move |_, _, _| open_agent_log(&log_path)))
                                 .child(Button::new("agent-refresh-activity")
                                            .ghost()
                                            .icon(Icon::empty().path("icons/refresh-cw.svg"))
                                            .tooltip(t!("agent.activity_refresh").to_string())
                                            .on_click(move |_, _, cx| {
                                                refresh_entity.update(cx, |ctrl, cx| {
                                                                  ctrl.refresh_mcp_activity(cx);
                                                              });
                                            }))))
         .child(activity_rows(&data, colors))
         .child(log_location(&data.log_path, colors))
}
