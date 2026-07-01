//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use crate::ui::actions::ShowSettings;
use crate::ui::app::{CollectionsServiceFactory, LoginServiceFactory, ServiceFactory};
use gpui::{
    AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, Styled, div, prelude::FluentBuilder as _, px,
};
use gpui_component::WindowExt as _;
use gpui_component::input::{InputEvent, InputState};
use gpui_component::notification::{Notification, NotificationType};
use gpui_component::resizable::{ResizableState, h_resizable, resizable_panel};

use crate::ui::views::{
    activity_panel_view::render_activity_panel, catalog_view::CatalogView,
    detail_panel_view::render_detail_panel, notification_banner_view::render_notification_banner,
    settings_view::render_settings_panel, sidebar_view::render_sidebar,
    toolbar_view::render_toolbar,
};
use crate::{
    controllers::{
        activity::ActivityController, auth_state::AuthStateController, library::LibraryController,
        settings::SettingsController,
    },
    credentials::{CredentialStore, KeyringCredentialStore},
    data::{
        constants::{KEYRING_SERVICE, KEYRING_API_KEY},
        auth_state::AuthState,
        events::{
            ActivityChanged, AuthStateChanged, CollectionCreateFailed, DownloadComplete,
            DownloadError, LibraryChanged, LogoutRequested, SettingsChanged, SignInSucceeded,
            StartupAuthBegun, StartupAuthFailed,
        },
        theme::LibriTheme,
        ui_prefs::UiPrefs,
    },
    services::{LibraryService, collections::CollectionsService},
};
/// Type-tag used to identify the startup-auth toast notification.
struct AuthPendingNotif;

/// Top-level GPUI view for the Libri library window.
pub struct LibraryRootView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    activity: Entity<ActivityController>,
    auth_state: Entity<AuthStateController>,
    catalog_view: Entity<CatalogView>,
    /// Resizable state for the three-column main layout (sidebar / catalog / detail).
    resize_state: Entity<ResizableState>,
    /// Restored or default sidebar panel initial width.
    sidebar_width: f32,
    /// Restored or default detail panel initial width.
    detail_width: f32,
    /// Default focus handle for the root div, ensuring menu-triggered actions
    /// always have a dispatch path even before any child element grabs focus.
    root_focus: FocusHandle,
    /// Focus handle for the settings overlay; grabbed when the panel opens so
    /// Escape key events route to the backdrop instead of the catalog.
    settings_focus: FocusHandle,
    /// Editable search input wired to the library controller's search query.
    search_input: Entity<InputState>,
    /// Draft name input for the "Create Collection" dialog.
    collection_name_input: Entity<InputState>,
}

