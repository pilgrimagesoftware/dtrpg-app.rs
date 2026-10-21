use super::*;

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
        InspectorTab::Collections => t!("detail.inspector_collections_tab").to_string(),
        InspectorTab::Contents => t!("detail.inspector_contents_tab").to_string(),
        InspectorTab::Summary => t!("detail.inspector_summary_tab").to_string(),
    }
}

/// Renders the row of inspector tabs, plus, when `pop_out` is given, a
/// control at the right end that moves the inspector into its own window.
pub(super) fn render_inspector_tab_bar(entry_id: Arc<str>, entity: Entity<LibraryController>,
                                       active: InspectorTab,
                                       pop_out: Option<Entity<TabsController>>,
                                       colors: &ColorTokens)
                                       -> AnyElement {
    let tabs = [InspectorTab::Overview,
                InspectorTab::Files,
                InspectorTab::Collections,
                InspectorTab::Contents,
                InspectorTab::Summary];
    div().id("inspector-tabs")
         .flex()
         .items_center()
         .gap(px(4.0))
         .border_b_1()
         .border_color(colors.border)
         .px(px(20.0))
         .children(tabs.into_iter().map(|tab| {
             let click_entity = entity.clone();
             let click_id = Arc::clone(&entry_id);
             let active_color = if tab == active { colors.accent } else { colors.border };
             div().id(SharedString::from(format!("inspector-tab-{tab:?}")))
                 .px(px(10.0))
                 .py(px(8.0))
                 .text_xs()
                 .text_color(if tab == active { colors.text_primary } else { colors.text_secondary })
                 .border_b_2()
                 .border_color(active_color)
                 .cursor_pointer()
                 .on_click(move |_, window, cx| {
                     click_entity.update(cx, |controller, cx| {
                         controller.set_inspector_tab(Arc::clone(&click_id), tab, cx);
                         if tab == InspectorTab::Contents {
                             controller.ensure_inspector_filter_input(Arc::clone(&click_id),
                                                                     window, cx);
                             controller.load_inspector_contents(Arc::clone(&click_id), cx);
                         }
                         if tab == InspectorTab::Summary {
                             controller.load_inspector_summary(Arc::clone(&click_id), cx);
                         }
                     });
                 })
                 .child(inspector_tab_label(tab))
         }))
         .children(pop_out.map(|tabs| render_pop_out_button(&entry_id, tabs, colors)))
         .into_any_element()
}

/// The control that moves an item's inspector out of the main window's tab
/// and into its own window.
fn render_pop_out_button(entry_id: &Arc<str>, tabs: Entity<TabsController>, colors: &ColorTokens)
                         -> AnyElement {
    let item_id = Arc::clone(entry_id);
    div().flex_1()
         .flex()
         .justify_end()
         .child(div().id("inspector-pop-out")
                     .p(px(6.0))
                     .rounded_sm()
                     .cursor_pointer()
                     .text_color(colors.text_secondary)
                     .tooltip(|window, cx| {
                         Tooltip::new(t!("detail.pop_out_tooltip").to_string()).build(window, cx)
                     })
                     .on_click(move |_, _, cx| {
                         tabs.update(cx, |tabs, cx| {
                                 tabs.request_pop_out(Arc::clone(&item_id), cx)
                             });
                     })
                     .child(Icon::new(IconName::ExternalLink).size(px(14.0))))
         .into_any_element()
}

pub(super) struct InspectorRenderContext<'a> {
    pub(super) storage_root_path: &'a Path,
    pub(super) entity:            Entity<LibraryController>,
    pub(super) tabs:              &'a Entity<TabsController>,
    pub(super) colors:            &'a ColorTokens,
    pub(super) window:            &'a mut Window,
    pub(super) cx:                &'a mut App,
}

