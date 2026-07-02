//! Sidebar view: wordmark, smart nav, publisher nav, collections nav, storage footer.

use std::collections::HashSet;
use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, Context, ElementId, Entity, IntoElement, SharedString, Window, div, px,
};
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::dialog::{Dialog, DialogButtonProps, DialogHeader, DialogTitle};
use gpui_component::input::{Input, InputState};
use gpui_component::menu::PopupMenuItem;
use gpui_component::sidebar::{
    Sidebar, SidebarCollapsible, SidebarFooter, SidebarHeader, SidebarItem, SidebarMenu,
    SidebarMenuItem,
};
use gpui_component::{ActiveTheme, Collapsible, Side, Sizable as _};

use crate::controllers::activity::ActivityController;
use crate::controllers::library::LibraryController;
use crate::data::collection::CollectionEntry;
use crate::data::library::SectionCounts;
use crate::data::ui_prefs::UiPrefs;
use crate::util::filter::SidebarFilter;
use crate::util::matching::name_matches_query;
use crate::util::pluralize::pluralize;
use crate::util::publisher::PublisherEntry;
use rust_i18n::t;

/// Inline search bar state for a collapsible sidebar section (Publishers or Collections).
///
/// Session-only: the search query is never persisted and is cleared whenever the
/// section's search bar is collapsed.
pub struct SidebarSectionSearch {
    /// Whether the section's search bar is currently expanded.
    pub open: bool,
    /// Current filter text; empty means no filtering.
    pub query: String,
    /// Backing text input, owned by the root view so its subscription lives
    /// for the lifetime of the window.
    pub input: Entity<InputState>,
}

/// A sidebar child that is either a [`SidebarMenu`] or a thin horizontal divider.
#[derive(Clone)]
enum SidebarContent {
    Menu(Box<SidebarMenu>),
    Separator,
}

impl Collapsible for SidebarContent {
    fn collapsed(self, collapsed: bool) -> Self {
        match self {
            Self::Menu(m) => Self::Menu(Box::new(m.collapsed(collapsed))),
            Self::Separator => Self::Separator,
        }
    }

    fn is_collapsed(&self) -> bool {
        match self {
            Self::Menu(m) => m.is_collapsed(),
            Self::Separator => false,
        }
    }
}

impl SidebarItem for SidebarContent {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        match self {
            Self::Menu(m) => m.render(id, window, cx).into_any_element(),
            Self::Separator => div()
                .h(px(1.))
                .w_full()
                .my_1()
                .bg(cx.theme().sidebar_border)
                .into_any_element(),
        }
    }
}

