//! Toolbar view: section title, search, sort dropdown, group toggle, layout switcher.

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{AnyElement, App, div, img, px, Entity, Image, ImageFormat, ImageSource, IntoElement, ObjectFit, ParentElement, Styled};
use gpui_component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_component::menu::{DropdownMenu, PopupMenuItem};
use gpui_component::tooltip::Tooltip;

use crate::controllers::library::LibraryController;
use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::{
    enums::{CatalogPresentation},
    theme::ColorTokens,
};
use crate::util::filter::*;
use crate::util::sort::*;

fn section_title_for(filter: &SidebarFilter) -> &str {
    match filter {
        SidebarFilter::AllTitles => "All Titles",
        SidebarFilter::RecentlyAdded => "Recently Added",
        SidebarFilter::OnDevice => "On This Device",
        SidebarFilter::InCloud => "In the Cloud",
        SidebarFilter::Publisher(_) => "Publisher",
    }
}

/// Renders the toolbar row above the catalog.
#[allow(clippy::too_many_arguments)]
pub fn render_toolbar(
    filter: &SidebarFilter,
    matched_count: usize,
    search_query: &str,
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
    let border_strong = colors.border_strong;
    let text_primary = colors.text_primary;
    let text_tertiary = colors.text_tertiary;
    let bg = colors.surface_alt;
    let accent = colors.accent;
    let accent_soft = colors.accent_soft;

    let title = section_title_for(filter).to_string();
    let search_query = search_query.to_string();

    div()
        .h(px(53.0))
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
                .items_baseline()
                .gap(px(11.0))
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
                        .child(matched_count.to_string()),
                ),
        )
        // ── Spacer ────────────────────────────────────────────────────────
        .child(div().flex_1())
        // ── Controls ──────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(render_search(
                    search_query,
                    entity.clone(),
                    bg,
                    border_strong,
                    text_primary,
                    text_tertiary,
                ))
                .child(render_sort_selector(
                    sort,
                    sort_direction,
                    entity.clone(),
                    bg,
                    border_strong,
                    text_primary,
                    text_tertiary,
                ))
                .child(render_group_toggle(
                    grouped,
                    entity.clone(),
                    bg,
                    border_strong,
                    text_primary,
                    accent,
                    accent_soft,
                ))
                .child(render_layout_switcher(
                    presentation,
                    entity,
                    bg,
                    border_strong,
                    text_primary,
                    accent,
                    accent_soft,
                ))
                .child(render_settings_button(settings.clone(), text_primary, border_strong))
                .child(render_avatar_button(auth, settings, colors, cx)),
        )
}

// ── Search ────────────────────────────────────────────────────────────────────

fn render_search(
    query: String,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    text_tertiary: gpui::Hsla,
) -> impl IntoElement + 'static {
    let has_query = !query.is_empty();
    let entity_clear = entity.clone();

    div()
        .flex()
        .items_center()
        .gap(px(7.0))
        .h(px(30.0))
        .px(px(9.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(bg)
        .w(px(188.0))
        .child(div().text_xs().text_color(text_tertiary).child("⌕"))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .text_color(if has_query { text_primary } else { text_tertiary })
                .truncate()
                .child(if has_query { query } else { "Search…".into() }),
        )
        .when(has_query, |el| {
            el.child(
                div()
                    .id("search-clear")
                    .text_xs()
                    .text_color(text_tertiary)
                    .cursor_pointer()
                    .on_click(move |_, _, cx| {
                        entity_clear.update(cx, |ctrl, cx| {
                            ctrl.clear_search_query(cx);
                        });
                    })
                    .child("✕"),
            )
        })
}

// ── Sort selector ─────────────────────────────────────────────────────────────

fn render_sort_selector(
    current: SortMethod,
    direction: SortDirection,
    entity: Entity<LibraryController>,
    _bg: gpui::Hsla,
    _border: gpui::Hsla,
    _text_primary: gpui::Hsla,
    _text_tertiary: gpui::Hsla,
) -> impl IntoElement + 'static {
    let label = match current {
        SortMethod::Title => "Title",
        SortMethod::Publisher => "Publisher",
        SortMethod::DateAdded => "Date Added",
        SortMethod::PageCount => "Pages",
        SortMethod::Custom { .. } => "Custom",
    };

    let is_custom = matches!(current, SortMethod::Custom { .. });

    Button::new("sort-selector")
        .label(label)
        .ghost()
        .tooltip("Sort order")
        .dropdown_menu(move |menu, _, _| {
            let e = entity.clone();
            let e2 = entity.clone();
            let e3 = entity.clone();
            let e4 = entity.clone();
            let e5 = entity.clone();
            let e6 = entity.clone();
            let mut m = menu
                .item(
                    PopupMenuItem::new("Title")
                        .checked(current == SortMethod::Title)
                        .on_click(move |_, _, cx| {
                            e.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::Title, cx));
                        }),
                )
                .item(
                    PopupMenuItem::new("Publisher")
                        .checked(current == SortMethod::Publisher)
                        .on_click(move |_, _, cx| {
                            e2.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::Publisher, cx));
                        }),
                )
                .item(
                    PopupMenuItem::new("Date Added")
                        .checked(current == SortMethod::DateAdded)
                        .on_click(move |_, _, cx| {
                            e3.update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::DateAdded, cx));
                        }),
                )
                .item(
                    PopupMenuItem::new("Pages")
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
                    PopupMenuItem::new("Ascending")
                        .checked(direction == SortDirection::Ascending)
                        .on_click(move |_, _, cx| {
                            e5.update(cx, |ctrl, cx| {
                                ctrl.set_sort_direction(SortDirection::Ascending, cx);
                            });
                        }),
                )
                .item(
                    PopupMenuItem::new("Descending")
                        .checked(direction == SortDirection::Descending)
                        .on_click(move |_, _, cx| {
                            e6.update(cx, |ctrl, cx| {
                                ctrl.set_sort_direction(SortDirection::Descending, cx);
                            });
                        }),
                )
        })
}

