//! Toolbar view: section title, search, sort dropdown, group toggle, layout switcher.

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, Entity, Image, ImageFormat, ImageSource, IntoElement, MouseButton, ObjectFit,
    ParentElement, Styled, div, img, px,
};
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_component::input::{Input, InputState};
use gpui_component::menu::{DropdownMenu, PopupMenuItem};
use gpui_component::tab::{Tab, TabBar};

use crate::controllers::library::LibraryController;
use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::{enums::CatalogPresentation, theme::ColorTokens};
use crate::util::filter::*;
use crate::util::pluralize::pluralize;
use crate::util::sort::*;
use rust_i18n::t;

fn section_title_for(filter: &SidebarFilter) -> String {
    match filter {
        SidebarFilter::AllTitles => t!("sidebar.all_titles").to_string(),
        SidebarFilter::RecentlyAdded => t!("sidebar.recently_added").to_string(),
        SidebarFilter::OnDevice => t!("sidebar.on_this_device").to_string(),
        SidebarFilter::InCloud => t!("sidebar.in_the_cloud").to_string(),
        SidebarFilter::Publisher(name) => format!("Publisher: {name}"),
        SidebarFilter::Collection(_, name) => format!("Collection: {name}"),
    }
}

/// Builds the count text shown beneath the section title.
///
/// `filter_count` is the number of catalog items matching `filter` alone (ignoring
/// search); `matched_count` additionally applies the search query; `total_count`
/// is the whole-catalog count. Publisher filters keep the legacy "publisher item(s),
/// total item(s)" wording; `AllTitles` shows the plain catalog total; every other
/// filter (smart sections, collections) shows `filter_count` since the filter
/// itself already narrows the result set below the catalog total.
fn count_label_for(
    filter: &SidebarFilter,
    filter_count: usize,
    matched_count: usize,
    total_count: usize,
    has_search: bool,
) -> String {
    if matches!(filter, SidebarFilter::Publisher(_)) {
        return if has_search {
            format!(
                "{}, {} ({} filtered)",
                pluralize(filter_count, "publisher item", "publisher items"),
                pluralize(total_count, "total item", "total items"),
                matched_count,
            )
        } else {
            format!(
                "{}, {}",
                pluralize(filter_count, "publisher item", "publisher items"),
                pluralize(total_count, "total item", "total items"),
            )
        };
    }

    let base_count = if matches!(filter, SidebarFilter::AllTitles) {
        total_count
    } else {
        filter_count
    };
    if has_search {
        format!(
            "{} ({} filtered)",
            pluralize(base_count, "item", "items"),
            matched_count,
        )
    } else {
        pluralize(base_count, "item", "items")
    }
}

/// Renders the toolbar row above the catalog.
#[allow(clippy::too_many_arguments)]
pub fn render_toolbar(
    filter: &SidebarFilter,
    filter_count: usize,
    matched_count: usize,
    total_count: usize,
    search_query: &str,
    search_input: Entity<InputState>,
    sort: SortMethod,
    sort_direction: SortDirection,
    grouped: bool,
    presentation: CatalogPresentation,
    entity: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    auth: &AuthStateSnapshot,
    colors: &ColorTokens,
    cx: &App,
) -> impl IntoElement + 'static + use<> {
    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_tertiary = colors.text_tertiary;

    let title = section_title_for(filter);
    let has_search = !search_query.is_empty();
    let count_label = count_label_for(filter, filter_count, matched_count, total_count, has_search);

    div()
        .h(px(57.0))
        .flex_none()
        .flex()
        .items_center()
        .gap(px(16.0))
        .px(px(18.0))
        .border_b_1()
        .border_color(border)
        .bg(surface)
        // ── Title + count ─────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(1.0))
                .min_w_0()
                .child(
                    div()
                        .text_color(text_primary)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_lg()
                        .truncate()
                        .child(title),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .whitespace_nowrap()
                        .child(count_label),
                ),
        )
        // ── Spacer / drag region ──────────────────────────────────────────
        .child(
            div()
                .id("toolbar-drag-region")
                .flex_1()
                .h_full()
                .on_mouse_down(MouseButton::Left, |_, window, _| {
                    window.start_window_move();
                }),
        )
        // ── Controls ──────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(Input::new(&search_input).w(px(188.0)).cleanable(true))
                .child(render_sort_selector(
                    sort,
                    sort_direction,
                    grouped,
                    entity.clone(),
                ))
                .child(render_layout_switcher(presentation, entity))
                .child(render_settings_button(settings.clone()))
                .child(render_avatar_button(auth, settings, colors, cx)),
        )
}

// ── Sort selector ─────────────────────────────────────────────────────────────

