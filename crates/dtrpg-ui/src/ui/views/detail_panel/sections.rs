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

/// Stands in for the link while a translated sentence is split around it.
const LINK_MARKER: &str = "\u{e000}";

/// Splits a sentence translated with [`LINK_MARKER`] where its link goes, into
/// the text before the link and the text after it. A sentence with no marker
/// has no link: all of it is before.
fn split_around_link(sentence: &str) -> (&str, &str) {
    sentence.split_once(LINK_MARKER).unwrap_or((sentence, ""))
}

/// The Opens With section: which application opens the item's file type, and
/// where to change it.
///
/// An override set in Settings > File Openers names its application; with
/// none, the system's default application opens the file. The hint under it
/// links to that page of the settings.
pub(super) fn render_opens_with(item: &LibraryItem, file_openers: &[FileOpenerEntry],
                                settings: &Entity<SettingsController>, colors: &ColorTokens)
                                -> AnyElement {
    let extension = item.format.to_lowercase();
    let application =
        file_openers.iter()
                    .find(|opener| opener.extension == extension)
                    .map(|opener| opener.app_path.display().to_string())
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
         .child(render_file_openers_hint(settings, colors))
         .into_any_element()
}

/// The hint under Opens With, with "Settings > File Openers" as a link that
/// opens the settings on that page.
fn render_file_openers_hint(settings: &Entity<SettingsController>, colors: &ColorTokens)
                            -> AnyElement {
    let sentence = t!("detail.opens_with_note", link = LINK_MARKER).to_string();
    let (before, after) = split_around_link(&sentence);
    let settings = settings.clone();
    let link = div().id("opens-with-settings-link")
                    .cursor_pointer()
                    .text_color(colors.accent)
                    .underline()
                    .on_click(move |_, window, cx| {
                        settings.update(cx, |ctrl, cx| {
                                    ctrl.set_active_page_ix(FILE_OPENERS_PAGE_IX, cx)
                                });
                        window.dispatch_action(Box::new(ShowSettings), cx);
                    })
                    .child(t!("detail.opens_with_settings_link").to_string());
    div().flex()
         .flex_wrap()
         .items_baseline()
         .gap_x(px(3.0))
         .text_xs()
         .text_color(colors.text_tertiary)
         .child(before.trim_end().to_string())
         .child(div().flex().child(link).child(after.to_string()))
         .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::{LINK_MARKER, split_around_link};

    #[test]
    fn a_sentence_is_split_where_its_link_goes() {
        let sentence = format!("Overrides live in {LINK_MARKER}.");

        assert_eq!(split_around_link(&sentence), ("Overrides live in ", "."));
    }

    #[test]
    fn a_sentence_without_a_marker_has_no_text_after_its_link() {
        assert_eq!(split_around_link("No link here"), ("No link here", ""));
    }
}
