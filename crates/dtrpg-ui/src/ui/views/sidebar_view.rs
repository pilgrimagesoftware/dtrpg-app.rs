//! Sidebar view: wordmark, smart nav, publisher nav, collections nav, storage footer.

use std::collections::HashSet;
use std::sync::Arc;

use gpui::prelude::*;
use gpui::{App, ElementId, Entity, IntoElement, Window, div, px};
use gpui_component::sidebar::{
    Sidebar, SidebarCollapsible, SidebarFooter, SidebarHeader, SidebarItem, SidebarMenu,
    SidebarMenuItem,
};
use gpui_component::{ActiveTheme, Collapsible, Side};

use crate::controllers::activity::ActivityController;
use crate::controllers::library::LibraryController;
use crate::data::collection::CollectionEntry;
use crate::data::library::SectionCounts;
use crate::util::filter::SidebarFilter;
use crate::util::publisher::PublisherEntry;

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
) -> impl IntoElement + 'static {
    let active = filter.clone();

    // ── Library smart-filter menu ─────────────────────────────────────────────
    let lib_menu = SidebarMenu::new()
        .child(nav_item(
            "All Titles",
            counts.all,
            active == SidebarFilter::AllTitles,
            SidebarFilter::AllTitles,
            entity.clone(),
        ))
        .child(nav_item(
            "Recently Added",
            counts.recently_added,
            active == SidebarFilter::RecentlyAdded,
            SidebarFilter::RecentlyAdded,
            entity.clone(),
        ))
        .child(nav_item(
            "On This Device",
            counts.on_device,
            active == SidebarFilter::OnDevice,
            SidebarFilter::OnDevice,
            entity.clone(),
        ))
        .child(nav_item(
            "In the Cloud",
            counts.in_cloud,
            active == SidebarFilter::InCloud,
            SidebarFilter::InCloud,
            entity.clone(),
        ));

    // ── Publishers menu ───────────────────────────────────────────────────────
    let pub_children: Vec<SidebarMenuItem> = publishers
        .into_iter()
        .map(|p| {
            let is_active = active == SidebarFilter::Publisher(Arc::clone(&p.name));
            let f = SidebarFilter::Publisher(Arc::clone(&p.name));
            nav_item(p.name.as_ref(), p.count, is_active, f, entity.clone())
        })
        .collect();

    let pub_menu = SidebarMenu::new().child(
        SidebarMenuItem::new("Publishers")
            .click_to_toggle(true)
            .default_open(true)
            .children(pub_children),
    );

    // ── Sidebar assembly ──────────────────────────────────────────────────────
    let mut sidebar_builder = Sidebar::new("sidebar")
        .collapsible(SidebarCollapsible::None)
        .side(Side::Left)
        .w(px(250.))
        .header(build_header())
        .child(SidebarContent::Menu(Box::new(lib_menu)))
        .child(SidebarContent::Separator)
        .child(SidebarContent::Menu(Box::new(pub_menu)));

    // ── Collections menu (always present) ────────────────────────────────────
    let col_children: Vec<SidebarMenuItem> = collections
        .into_iter()
        .map(|c| {
            let is_active = active == SidebarFilter::Collection(c.id);
            let f = SidebarFilter::Collection(c.id);
            let count = c
                .member_ids
                .iter()
                .filter(|id| catalog_ids.contains(id))
                .count();
            nav_item(c.name.as_ref(), count, is_active, f, entity.clone())
        })
        .collect();

    let col_menu = SidebarMenu::new().child(
        SidebarMenuItem::new("Collections")
            .click_to_toggle(true)
            .default_open(true)
            .children(col_children),
    );

    sidebar_builder = sidebar_builder
        .child(SidebarContent::Separator)
        .child(SidebarContent::Menu(Box::new(col_menu)));

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
    SidebarHeader::new().child(
        div()
            .text_xl()
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .child("Libri"),
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
                    .child(format!("{total_count} titles"))
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
