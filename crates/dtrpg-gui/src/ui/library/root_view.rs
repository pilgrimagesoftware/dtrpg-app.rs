//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use gpui::{div, px, Context, IntoElement, ParentElement, Render, Styled};

use crate::ui::library::{
    catalog_view::render_catalog,
    detail_panel_view::render_detail_panel,
    sidebar_view::render_sidebar,
    state::{LibraryChanged, LibraryController},
    toolbar_view::render_toolbar,
    theme::LibriTheme,
};

/// Top-level GPUI view for the Libri library window.
pub struct LibraryRootView {
    controller: Entity<LibraryController>,
}

impl LibraryRootView {
    /// Constructs the root view and wires up the controller subscription.
    pub fn new(_window: &mut gpui::Window, cx: &mut Context<Self>) -> Self {
        let controller = cx.new(|_| LibraryController::new());

        // Re-render whenever the controller emits a change.
        cx.subscribe(&controller, |_this, _ctrl, _event: &LibraryChanged, cx| {
            cx.notify();
        })
        .detach();

        Self { controller }
    }
}

impl Render for LibraryRootView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = self.controller.clone();

        // Read immutable controller state.
        let (
            filter,
            counts,
            publishers,
            total_count,
            total_mb,
            matched_count,
            search_query,
            sort,
            grouped,
            presentation,
            selected_item,
            items,
        ) = self.controller.read(cx).snapshot();

        // Read theme global.
        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;
        let density = &theme.density_constants;

        let sidebar = render_sidebar(
            &filter,
            counts,
            &publishers,
            total_count,
            total_mb,
            entity.clone(),
            colors,
        );
        let toolbar = render_toolbar(
            &filter,
            matched_count,
            &search_query,
            sort,
            grouped,
            presentation,
            entity.clone(),
            colors,
        );
        let catalog = render_catalog(items, presentation, grouped, entity.clone(), colors, density);
        let panel = render_detail_panel(selected_item.as_ref(), entity.clone(), colors);

        let desktop_bg = colors.desktop_bg;
        let surface = colors.surface;

        div()
            .size_full()
            .bg(desktop_bg)
            .flex()
            .items_center()
            .justify_center()
            .p(px(24.0))
            .child(
                div()
                    .max_w(px(1320.0))
                    .max_h(px(862.0))
                    .w_full()
                    .h_full()
                    .rounded(px(14.0))
                    .overflow_hidden()
                    .bg(surface)
                    .flex()
                    .relative()
                    .child(sidebar)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .bg(surface)
                            .child(toolbar)
                            .child(catalog),
                    )
                    .child(panel),
            )
    }
}