pub(super) fn render_inspector_contents(state: &InspectorContentsState, entry_id: &Arc<str>,
                                        entity: Entity<LibraryController>, colors: &ColorTokens,
                                        cx: &App)
                                        -> AnyElement {
    match state {
        InspectorContentsState::Unavailable => {
            div().p(px(20.0))
                 .text_sm()
                 .text_color(colors.text_secondary)
                 .child(t!("detail.inspector_contents_unavailable").to_string())
                 .into_any_element()
        }
        InspectorContentsState::Loading => {
            div().p(px(20.0))
                 .text_sm()
                 .text_color(colors.text_secondary)
                 .child(t!("detail.inspector_contents_loading").to_string())
                 .into_any_element()
        }
        InspectorContentsState::Loaded(entries) => {
            let selected_count = entity.read(cx).selected_inspector_entry_count(entry_id);
            let extracting = entity.read(cx).is_inspector_extracting(entry_id);
            let rows = entries.iter().map(|entry| {
                                         let name = entry.name.clone();
                                         let selected =
                                             entity.read(cx)
                                                   .is_inspector_entry_selected(entry_id, &name);
                                         let is_directory = entry.is_directory;
                                         let size_bytes = entry.size_bytes;
                                         let click_entity = entity.clone();
                                         let click_id = Arc::clone(entry_id);
                                         let toggle_name = name.clone();
                                         let preview_name = name.clone();
                                         let preview_entity = entity.clone();
                                         let preview_id = Arc::clone(entry_id);
                                         let depth =
                                             name.trim_end_matches('/').matches('/').count();
                                         let label = name.trim_end_matches('/')
                                                         .rsplit('/')
                                                         .next()
                                                         .unwrap_or(&name)
                                                         .to_string();
                                         div()
                    .id(SharedString::from(format!("inspector-entry-{name}")))
                    .pl(px(depth as f32 * 14.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .on_click(move |_, _, cx| {
                        if !is_directory {
                            click_entity.update(cx, |controller, cx| {
                                controller.toggle_inspector_entry(Arc::clone(&click_id),
                                                                toggle_name.clone(), cx);
                            });
                        }
                    })
                    .child(if is_directory { "▸" } else if selected { "☑" } else { "□" })
                    .child(div().flex_1().text_sm().text_color(colors.text_primary).child(
                        if is_directory { format!("{label}/") } else { label },
                    ))
                    .child(div().text_xs().text_color(colors.text_tertiary).child(
                        if is_directory { String::new() } else { format!("{} bytes", size_bytes) },
                    ))
                    .child(if is_directory {
                        div().into_any_element()
                    } else {
                        Button::new(format!("inspector-preview-{name}"))
                            .ghost()
                            .child(t!("detail.inspector_preview").to_string())
                            .on_click(move |_, _, cx| {
                                cx.stop_propagation();
                                preview_entity.update(cx, |controller, cx| {
                                    controller.preview_inspector_entry(Arc::clone(&preview_id),
                                                                        preview_name.clone(), cx);
                                });
                            })
                            .into_any_element()
                    })
                                     });
            let preview = entity.read(cx).inspector_preview_for(entry_id);
            div()
                .p(px(20.0))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .children(render_inspector_preview(&preview, entry_id, entity.clone(), colors))
                .children(rows)
                .child(
                    Button::new("inspector-extract-selected")
                        .ghost()
                        .disabled(selected_count == 0 || extracting)
                        .loading(extracting)
                        .child(t!("detail.inspector_extract_selected").to_string())
                        .on_click({
                            let click_entity = entity.clone();
                            let click_id = Arc::clone(entry_id);
                            move |_, _, cx| {
                                click_entity.update(cx, |controller, cx| {
                                    controller.extract_inspector_entries(Arc::clone(&click_id), cx);
                                });
                            }
                        }),
                )
                .into_any_element()
        }
    }
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
            div().flex()
                 .flex_col()
                 .child(render_item_tier(item,
                                         context.storage_root_path,
                                         context.entity.clone(),
                                         context.tabs,
                                         context.colors,
                                         context.window,
                                         context.cx))
                 .child(render_verify_on_disk(&item.id, verifying, context.entity.clone()))
                 .into_any_element()
        }
        InspectorTab::Collections => render_collections_section(item,
                                                                context.entity.clone(),
                                                                context.colors,
                                                                context.cx).into_any_element(),
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
        InspectorTab::Overview => div().into_any_element(),
    };

    let pop_out = context.tabs
                         .read(context.cx)
                         .allows_pop_out()
                         .then(|| context.tabs.clone());

    div().id("inspector-secondary-tab")
         .flex_1()
         .min_h_0()
         .flex()
         .flex_col()
         .child(render_inspector_tab_bar(Arc::clone(&item.id),
                                         context.entity,
                                         tab,
                                         pop_out,
                                         context.colors))
         .child(div().flex_1()
                     .min_h_0()
                     .overflow_y_scrollbar()
                     .child(content))
         .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

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