// ── Group toggle ──────────────────────────────────────────────────────────────

fn render_group_toggle(
    grouped: bool,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    accent: gpui::Hsla,
    accent_soft: gpui::Hsla,
) -> impl IntoElement + 'static {
    let btn_bg = if grouped { accent_soft } else { bg };
    let text_color = if grouped { accent } else { text_primary };

    div()
        .id("group-toggle")
        .h(px(30.0))
        .px(px(11.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(btn_bg)
        .flex()
        .items_center()
        .cursor_pointer()
        .tooltip(|window, cx| Tooltip::new("Group by publisher").build(window, cx))
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.set_grouped(!grouped, cx));
        })
        .child(div().text_sm().text_color(text_color).child("Group"))
}

// ── Layout switcher ───────────────────────────────────────────────────────────

fn render_layout_switcher(
    current: CatalogPresentation,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    accent: gpui::Hsla,
    accent_soft: gpui::Hsla,
) -> impl IntoElement + 'static {
    let modes = [
        (CatalogPresentation::List, "layout-list", "List view"),
        (CatalogPresentation::Thumbs, "layout-thumbs", "Thumbnail view"),
        (CatalogPresentation::Grid, "layout-grid", "Grid view"),
    ];
    let labels = ["List", "Thumbs", "Grid"];

    let mut row = div()
        .flex()
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(bg)
        .overflow_hidden();

    for ((mode, id_str, tooltip_text), label) in modes.into_iter().zip(labels) {
        let is_active = current == mode;
        let btn_bg = if is_active {
            accent_soft
        } else {
            gpui::hsla(0.0, 0.0, 0.0, 0.0)
        };
        let text = if is_active { accent } else { text_primary };
        let e = entity.clone();

        row = row.child(
            div()
                .id(id_str)
                .h(px(28.0))
                .px(px(10.0))
                .bg(btn_bg)
                .flex()
                .items_center()
                .cursor_pointer()
                .tooltip(move |window, cx| Tooltip::new(tooltip_text).build(window, cx))
                .on_click(move |_, _, cx| {
                    e.update(cx, |ctrl, cx| ctrl.set_presentation(mode, cx));
                })
                .child(div().text_sm().text_color(text).child(label)),
        );
    }

    row
}

// ── Settings gear button ──────────────────────────────────────────────────────

fn render_settings_button(
    settings: Entity<SettingsController>,
    text_primary: gpui::Hsla,
    border: gpui::Hsla,
) -> impl IntoElement + 'static {
    div()
        .id("settings-gear")
        .h(px(30.0))
        .w(px(30.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .tooltip(|window, cx| Tooltip::new("Settings").build(window, cx))
        .on_click(move |_, _, cx| {
            settings.update(cx, |ctrl, cx| ctrl.toggle(cx));
        })
        .child(div().text_sm().text_color(text_primary).child("⚙"))
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
        return div()
            .id("avatar-btn")
            .h(px(30.0))
            .w(px(30.0))
            .rounded_full()
            .bg(colors.surface_alt)
            .border_1()
            .border_color(colors.border_strong)
            .flex()
            .items_center()
            .justify_center()
            .tooltip(|window, cx| Tooltip::new("Not signed in").build(window, cx))
            .child(div().text_xs().text_color(colors.text_tertiary).child("👤"))
            .into_any_element();
    }

    let initial_text = auth
        .display_initial
        .map(|c| c.to_string())
        .unwrap_or_else(|| "?".to_string());

    let accent = colors.accent;
    let avatar_variant = ButtonCustomVariant::new(cx)
        .color(accent)
        .foreground(gpui::white())
        .hover(gpui::Hsla { l: (accent.l * 0.85).min(1.0), ..accent })
        .active(gpui::Hsla { l: (accent.l * 0.75).min(1.0), ..accent });

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

    Button::new("avatar-btn")
        .custom(avatar_variant)
        .tooltip("Account")
        .rounded_full()
        .w(px(30.0))
        .h(px(30.0))
        .child(inner)
        .dropdown_menu(move |menu, _, _| {
            let s = settings.clone();
            menu.item(
                PopupMenuItem::new("Log Out").on_click(move |_, _, cx| {
                    s.update(cx, |ctrl, cx| ctrl.logout(cx));
                }),
            )
        })
        .into_any_element()
}
