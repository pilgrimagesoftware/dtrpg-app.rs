//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use gpui::{AppContext, div, Context, Entity, FocusHandle, IntoElement, ParentElement, Render, Styled};

use crate::{
    controllers::{activity::ActivityController, library::LibraryController, settings::SettingsController},
    credentials::{CredentialStore, KeyringCredentialStore, keys},
    data::{
        events::{ActivityChanged, LibraryChanged, LogoutRequested, SettingsChanged},
        theme::LibriTheme,
    },
    services::LibraryService,
};
use crate::ui::views::{
    activity_panel_view::render_activity_panel,
    catalog_view::render_catalog,
    detail_panel_view::render_detail_panel,
    settings_view::render_settings_panel,
    sidebar_view::render_sidebar,
    toolbar_view::render_toolbar,
};
use crate::ui::windows::login::open_login_window;

/// Top-level GPUI view for the Libri library window.
pub struct LibraryRootView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    activity: Entity<ActivityController>,
    /// Focus handle for the settings overlay; grabbed when the panel opens so
    /// Escape key events route to the backdrop instead of the catalog.
    settings_focus: FocusHandle,
}

impl LibraryRootView {
    /// Constructs the root view and wires up the controller subscriptions.
    pub fn new(_window: &mut gpui::Window, cx: &mut Context<Self>, service: Box<dyn LibraryService>) -> Self {
        let activity = cx.new(|_| ActivityController::new());
        let controller = cx.new(|cx| LibraryController::new(service, activity.clone(), cx));
        let settings = cx.new(|_| SettingsController::new());
        let settings_focus = cx.focus_handle();

        cx.subscribe(&controller, |_this, _ctrl, _event: &LibraryChanged, cx| {
            cx.notify();
        })
        .detach();

        cx.subscribe(&activity, |_this, _ctrl, _event: &ActivityChanged, cx| {
            cx.notify();
        })
        .detach();

        cx.subscribe(&settings, |_this, _ctrl, _event: &SettingsChanged, cx| {
            cx.notify();
        })
        .detach();

        // Handle logout: delete credentials, open login window, close library window.
        cx.subscribe(&settings, |_this, _ctrl, _event: &LogoutRequested, cx| {
            for account in [keys::API_KEY, keys::ACCESS_TOKEN, keys::REFRESH_TOKEN] {
                let store = KeyringCredentialStore::new(keys::SERVICE, account);
                if let Err(e) = store.delete() {
                    tracing::warn!("credential delete ({account}): {e}");
                }
            }
            let entity_id = cx.entity_id();
            open_login_window(cx);
            cx.with_window(entity_id, |window, _cx| window.remove_window());
        })
        .detach();

        Self { controller, settings, activity, settings_focus }
    }
}

impl Render for LibraryRootView {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let lib_entity = self.controller.clone();
        let settings_entity = self.settings.clone();
        let activity_entity = self.activity.clone();

        let snap = self.controller.read(cx).snapshot();
        let (filter, counts, publishers, total_count, total_mb, matched_count,
             search_query, sort, grouped, presentation, selected_item, items) = (
            snap.filter, snap.counts, snap.publishers, snap.total_count, snap.total_mb,
            snap.matched_count, snap.search_query, snap.sort, snap.grouped,
            snap.presentation, snap.selected_item, snap.items,
        );

        let settings_snap = self.settings.read(cx).snapshot();
        let activity_snap = self.activity.read(cx).snapshot();

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
            activity_entity.clone(),
            activity_snap.in_progress_count,
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
        let catalog = render_catalog(items, presentation, grouped, lib_entity.clone(), colors, density, settings_snap.storage_root_path.clone());
        let panel = render_detail_panel(selected_item.as_ref(), settings_snap.storage_root_path.clone(), lib_entity, colors);

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
                    settings_snap.is_authenticated,
                    settings_snap.storage_root_path,
                    settings_entity,
                    &self.settings_focus,
                    colors,
                );
                content = content.child(overlay);
            }

            content
        };

        // Wrap the sidebar in a relative container so the activity panel overlay
        // (which is absolute-positioned) is anchored within the sidebar column.
        let mut sidebar_col = div()
            .flex_none()
            .relative()
            .child(sidebar);

        if activity_snap.panel_open {
            let overlay = render_activity_panel(&activity_snap, activity_entity, colors);
            sidebar_col = sidebar_col.child(overlay);
        }

        div()
            .size_full()
            .bg(surface)
            .text_color(text_primary)
            .flex()
            .relative()
            .child(sidebar_col)
            .child(main_content)
            .child(panel)
    }
}
