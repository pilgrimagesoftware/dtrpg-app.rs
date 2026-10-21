use super::*;

/// One section of the Info tab: a small-caps heading with a rule beside it,
/// and the section's body below.
pub(super) fn render_section(heading: String, body: impl IntoElement, colors: &ColorTokens,
                             label_font_family: &str)
                             -> AnyElement {
    div().flex()
         .flex_col()
         .gap(px(8.0))
         .child(div().flex()
                     .items_center()
                     .gap(px(8.0))
                     .child(div().font_family(label_font_family.to_string())
                                 .text_xs()
                                 .text_color(colors.text_tertiary)
                                 .child(small_caps_text(heading)))
                     .child(div().flex_1().h(px(1.0)).bg(colors.border)))
         .child(body)
         .into_any_element()
}

/// The Opens With section: which application opens the item's file type, and
/// where to change it.
///
/// An override set in Settings > File Openers names its application; with
/// none, the system's default application opens the file.
pub(super) fn render_opens_with(item: &LibraryItem, file_openers: &[FileOpenerEntry],
                                colors: &ColorTokens)
                                -> AnyElement {
    let extension = item.format.to_lowercase();
    let application =
        file_openers.iter()
                    .find(|opener| opener.extension == extension)
                    .and_then(|opener| opener.app_path.file_stem())
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| t!("detail.opens_with_system_default").to_string());
    div().flex()
         .flex_col()
         .gap(px(6.0))
         .child(div().flex()
                     .items_center()
                     .justify_between()
                     .gap(px(8.0))
                     .text_sm()
                     .child(div().text_color(colors.text_primary)
                                 .child(t!("detail.opens_with_files",
                                           extension = item.format.to_string()).to_string()))
                     .child(div().text_color(colors.text_secondary).child(application)))
         .child(div().text_xs()
                     .text_color(colors.text_tertiary)
                     .child(t!("detail.opens_with_note").to_string()))
         .into_any_element()
}
