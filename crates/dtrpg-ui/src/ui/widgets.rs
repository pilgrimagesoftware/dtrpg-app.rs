//! Small reusable rendering helpers shared across views.

use gpui::{App, ElementId, MouseDownEvent, SharedString, Styled as _, Window};
use gpui_kit::base::{GlobalState, TextSelection};
use gpui_kit::component::text::TextView;

use crate::data::theme::ColorTokens;
use crate::util::text::escape_markdown;

/// Renders `text` as selectable, copyable plain text via `gpui-kit`'s
/// `TextView`.
///
/// The text is Markdown-escaped first — `TextView` parses its input as
/// Markdown, and raw catalog/account data can contain characters (`*`, `_`,
/// `#`, ...) that would otherwise be misread as formatting. `id` must be
/// unique among sibling elements, same as any other `gpui` element id.
pub fn selectable_text(id: impl Into<ElementId>, text: impl Into<SharedString>) -> TextView {
    let escaped = escape_markdown(text.into().as_ref());
    TextView::markdown(id, escaped).selectable(true)
}

/// Keeps window text selection to presses that start on selectable text.
///
/// A window selection starts on any left press, including presses on drag
/// handles, rows and other controls. Controls that own a press suppress it
/// themselves; window roots call this for everything else. A press on a
/// selectable text field is not suppressed, so drag selection and multi-click
/// selection inside the field keep working.
pub fn suppress_selection_outside_text(event: &MouseDownEvent, window: &mut Window, cx: &mut App) {
    if !TextSelection::is_selectable_at(event.position, window, cx) {
        GlobalState::suppress_text_selection(cx);
    }
}

/// Uppercases a field/section label.
///
/// `gpui` has no `font-variant: small-caps`, and synthesizing it by rendering
/// two run sizes in one row put mismatched-height text on the same line
/// (baselines didn't line up). Plain uppercase gets the "this is a label, not
/// a value" visual distinction without that defect.
pub fn small_caps_text(text: impl Into<SharedString>) -> String {
    text.into().to_uppercase()
}

/// A small, secondary-colored note under a setting, which can be selected and
/// copied. `id` must be unique among its siblings.
pub fn selectable_note(id: impl Into<ElementId>, text: impl Into<SharedString>,
                       colors: &ColorTokens)
                       -> TextView {
    selectable_text(id, text).text_xs()
                             .text_color(colors.text_secondary)
}
