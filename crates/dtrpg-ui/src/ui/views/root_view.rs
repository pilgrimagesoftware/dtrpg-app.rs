//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use gpui::{AppContext, div, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Render, Styled};
use gpui_component::input::{InputEvent, InputState};
use crate::ui::actions::ShowSettings;
use crate::ui::app::{LoginServiceFactory, ServiceFactory};

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
        events::{ActivityChanged, AuthStateChanged, LibraryChanged, LogoutRequested, SettingsChanged, SignInSucceeded},
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
/// Top-level GPUI view for the Libri library window.
pub struct LibraryRootView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    activity: Entity<ActivityController>,
    auth_state: Entity<AuthStateController>,
    catalog_view: Entity<CatalogView>,
    /// Default focus handle for the root div, ensuring menu-triggered actions
    /// always have a dispatch path even before any child element grabs focus.
    root_focus: FocusHandle,
    /// Focus handle for the settings overlay; grabbed when the panel opens so
    /// Escape key events route to the backdrop instead of the catalog.
    settings_focus: FocusHandle,
    /// Editable search input wired to the library controller's search query.
    search_input: Entity<InputState>,
}

impl LibraryRootView {
    /// Constructs the root view and wires up the controller subscriptions.
    ///
    /// `auth_state` reflects the outcome of startup re-authentication. The window
    /// always opens; the banner reflects whether the user is signed in.
    pub fn new(window: &mut gpui::Window, cx: &mut Context<Self>, service: Box<dyn LibraryService>, auth_state: AuthState) -> Self {
        let activity = cx.new(|_| ActivityController::new());
        let controller = cx.new(|cx| LibraryController::new(service, activity.clone(), cx));
        let login_service = cx.global::<LoginServiceFactory>().0();
        let settings = cx.new(|cx| {
            let mut ctrl = SettingsController::new(login_service, cx);
            if auth_state == AuthState::Authenticated {
                ctrl.set_logged_in(None, cx);
            }
            ctrl
        });

        let api_key_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Paste API key here\u{2026}")
        });
        let settings_for_input = settings.clone();
        cx.subscribe(&api_key_input, move |_this, input_entity, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                let value = input_entity.read(cx).value().to_string();
                settings_for_input.update(cx, |ctrl, cx| ctrl.set_api_key_draft(value, cx));
            }
        })
        .detach();
        settings.update(cx, |ctrl, _cx| ctrl.set_api_key_input(api_key_input));

        let storage_path_placeholder = {
            use crate::data::storage::StorageConfig;
            StorageConfig::load().root_path().to_string_lossy().into_owned()
        };
        let storage_path_input = cx.new(|cx| {
            InputState::new(window, cx).default_value(&storage_path_placeholder)
        });
        let settings_for_storage = settings.clone();
        cx.subscribe(&storage_path_input, move |_this, input_entity, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                let value = input_entity.read(cx).value().to_string();
                settings_for_storage.update(cx, |ctrl, cx| ctrl.set_storage_path_draft(value, cx));
            }
        })
        .detach();
        settings.update(cx, |ctrl, _cx| ctrl.set_storage_path_input(storage_path_input));

        let catalog_view = cx.new(|cx| CatalogView::new(window, cx, controller.clone(), settings.clone()));
        let auth_state = cx.new(|_| AuthStateController::new(auth_state));
        let root_focus = cx.focus_handle();
        root_focus.focus(window, cx);
        let settings_focus = cx.focus_handle();

        let search_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Search\u{2026}")
        });

        let controller_for_search = controller.clone();
        cx.subscribe(&search_input, move |_this, input_entity, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                let value = input_entity.read(cx).value().to_string();
                controller_for_search.update(cx, |ctrl, cx| {
                    ctrl.set_search_query(value, cx);
                });
            }
        })
        .detach();

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

        // Handle logout: delete the API key and transition to unauthenticated.
        // Tokens are in-memory only and need no explicit deletion.
        let auth_state_for_logout = auth_state.clone();
        cx.subscribe(&settings, move |_this, _ctrl, _event: &LogoutRequested, cx| {
            let store = KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY);
            if let Err(e) = store.delete() {
                tracing::warn!("credential delete (api-key): {e}");
            }
            auth_state_for_logout.update(cx, |ctrl, cx| ctrl.set_state(AuthState::Unauthenticated, cx));
        })
        .detach();

        // Handle sign-in: replace the library service and mark as authenticated.
        let auth_state_for_signin = auth_state.clone();
        let controller_for_signin = controller.clone();
        cx.subscribe(&settings, move |_this, _ctrl, event: &SignInSucceeded, cx| {
            let tokens = event.0.clone();
            let service = cx.global::<ServiceFactory>().0.as_ref()(Some(tokens));
            controller_for_signin.update(cx, |ctrl, cx| ctrl.replace_service(service, cx));
            auth_state_for_signin.update(cx, |ctrl, cx| ctrl.set_state(AuthState::Authenticated, cx));
        })
        .detach();

        Self { controller, settings, activity, auth_state, catalog_view, root_focus, settings_focus, search_input }
    }
}

impl Focusable for LibraryRootView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.root_focus.clone()
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
             sort, sort_direction, grouped, presentation, selected_item) = (
            snap.filter, snap.counts, snap.publishers, snap.total_count, snap.total_mb,
            snap.matched_count, snap.sort, snap.sort_direction, snap.grouped,
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
            self.search_input.clone(),
            sort,
            sort_direction,
            grouped,
            presentation,
            lib_entity.clone(),
            settings_entity.clone(),
            &settings_snap.auth,
            colors,
            cx,
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
                // Focus the backdrop on first open so Escape works immediately.
                // Don't steal focus if a child (e.g. the API key input) already has it.
                if !self.settings_focus.contains_focused(window, cx) {
                    window.focus(&self.settings_focus, cx);
                }
                let overlay = render_settings_panel(
                    settings_snap.active_tab,
                    &settings_snap.file_openers,
                    settings_snap.is_authenticated,
                    settings_snap.auth,
                    settings_snap.storage_root_path,
                    settings_snap.storage_path_exists,
                    settings_entity,
                    &self.settings_focus,
                    colors,
                    settings_snap.api_key_input,
                    settings_snap.sign_in_in_progress,
                    settings_snap.sign_in_error,
                    settings_snap.storage_path_input,
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

        let settings_for_action = self.settings.clone();

        div()
            .size_full()
            .bg(surface)
            .text_color(text_primary)
            .flex()
            .relative()
            .track_focus(&self.root_focus)
            .on_action(move |_: &ShowSettings, _, cx| {
                settings_for_action.update(cx, |ctrl, cx| ctrl.open(cx));
            })
            .child(sidebar_col)
            .child(main_content)
            .child(panel)
    }
}