fn render_sort_selector(
    current: SortMethod,
    direction: SortDirection,
    grouped: bool,
    entity: Entity<LibraryController>,
) -> impl IntoElement + 'static {
    let label = match current {
        SortMethod::Title => t!("toolbar.sort_title"),
        SortMethod::Publisher => t!("toolbar.sort_publisher"),
        SortMethod::DateAdded => t!("toolbar.sort_date_added"),
        SortMethod::PageCount => t!("toolbar.sort_pages"),
        SortMethod::Custom { .. } => "Custom".into(),
    };

    let is_custom = matches!(current, SortMethod::Custom { .. });

    Button::new("sort-selector")
        .label(label)
        .ghost()
        .dropdown_caret(true)
        .tooltip(t!("toolbar.sort_by"))
        .dropdown_menu(move |menu, _, _| {
            let e = entity.clone();
            let e2 = entity.clone();
            let e3 = entity.clone();
            let e4 = entity.clone();
            let e5 = entity.clone();
            let e6 = entity.clone();
            let e7 = entity.clone();
            let mut m = menu
                .item(
                    PopupMenuItem::new(t!("toolbar.sort_title"))
                        .checked(current == SortMethod::Title)
                        .on_click(move |_, _, cx| {
                            e.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::Title, cx));
                        }),
                )
                .item(
                    PopupMenuItem::new(t!("toolbar.sort_publisher"))
                        .checked(current == SortMethod::Publisher)
                        .on_click(move |_, _, cx| {
                            e2.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::Publisher, cx));
                        }),
                )
                .item(
                    PopupMenuItem::new(t!("toolbar.sort_date_added"))
                        .checked(current == SortMethod::DateAdded)
                        .on_click(move |_, _, cx| {
                            e3.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::DateAdded, cx));
                        }),
                )
                .item(
                    PopupMenuItem::new(t!("toolbar.sort_pages"))
                        .checked(current == SortMethod::PageCount)
                        .on_click(move |_, _, cx| {
                            e4.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::PageCount, cx));
                        }),
                );
            if is_custom {
                m = m.item(PopupMenuItem::new("Custom").checked(true).disabled(true));
            }
            m.separator()
                .item(
                    PopupMenuItem::new(t!("toolbar.sort_ascending"))
                        .checked(direction == SortDirection::Ascending)
                        .on_click(move |_, _, cx| {
                            e5.update(cx, |ctrl, cx| {
                                ctrl.set_sort_direction(SortDirection::Ascending, cx);
                            });
                        }),
                )
                .item(
                    PopupMenuItem::new(t!("toolbar.sort_descending"))
                        .checked(direction == SortDirection::Descending)
                        .on_click(move |_, _, cx| {
                            e6.update(cx, |ctrl, cx| {
                                ctrl.set_sort_direction(SortDirection::Descending, cx);
                            });
                        }),
                )
                .separator()
                .item(
                    PopupMenuItem::new(t!("toolbar.group_by_publisher"))
                        .checked(grouped)
                        .on_click(move |_, _, cx| {
                            e7.update(cx, |ctrl, cx| ctrl.set_grouped(!grouped, cx));
                        }),
                )
        })
}

// ── Layout switcher ───────────────────────────────────────────────────────────

fn render_layout_switcher(
    current: CatalogPresentation,
    entity: Entity<LibraryController>,
) -> impl IntoElement + 'static {
    let selected = match current {
        CatalogPresentation::List => 0usize,
        CatalogPresentation::Thumbs => 1,
        CatalogPresentation::Grid => 2,
    };
    TabBar::new("layout-switcher")
        .segmented()
        .selected_index(selected)
        .child(Tab::new().label(t!("toolbar.view_list")))
        .child(Tab::new().label(t!("toolbar.view_thumbs")))
        .child(Tab::new().label(t!("toolbar.view_grid")))
        .on_click(move |ix, _, cx| {
            let mode = match ix {
                0 => CatalogPresentation::List,
                1 => CatalogPresentation::Thumbs,
                _ => CatalogPresentation::Grid,
            };
            entity.update(cx, |ctrl, cx| ctrl.set_presentation(mode, cx));
        })
}

// ── Settings gear button ──────────────────────────────────────────────────────

fn render_settings_button(settings: Entity<SettingsController>) -> impl IntoElement + 'static {
    Button::new("settings-gear")
        .ghost()
        .icon(IconName::Settings)
        .tooltip("Settings")
        .on_click(move |_, _, cx| {
            settings.update(cx, |ctrl, cx| ctrl.toggle(cx));
        })
}

// ── Avatar button ─────────────────────────────────────────────────────────────

fn detect_image_format(bytes: &[u8]) -> ImageFormat {
    if bytes.starts_with(b"\x89PNG") {
        ImageFormat::Png
    } else {
        ImageFormat::Jpeg
    }
}

