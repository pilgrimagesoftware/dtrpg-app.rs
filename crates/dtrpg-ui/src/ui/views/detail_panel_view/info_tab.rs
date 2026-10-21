use super::*;

/// Renders the Info tab: the description, tags, collections, a details table,
/// what opens the file, and the collapsed "other details".
pub(super) fn render_info_tab(item: &LibraryItem, storage_root_path: &Path,
                              file_openers: &[FileOpenerEntry],
                              controller: Entity<LibraryController>, colors: &ColorTokens,
                              cx: &App)
                              -> AnyElement {
    let label_font_family = cx.global::<LibriTheme>().fonts.label_font.clone();
    let value_font_family = cx.global::<LibriTheme>().fonts.value_font.clone();
    let section = |heading: String, body: AnyElement| {
        render_section(heading, body, colors, &label_font_family)
    };

    let description = selectable_text("inspector-description", item.desc.to_string())
        .text_sm()
        .text_color(colors.text_secondary)
        .into_any_element();
    let tags = render_tags_section(item, controller.clone(), colors, cx).into_any_element();
    let collections =
        render_collections_section(item, controller.clone(), colors, cx).into_any_element();
    let details = render_metadata_table(item,
                                        storage_root_path,
                                        &label_font_family,
                                        &value_font_family).into_any_element();

    div().flex()
         .flex_col()
         .gap(px(20.0))
         .p(px(16.0))
         .when(!item.desc.trim().is_empty(), |this| {
             this.child(section(t!("detail.description_heading").to_string(), description))
         })
         .child(section(t!("detail.tags_heading").to_string(), tags))
         .child(section(t!("detail.collections_heading").to_string(), collections))
         .child(section(t!("detail.details_heading").to_string(), details))
         .child(section(t!("detail.opens_with_heading").to_string(),
                        render_opens_with(item, file_openers, colors)))
         .child(render_other_details(item, controller, colors, cx))
         .into_any_element()
}
