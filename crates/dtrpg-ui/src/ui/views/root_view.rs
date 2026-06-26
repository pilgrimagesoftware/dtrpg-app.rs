//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use gpui::{div, AppContext, Context, Entity, FocusHandle, IntoElement, ParentElement, Render, Styled};

use crate::{
    controllers::{library::LibraryController, settings::SettingsController},
    data::{
        events::{LibraryChanged, SettingsChanged},
        theme::LibriTheme,
    },
};
use crate::ui::views::{
    catalog_view::render_catalog,
    detail_panel_view::render_detail_panel,
    settings_view::render_settings_panel,
    sidebar_view::render_sidebar,
    toolbar_view::render_toolbar,
};

/// Top-level GPUI view for the Libri library window.
pub struct LibraryRootView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    /// Focus handle for the settings overlay; grabbed when the panel opens so
    /// Escape key events route to the backdrop instead of the catalog.
    settings_focus: FocusHandle,
}

impl LibraryRootView {
    /// Constructs the root view and wires up the controller subscriptions.
    pub fn new(_window: &mut gpui::Window, cx: &mut Context<Self>) -> Self {
        let controller = cx.new(|_| LibraryController::new());
        let settings = cx.new(|_| SettingsController::new());
        let settings_focus = cx.focus_handle();

        cx.subscribe(&controller, |_this, _ctrl, _event: &LibraryChanged, cx| {
            cx.notify();
        })
        .detach();

        cx.subscribe(&settings, |_this, _ctrl, _event: &SettingsChanged, cx| {
            cx.notify();
        })
        .detach();

        Self { controller, settings, settings_focus }
    }
}

impl Render for LibraryRootView {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let lib_entity = self.controller.clone();
        let settings_entity = self.settings.clone();

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

        let settings_snap = self.settings.read(cx).snapshot();

        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;
        let density = &theme.density_constants;

        let sidebar = render_sidebar(
            &filter,
            counts,
            &publishers,
            total_count,
            total_mb,
            lib_entity.clone(),
            colors,
        );
        let toolbar = render_toolbar(
            &filter,
            matched_count,
            &search_query,
            sort,
            grouped,
            presentation,
            lib_entity.clone(),
            settings_entity.clone(),
            colors,
        );
        let catalog = render_catalog(items, presentation, grouped, lib_entity.clone(), colors, density);
        let panel = render_detail_panel(selected_item.as_ref(), lib_entity, colors);

        let surface = colors.surface;
        let text_primary = colors.text_primary;

        // Settings overlay is rendered inside the main content area so the
        // sidebar remains visible behind it.
        let main_content = {
            let mut content = div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .relative()
                .bg(surface)
                .child(toolbar)
                .child(catalog);

            if settings_snap.is_open {
                // Grab keyboard focus so Escape routes to the backdrop, not the catalog.
                window.focus(&self.settings_focus, cx);
                let overlay = render_settings_panel(
                    settings_snap.active_tab,
                    &settings_snap.file_openers,
                    settings_entity,
                    &self.settings_focus,
                    colors,
                );
                content = content.child(overlay);
            }

            content
        };

        div()
            .size_full()
            .bg(surface)
            .text_color(text_primary)
            .flex()
            .relative()
            .child(sidebar)
            .child(main_content)
            .child(panel)
    }
}