/// Renders the full sidebar column using gpui-component `Sidebar`.
#[allow(clippy::too_many_arguments)]
pub fn render_sidebar(
    filter: SidebarFilter,
    counts: SectionCounts,
    publishers: Vec<PublisherEntry>,
    collections: Vec<CollectionEntry>,
    catalog_ids: HashSet<u64>,
    total_count: usize,
    total_mb: f64,
    entity: Entity<LibraryController>,
    activity_entity: Entity<ActivityController>,
    activity_in_progress: usize,
    activity_recent_count: usize,
    activity_recent_error_count: usize,
    collection_name_input: Entity<InputState>,
    publisher_search: SidebarSectionSearch,
    collection_search: SidebarSectionSearch,
) -> impl IntoElement + 'static {
    let active = filter.clone();
    let prefs = UiPrefs::load();
    let publishers_open = prefs.publishers_open();
    let collections_open = prefs.collections_open();

    // ── Library smart-filter menu ─────────────────────────────────────────────
    let lib_menu = SidebarMenu::new()
        .child(nav_item(
            &t!("sidebar.all_titles"),
            counts.all,
            active == SidebarFilter::AllTitles,
            SidebarFilter::AllTitles,
            entity.clone(),
        ))
        .child(nav_item(
            &t!("sidebar.recently_added"),
            counts.recently_added,
            active == SidebarFilter::RecentlyAdded,
            SidebarFilter::RecentlyAdded,
            entity.clone(),
        ))
        .child(nav_item(
            &t!("sidebar.on_this_device"),
            counts.on_device,
            active == SidebarFilter::OnDevice,
            SidebarFilter::OnDevice,
            entity.clone(),
        ))
        .child(nav_item(
            &t!("sidebar.in_the_cloud"),
            counts.in_cloud,
            active == SidebarFilter::InCloud,
            SidebarFilter::InCloud,
            entity.clone(),
        ));

    let publishers_count = publishers.len();

    // ── Publishers menu ───────────────────────────────────────────────────────
    let pub_children: Vec<SidebarMenuItem> = publishers
        .into_iter()
        .filter(|p| name_matches_query(p.name.as_ref(), &publisher_search.query))
        .map(|p| {
            let is_active = active == SidebarFilter::Publisher(Arc::clone(&p.name));
            let f = SidebarFilter::Publisher(Arc::clone(&p.name));
            nav_item(p.name.as_ref(), p.count, is_active, f, entity.clone())
        })
        .collect();

    let entity_for_pub = entity.clone();
    let pub_title: SharedString = if publisher_search.open {
        "".into()
    } else {
        t!("sidebar.publishers").into()
    };
    let pub_menu = SidebarMenu::new().child(
        SidebarMenuItem::new(pub_title)
            .default_open(publishers_open)
            .click_to_toggle(true)
            .on_click(move |_, _, cx| {
                UiPrefs::load().save_publishers_open(!publishers_open);
                entity_for_pub.update(cx, |ctrl, cx| ctrl.notify_ui_change(cx));
            })
            .suffix(render_section_search_suffix(
                "publishers",
                publishers_count,
                publisher_search,
                entity.clone(),
                LibraryController::toggle_publisher_search,
            ))
            .children(pub_children),
    );

    // ── Sidebar assembly (collections before publishers) ──────────────────────
    // `w_full()` rather than a fixed pixel width: the parent `resizable_panel`
    // in `root_view.rs` already owns width management (its divider IS the drag
    // handle — no separate resize control). A fixed width here would leave the
    // visible sidebar content pinned regardless of the panel's actual dragged
    // width, decoupling what you drag from where the sidebar and catalog
    // actually meet.
    let mut sidebar_builder = Sidebar::new("sidebar")
        .collapsible(SidebarCollapsible::None)
        .side(Side::Left)
        .w_full()
        .header(build_header())
        .child(SidebarContent::Menu(Box::new(lib_menu)))
        .child(SidebarContent::Separator);

    // ── Collections menu (always present) ────────────────────────────────────
    let collections_count = collections.len();

    let col_children: Vec<SidebarMenuItem> = collections
        .into_iter()
        .filter(|c| name_matches_query(c.name.as_ref(), &collection_search.query))
        .map(|c| {
            let is_active = matches!(&active, SidebarFilter::Collection(id, _) if *id == c.id);
            let f = SidebarFilter::Collection(c.id, Arc::clone(&c.name));
            let count = c
                .member_ids
                .iter()
                .filter(|id| catalog_ids.contains(id))
                .count();
            let col_id = c.id;
            let entity_reload = entity.clone();
            let entity_delete = entity.clone();
            nav_item(c.name.as_ref(), count, is_active, f, entity.clone()).context_menu(
                move |menu, _, _| {
                    menu.item(PopupMenuItem::new(t!("collections.reload")).on_click({
                        let entity = entity_reload.clone();
                        move |_, _, cx| {
                            entity.update(cx, |ctrl, cx| ctrl.load_collections(cx));
                        }
                    }))
                    .item(
                        PopupMenuItem::new(t!("collections.delete")).on_click({
                            let entity = entity_delete.clone();
                            move |_, _, cx| {
                                entity.update(cx, |ctrl, cx| ctrl.delete_collection(col_id, cx));
                            }
                        }),
                    )
                },
            )
        })
        .collect();

    let entity_for_col = entity.clone();
    let col_title: SharedString = if collection_search.open {
        "".into()
    } else {
        t!("sidebar.collections").into()
    };
    let collection_search_open = collection_search.open;
    let collection_search_input = collection_search.input.clone();
    let col_menu = SidebarMenu::new().child(
        SidebarMenuItem::new(col_title)
            .default_open(collections_open)
            .click_to_toggle(true)
            .on_click(move |_, _, cx| {
                UiPrefs::load().save_collections_open(!collections_open);
                entity_for_col.update(cx, |ctrl, cx| ctrl.notify_ui_change(cx));
            })
            .suffix({
                let input = collection_name_input.clone();
                let ctrl = entity.clone();
                let search_input = collection_search_input.clone();
                let entity_for_toggle = entity.clone();
                move |_window, cx| {
                    let input = input.clone();
                    let ctrl = ctrl.clone();

                    if collection_search_open {
                        let entity_for_close = entity_for_toggle.clone();
                        let search_input_for_close = search_input.clone();
                        return div()
                            .id("collections-search-suffix")
                            .on_click(|_, _, cx| cx.stop_propagation())
                            .flex()
                            .items_center()
                            .w_full()
                            .gap(px(4.))
                            .child(div().flex_1().child(Input::new(&search_input).small()))
                            .child(
                                Button::new("collections-search-close")
                                    .ghost()
                                    .compact()
                                    .icon(IconName::Close)
                                    .on_click(move |_, window, cx| {
                                        entity_for_close.update(cx, |ctrl, cx| {
                                            ctrl.toggle_collection_search(cx);
                                        });
                                        search_input_for_close.update(cx, |st, cx| {
                                            st.set_value("", window, cx);
                                        });
                                    }),
                            )
                            .into_any_element();
                    }

                    div()
                        .id("collections-suffix")
                        // Stop click propagation so the suffix button does not also
                        // fire the SidebarMenuItem header's on_click (which toggles collapse).
                        .on_click(|_, _, cx| cx.stop_propagation())
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .child(div().text_xs().child(collections_count.to_string()))
                        .child(
                            Button::new("collections-search-open")
                                .ghost()
                                .compact()
                                .icon(IconName::Search)
                                .tooltip(t!("sidebar.search_tooltip").to_string())
                                .on_click({
                                    let entity = entity_for_toggle.clone();
                                    let input = search_input.clone();
                                    move |_, window, cx| {
                                        entity.update(cx, |ctrl, cx| {
                                            ctrl.toggle_collection_search(cx);
                                        });
                                        input.update(cx, |st, cx| st.focus(window, cx));
                                    }
                                }),
                        )
                        .child(
                            Dialog::new(cx)
                                .trigger(
                                    Button::new("add-collection")
                                        .ghost()
                                        .compact()
                                        .icon(IconName::Plus)
                                        .tooltip(t!("collections.add_tooltip").to_string()),
                                )
                                .w(px(320.))
                                .close_button(false)
                                .overlay_closable(true)
                                .button_props(
                                    DialogButtonProps::default()
                                        .ok_text(t!("collections.add_dialog_confirm").to_string())
                                        .show_cancel(true)
                                        .cancel_text(
                                            t!("collections.add_dialog_cancel").to_string(),
                                        ),
                                )
                                .on_ok({
                                    let input = input.clone();
                                    let ctrl = ctrl.clone();
                                    move |_, _, cx| {
                                        let name = input.read(cx).value().trim().to_string();
                                        if name.is_empty() {
                                            return false;
                                        }
                                        ctrl.update(cx, |c, cx| c.create_collection(name, cx));
                                        true
                                    }
                                })
                                .on_cancel(|_, _, _| true)
                                .content({
                                    let input = collection_name_input.clone();
                                    move |content, _, _| {
                                        content
                                            .child(
                                                DialogHeader::new().px_4().pt_4().child(
                                                    DialogTitle::new()
                                                        .child(t!("collections.add_dialog_title")),
                                                ),
                                            )
                                            .child(div().px_4().py_2().child(Input::new(&input)))
                                    }
                                }),
                        )
                        .into_any_element()
                }
            })
            .children(col_children),
    );

    sidebar_builder = sidebar_builder
        .child(SidebarContent::Menu(Box::new(col_menu)))
        .child(SidebarContent::Separator)
        .child(SidebarContent::Menu(Box::new(pub_menu)));

    sidebar_builder.footer(build_footer(
        total_count,
        total_mb,
        activity_entity,
        activity_in_progress,
        activity_recent_count,
        activity_recent_error_count,
    ))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_header() -> SidebarHeader {
    // Remove the default SidebarHeader top padding so the wordmark aligns with
    // the macOS traffic lights (the Sidebar wrapper already adds pt_3 = 12px).
    SidebarHeader::new().pt_0().child(
        div()
            .text_xl()
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .child(t!("sidebar.app_name")),
    )
}