impl LibraryRootView {
    /// Constructs the root view and wires up the controller subscriptions.
    ///
    /// If `startup_api_key` is `Some`, a background re-authentication is started immediately.
    /// The window always opens in the unauthenticated state and transitions once auth completes.
    pub fn new(
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
        service: Box<dyn LibraryService>,
        collections_service: Box<dyn CollectionsService>,
        startup_api_key: Option<String>,
    ) -> Self {
        let activity = cx.new(|_| ActivityController::new());
        let controller = cx.new(|cx| {
            LibraryController::new(service, collections_service, activity.clone(), cx)
        });
        let login_service = cx.global::<LoginServiceFactory>().0();
        let settings = cx.new(|cx| SettingsController::new(login_service, cx));

        let api_key_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Paste API key here\u{2026}"));
        let settings_for_input = settings.clone();
        cx.subscribe(
            &api_key_input,
            move |_this, input_entity, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = input_entity.read(cx).value().to_string();
                    settings_for_input.update(cx, |ctrl, cx| ctrl.set_api_key_draft(value, cx));
                }
            },
        )
        .detach();
        settings.update(cx, |ctrl, _cx| ctrl.set_api_key_input(api_key_input));

        let email_initial = settings.read(cx).email_draft().to_owned();
        let email_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Email (optional, for avatar)");
            if !email_initial.is_empty() {
                state = state.default_value(&email_initial);
            }
            state
        });
        let settings_for_email = settings.clone();
        cx.subscribe(
            &email_input,
            move |_this, input_entity, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = input_entity.read(cx).value().to_string();
                    settings_for_email.update(cx, |ctrl, cx| ctrl.set_email_draft(value, cx));
                }
            },
        )
        .detach();
        settings.update(cx, |ctrl, _cx| ctrl.set_email_input(email_input));

        let storage_path_placeholder = {
            use crate::data::storage::StorageConfig;
            StorageConfig::load()
                .root_path()
                .to_string_lossy()
                .into_owned()
        };
        let storage_path_input =
            cx.new(|cx| InputState::new(window, cx).default_value(&storage_path_placeholder));
        let settings_for_storage = settings.clone();
        cx.subscribe(
            &storage_path_input,
            move |_this, input_entity, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = input_entity.read(cx).value().to_string();
                    settings_for_storage
                        .update(cx, |ctrl, cx| ctrl.set_storage_path_draft(value, cx));
                }
            },
        )
        .detach();
        settings.update(cx, |ctrl, _cx| {
            ctrl.set_storage_path_input(storage_path_input)
        });

        let catalog_view =
            cx.new(|cx| CatalogView::new(window, cx, controller.clone(), settings.clone()));
        let auth_state = cx.new(|_| AuthStateController::new(AuthState::Unauthenticated));
        let root_focus = cx.focus_handle();
        root_focus.focus(window, cx);
        let settings_focus = cx.focus_handle();

        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search\u{2026}"));

        let controller_for_search = controller.clone();
        cx.subscribe(
            &search_input,
            move |_this, input_entity, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = input_entity.read(cx).value().to_string();
                    controller_for_search.update(cx, |ctrl, cx| {
                        ctrl.set_search_query(value, cx);
                    });
                }
            },
        )
        .detach();

        let collection_name_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Collection name\u{2026}"));

        cx.subscribe_in(
            &controller,
            window,
            |_this, _ctrl, event: &CollectionCreateFailed, window, cx| {
                window.push_notification(
                    Notification::new()
                        .message(event.message.clone())
                        .with_type(NotificationType::Error)
                        .autohide(false),
                    cx,
                );
            },
        )
        .detach();

        cx.subscribe(&controller, |_this, _ctrl, _event: &LibraryChanged, cx| {
            cx.notify();
        })
        .detach();

        cx.subscribe(&activity, |_this, _ctrl, _event: &ActivityChanged, cx| {
            cx.notify();
        })
        .detach();

        cx.subscribe_in(
            &activity,
            window,
            |_this, _ctrl, event: &DownloadComplete, window, cx| {
                let msg = format!("Downloaded: {}", event.title);
                window.push_notification(Notification::success(msg), cx);
            },
        )
        .detach();

        cx.subscribe_in(
            &activity,
            window,
            |_this, _ctrl, event: &DownloadError, window, cx| {
                let msg = format!("{}: {}", event.title, event.message);
                window.push_notification(
                    Notification::new()
                        .message(msg)
                        .with_type(NotificationType::Error)
                        .autohide(false),
                    cx,
                );
            },
        )
        .detach();

        cx.subscribe(&settings, |_this, _ctrl, _event: &SettingsChanged, cx| {
            cx.notify();
        })
        .detach();

        cx.subscribe(
            &auth_state,
            |_this, _ctrl, _event: &AuthStateChanged, cx| {
                cx.notify();
            },
        )
        .detach();

        // Handle logout: delete the API key and transition to unauthenticated.
        // Tokens are in-memory only and need no explicit deletion.
        let auth_state_for_logout = auth_state.clone();
        cx.subscribe(
            &settings,
            move |_this, _ctrl, _event: &LogoutRequested, cx| {
                let store = KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY);
                if let Err(e) = store.delete() {
                    tracing::warn!("credential delete (api-key): {e}");
                }
                auth_state_for_logout.update(cx, |ctrl, cx| {
                    ctrl.set_state(AuthState::Unauthenticated, cx)
                });
            },
        )
        .detach();

        // Handle sign-in: replace both services, mark authenticated, dismiss any auth toast.
        let auth_state_for_signin = auth_state.clone();
        let controller_for_signin = controller.clone();
        cx.subscribe_in(
            &settings,
            window,
            move |_this, _settings, event: &SignInSucceeded, window, cx| {
                let tokens = event.0.clone();
                let service =
                    cx.global::<ServiceFactory>().0.as_ref()(Some(tokens.clone()));
                let collections_service =
                    cx.global::<CollectionsServiceFactory>().0.as_ref()(Some(tokens));
                controller_for_signin.update(cx, |ctrl, cx| {
                    ctrl.replace_service(service, collections_service, cx);
                });
                auth_state_for_signin
                    .update(cx, |ctrl, cx| ctrl.set_state(AuthState::Authenticated, cx));
                window.remove_notification::<AuthPendingNotif>(cx);
            },
        )
        .detach();

        // Handle startup auth beginning: suppress the "Not signed in" banner and show a toast.
        let auth_state_for_begun = auth_state.clone();
        cx.subscribe_in(
            &settings,
            window,
            move |_this, _settings, _event: &StartupAuthBegun, window, cx| {
                auth_state_for_begun.update(cx, |ctrl, cx| ctrl.set_auth_pending(true, cx));
                window.push_notification(
                    Notification::new()
                        .message("Signing in to DriveThruRPG...")
                        .autohide(false)
                        .id::<AuthPendingNotif>(),
                    cx,
                );
            },
        )
        .detach();

        // Handle startup auth failure: clear pending state and dismiss the toast.
        let auth_state_for_failed = auth_state.clone();
        cx.subscribe_in(
            &settings,
            window,
            move |_this, _settings, _event: &StartupAuthFailed, window, cx| {
                auth_state_for_failed.update(cx, |ctrl, cx| ctrl.set_auth_pending(false, cx));
                window.remove_notification::<AuthPendingNotif>(cx);
            },
        )
        .detach();

        // Start background auth after all subscriptions are wired.
        if let Some(key) = startup_api_key {
            settings.update(cx, |ctrl, cx| ctrl.startup_auth(key, cx));
        }

        let ui_prefs = UiPrefs::load();
        let sidebar_width = ui_prefs.sidebar_width().unwrap_or(250.0);
        let detail_width = ui_prefs.detail_width().unwrap_or(320.0);
        let resize_state = cx.new(|_| ResizableState::default());

        cx.subscribe(&resize_state, move |_this, state, _event, cx| {
            let sizes = state.read(cx).sizes().clone();
            if sizes.len() >= 3 {
                let sidebar = sizes[0].as_f32();
                let detail = sizes[2].as_f32();
                UiPrefs::load().save_panel_widths(sidebar, detail);
            }
        })
        .detach();

        Self {
            controller,
            settings,
            activity,
            auth_state,
            catalog_view,
            resize_state,
            sidebar_width,
            detail_width,
            root_focus,
            settings_focus,
            search_input,
            collection_name_input,
        }
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
        let (
            filter,
            counts,
            publishers,
            collections,
            catalog_ids,
            total_count,
            total_mb,
            matched_count,
            sort,
            sort_direction,
            grouped,
            presentation,
            selected_item,
        ) = (
            snap.filter,
            snap.counts,
            snap.publishers,
            snap.collections,
            snap.catalog_ids,
            snap.total_count,
            snap.total_mb,
            snap.matched_count,
            snap.sort,
            snap.sort_direction,
            snap.grouped,
            snap.presentation,
            snap.selected_item,
        );

        let settings_snap = self.settings.read(cx).snapshot();
        let activity_snap = self.activity.read(cx).snapshot();
        let notices: Vec<_> = self
            .auth_state
            .read(cx)
            .active_notices()
            .into_iter()
            .cloned()
            .collect();

        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;

        let sidebar = render_sidebar(
            filter.clone(),
            counts,
            publishers,
            collections,
            catalog_ids,
            total_count,
            total_mb,
            lib_entity.clone(),
            activity_entity.clone(),
            activity_snap.in_progress_count,
            activity_snap.recent_count,
            activity_snap.recent_error_count,
            self.collection_name_input.clone(),
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
        let banner =
            render_notification_banner(notices, auth_entity, settings_entity.clone(), colors);
        let panel = render_detail_panel(
            selected_item.as_ref(),
            settings_snap.storage_root_path.clone(),
            lib_entity,
            colors,
        );

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
                    &settings_snap.file_openers,
                    settings_snap.auth,
                    settings_snap.storage_root_path,
                    settings_snap.storage_path_exists,
                    settings_entity,
                    &self.settings_focus,
                    colors,
                    settings_snap.api_key_input,
                    settings_snap.email_input,
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
        let mut sidebar_col = div().size_full().relative().child(sidebar);

        if activity_snap.panel_open {
            let overlay = render_activity_panel(&activity_snap, activity_entity, colors);
            sidebar_col = sidebar_col.child(overlay);
        }

        let settings_for_action = self.settings.clone();
        let sidebar_initial = self.sidebar_width;
        let detail_initial = self.detail_width;
        let has_detail = selected_item.is_some();

        div()
            .size_full()
            .bg(surface)
            .text_color(text_primary)
            .relative()
            .track_focus(&self.root_focus)
            .on_action(move |_: &ShowSettings, _, cx| {
                settings_for_action.update(cx, |ctrl, cx| ctrl.open(cx));
            })
            .child(
                h_resizable("main-layout")
                    .with_state(&self.resize_state)
                    .child(
                        resizable_panel()
                            .size(px(sidebar_initial))
                            .size_range(px(180.)..px(361.))
                            .child(sidebar_col),
                    )
                    .child(
                        resizable_panel()
                            .size_range(px(280.)..Pixels::MAX)
                            .child(main_content),
                    )
                    .when(has_detail, |group| {
                        group.child(
                            resizable_panel()
                                .size(px(detail_initial))
                                .size_range(px(240.)..px(481.))
                                .child(panel),
                        )
                    }),
            )
    }
}
