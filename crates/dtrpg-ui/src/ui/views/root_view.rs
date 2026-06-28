//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use gpui::{AppContext, div, Context, Entity, FocusHandle, IntoElement, ParentElement, Render, Styled};

use crate::{
    controllers::{
        activity::ActivityController,
        auth_state::AuthStateController,
        library::LibraryController,
        settings::SettingsController,
    },
    credentials::{CredentialStore, KeyringCredentialStore, keys},
    data::{
        auth_state::AuthState,
        events::{ActivityChanged, AuthStateChanged, LibraryChanged, LogoutRequested, SettingsChanged},
        theme::LibriTheme,
    },
    services::LibraryService,
};
use crate::ui::views::{
    activity_panel_view::render_activity_panel,
    catalog_view::CatalogView,
    detail_panel_view::render_detail_panel,
    notification_banner_view::render_notification_banner,
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
    auth_state: Entity<AuthStateController>,
    catalog_view: Entity<CatalogView>,
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
        let catalog_view = cx.new(|_| CatalogView::new(controller.clone(), settings.clone()));
        let auth_initial = {
            #[cfg(debug_assertions)]
            {
                match std::env::var("DTRPG_AUTH_STATE_OVERRIDE").as_deref().unwrap_or("") {
                    "unauthenticated" => AuthState::Unauthenticated,
                    "expired" => AuthState::SessionExpired,
                    _ => AuthState::Authenticated,
                }
            }
            #[cfg(not(debug_assertions))]
            { AuthState::Authenticated }
        };
        let auth_state = cx.new(|_| AuthStateController::new(auth_initial));
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

        cx.subscribe(&auth_state, |_this, _ctrl, _event: &AuthStateChanged, cx| {
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
            open_login_window(None, cx);
            cx.with_window(entity_id, |window, _cx| window.remove_window());
        })
        .detach();

        // Stub: simulate a signed-in user so the avatar button renders during development.
        #[cfg(debug_assertions)]
        settings.update(cx, |ctrl, cx| {
            ctrl.set_logged_in("test@example.com".into(), cx);
        });

        Self { controller, settings, activity, auth_state, catalog_view, settings_focus }
    }
}

impl Render for LibraryRootView {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let lib_entity = self.controller.clone();
        let settings_entity = self.settings.clone();
        let activity_entity = self.activity.clone();
        let auth_entity = self.auth_state.clone();

        let snap = self.controller.read(cx).snapshot();
        let (filter, counts, publishers, total_count, total_mb, matched_count,
             search_query, sort, grouped, presentation, selected_item) = (
            snap.filter, snap.counts, snap.publishers, snap.total_count, snap.total_mb,
            snap.matched_count, snap.search_query, snap.sort, snap.grouped,
            snap.presentation, snap.selected_item,
        );

        let settings_snap = self.settings.read(cx).snapshot();
        let activity_snap = self.activity.read(cx).snapshot();
        let notices: Vec<_> = self.auth_state.read(cx).active_notices()
            .into_iter()
            .cloned()
            .collect();

        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;

        let sidebar = render_sidebar(
            &filter,
            counts,
            &publishers,
            total_count,
            total_mb,
            lib_entity.clone(),
            activity_entity.clone(),
            activity_snap.in_progress_count,
            activity_snap.recent_count,
            activity_snap.recent_error_count,
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
            &settings_snap.auth,
            colors,
        );
        let banner = render_notification_banner(notices, auth_entity, settings_entity.clone(), colors);
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
                .child(banner)
                .child(self.catalog_view.clone());

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