fn build_footer(
    total_count: usize,
    total_mb: f64,
    activity_entity: Entity<ActivityController>,
    activity_in_progress: usize,
    activity_recent_count: usize,
    activity_recent_error_count: usize,
) -> SidebarFooter {
    let total_size_str = if total_mb >= 1024.0 {
        format!("{:.1} GB", total_mb / 1024.0)
    } else {
        format!("{:.0} MB", total_mb)
    };

    let total = activity_in_progress + activity_recent_count;
    let activity_label = if activity_in_progress > 0 {
        format!("\u{21bb} ({total})")
    } else if activity_recent_count > 0 {
        format!("\u{25cf} ({total})")
    } else {
        "\u{25cb}".to_string()
    };
    let has_errors = activity_recent_error_count > 0;

    SidebarFooter::new().child(
        div()
            .w_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .child(pluralize(total_count, "title", "titles"))
                    .child(total_size_str),
            )
            .child(
                div()
                    .id("activity-button")
                    .text_sm()
                    .cursor_pointer()
                    .when(has_errors, |el| el.text_color(gpui::red()))
                    .child(activity_label)
                    .on_click(move |_, _, cx| {
                        activity_entity.update(cx, |a, cx| a.toggle_panel(cx));
                    }),
            ),
    )
}

fn nav_item(
    label: &str,
    count: usize,
    is_active: bool,
    filter: SidebarFilter,
    entity: Entity<LibraryController>,
) -> SidebarMenuItem {
    let label = label.to_string();
    SidebarMenuItem::new(label)
        .active(is_active)
        .suffix(move |_, _| div().text_xs().child(count.to_string()))
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.set_filter(filter.clone(), cx));
        })
}