fn render_avatar_button(
    auth: &AuthStateSnapshot,
    settings: Entity<SettingsController>,
    colors: &ColorTokens,
    cx: &App,
) -> AnyElement {
    if !auth.is_logged_in {
        let surface_alt = colors.surface_alt;
        let border_strong = colors.border_strong;
        let text_tertiary = colors.text_tertiary;
        let unauthenticated_variant = ButtonCustomVariant::new(cx)
            .color(surface_alt)
            .foreground(text_tertiary)
            .hover(gpui::Hsla {
                l: (surface_alt.l * 0.9).min(1.0),
                ..surface_alt
            })
            .active(gpui::Hsla {
                l: (surface_alt.l * 0.8).min(1.0),
                ..surface_alt
            });
        let inner = div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .text_xs()
            .text_color(text_tertiary)
            .child("👤")
            .into_any_element();
        return Button::new("avatar-btn")
            .custom(unauthenticated_variant)
            .tooltip("Not signed in")
            .rounded_full()
            .w(px(30.0))
            .h(px(30.0))
            .border_1()
            .border_color(border_strong)
            .child(inner)
            .dropdown_menu(move |menu, _, _| {
                let s = settings.clone();
                menu.item(
                    PopupMenuItem::new(t!("toolbar.sign_in")).on_click(move |_, _, cx| {
                        s.update(cx, |ctrl, cx| ctrl.open(cx));
                    }),
                )
            })
            .into_any_element();
    }

    let initial_text = auth
        .display_initial
        .map(|c| c.to_string())
        .unwrap_or_else(|| "D".to_string());

    let accent = colors.accent;
    let avatar_variant = ButtonCustomVariant::new(cx)
        .color(accent)
        .foreground(gpui::white())
        .hover(gpui::Hsla {
            l: (accent.l * 0.85).min(1.0),
            ..accent
        })
        .active(gpui::Hsla {
            l: (accent.l * 0.75).min(1.0),
            ..accent
        });

    let inner: AnyElement = if let Some(bytes) = &auth.avatar_bytes {
        let format = detect_image_format(bytes);
        let image = Arc::new(Image::from_bytes(format, bytes.as_ref().clone()));
        img(ImageSource::Image(image))
            .w(px(30.0))
            .h(px(30.0))
            .rounded_full()
            .object_fit(ObjectFit::Cover)
            .into_any_element()
    } else {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .text_xs()
            .text_color(gpui::white())
            .child(initial_text)
            .into_any_element()
    };

    let menu_email = auth
        .email
        .clone()
        .or_else(|| auth.api_key_hint.clone())
        .unwrap_or_else(|| "DriveThruRPG Account".to_string());

    Button::new("avatar-btn")
        .custom(avatar_variant)
        .tooltip("Account")
        .rounded_full()
        .w(px(30.0))
        .h(px(30.0))
        .child(inner)
        .dropdown_menu(move |menu, _, _| {
            let s = settings.clone();
            menu.item(PopupMenuItem::label(menu_email.clone()))
                .item(PopupMenuItem::separator())
                .item(
                    PopupMenuItem::new(t!("toolbar.log_out")).on_click(move |_, _, cx| {
                        s.update(cx, |ctrl, cx| ctrl.logout(cx));
                    }),
                )
        })
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::filter::SidebarFilter;

    #[test]
    fn publisher_filter_includes_name() {
        let title = section_title_for(&SidebarFilter::Publisher("Kobold Press".into()));
        assert!(
            title.contains("Kobold Press"),
            "publisher name must appear in label"
        );
    }

    #[test]
    fn collection_filter_includes_name() {
        let title = section_title_for(&SidebarFilter::Collection(42, "My Shelf".into()));
        assert!(
            title.contains("My Shelf"),
            "collection name must appear in label"
        );
    }

    #[test]
    fn collection_filter_count_reflects_filter_count_not_total() {
        // Regression: selecting a collection must show the collection's own
        // item count, not the whole-catalog total (which stays fixed across
        // filter changes and made the count text look unresponsive).
        let filter = SidebarFilter::Collection(42, "My Shelf".into());
        let label = count_label_for(&filter, 3, 3, 500, false);
        assert!(
            label.contains('3'),
            "expected collection filter count (3) in label, got: {label}"
        );
        assert!(
            !label.contains("500"),
            "collection label must not show the whole-catalog total, got: {label}"
        );
    }

    #[test]
    fn smart_section_filter_count_reflects_filter_count_not_total() {
        let filter = SidebarFilter::OnDevice;
        let label = count_label_for(&filter, 7, 7, 500, false);
        assert!(label.contains('7'));
        assert!(!label.contains("500"));
    }

    #[test]
    fn all_titles_filter_count_shows_total() {
        let label = count_label_for(&SidebarFilter::AllTitles, 500, 500, 500, false);
        assert!(label.contains("500"));
    }

    #[test]
    fn collection_filter_with_search_shows_matched_count() {
        let filter = SidebarFilter::Collection(42, "My Shelf".into());
        let label = count_label_for(&filter, 10, 2, 500, true);
        assert!(label.contains("2 filtered"));
    }

    #[test]
    fn non_dynamic_filters_return_nonempty_string() {
        for filter in [
            SidebarFilter::AllTitles,
            SidebarFilter::RecentlyAdded,
            SidebarFilter::OnDevice,
            SidebarFilter::InCloud,
        ] {
            assert!(!section_title_for(&filter).is_empty());
        }
    }
}