/// Builds a `SidebarMenuItem` suffix that toggles between a plain item count and a
/// full-width search input, for sections like Publishers whose header carries no
/// other actions.
///
/// When `search.open` is `false`, renders the item count plus a magnifying-glass
/// button. When `true`, renders the search input (backed by `search.input`) plus a
/// close button that toggles the section closed and clears the input's visible text.
fn render_section_search_suffix(
    id_prefix: &'static str,
    count: usize,
    search: SidebarSectionSearch,
    entity: Entity<LibraryController>,
    toggle: impl Fn(&mut LibraryController, &mut Context<LibraryController>) + Copy + 'static,
) -> impl Fn(&mut Window, &mut App) -> AnyElement + 'static {
    move |_window, _cx| {
        if search.open {
            let input = search.input.clone();
            let entity_for_close = entity.clone();
            let input_for_close = input.clone();
            div()
                .id(SharedString::from(format!("{id_prefix}-search-suffix")))
                .on_click(|_, _, cx| cx.stop_propagation())
                .flex()
                .items_center()
                .w_full()
                .gap(px(4.))
                .child(div().flex_1().child(Input::new(&input).small()))
                .child(
                    Button::new(SharedString::from(format!("{id_prefix}-search-close")))
                        .ghost()
                        .compact()
                        .icon(IconName::Close)
                        .on_click(move |_, window, cx| {
                            entity_for_close.update(cx, |ctrl, cx| toggle(ctrl, cx));
                            input_for_close.update(cx, |st, cx| st.set_value("", window, cx));
                        }),
                )
                .into_any_element()
        } else {
            let entity_for_open = entity.clone();
            let input_for_open = search.input.clone();
            div()
                .id(SharedString::from(format!("{id_prefix}-search-suffix")))
                .on_click(|_, _, cx| cx.stop_propagation())
                .flex()
                .items_center()
                .gap(px(4.))
                .child(div().text_xs().child(count.to_string()))
                .child(
                    Button::new(SharedString::from(format!("{id_prefix}-search-open")))
                        .ghost()
                        .compact()
                        .icon(IconName::Search)
                        .tooltip(t!("sidebar.search_tooltip").to_string())
                        .on_click(move |_, window, cx| {
                            entity_for_open.update(cx, |ctrl, cx| toggle(ctrl, cx));
                            input_for_open.update(cx, |st, cx| st.focus(window, cx));
                        }),
                )
                .into_any_element()
        }
    }
}
