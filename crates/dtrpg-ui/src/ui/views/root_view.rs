//! Root view: composes sidebar, toolbar, catalog, and detail panel.

use crate::ui::actions::{
    AddCollection, ReloadCatalog, ShowActivity, ShowAlertHistory, ShowSettings,
};
use crate::ui::app::{CollectionsServiceFactory, LoginServiceFactory, ServiceFactory};
use gpui::{
    AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, Styled, div, px,
};
use gpui_component::Root;
use gpui_component::WindowExt as _;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::dialog::{DialogAction, DialogClose, DialogFooter, DialogHeader, DialogTitle};
use gpui_component::input::{Input, InputEvent, InputState};
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
        auth_state::AuthState,
        constants::{KEYRING_API_KEY, KEYRING_SERVICE},
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
use rust_i18n::t;

/// Type-tag used to identify the startup-auth toast notification.
struct AuthPendingNotif;

/// Top-level GPUI view for the Libri library window.
pub struct LibraryRootView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    activity: Entity<ActivityController>,
    auth_state: Entity<AuthStateController>,
    catalog_view: Entity<CatalogView>,
    /// Resizable state for the two-column main layout (sidebar / catalog).
    resize_state: Entity<ResizableState>,
    /// Restored or default sidebar panel initial width.
    sidebar_width: f32,
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
    /// Inline search input for the sidebar's Publishers section.
    publisher_search_input: Entity<InputState>,
    /// Inline search input for the sidebar's Collections section.
    collection_search_input: Entity<InputState>,
    /// Draft extension input for the in-progress "add file opener" row in the
    /// File Openers settings list (inline, not a modal — see the subscription
    /// wired up in `new()` for the Enter/Blur commit behavior).
    file_opener_extension_input: Entity<InputState>,
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
        let controller =
            cx.new(|cx| LibraryController::new(service, collections_service, activity.clone(), cx));
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
        let file_opener_extension_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Extension (e.g. pdf)"));

        let publisher_search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search publishers\u{2026}"));
        let controller_for_publisher_search = controller.clone();
        cx.subscribe(
            &publisher_search_input,
            move |_this, input_entity, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = input_entity.read(cx).value().to_string();
                    controller_for_publisher_search.update(cx, |ctrl, cx| {
                        ctrl.set_publisher_search_query(value, cx);
                    });
                }
            },
        )
        .detach();

        let collection_search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search collections\u{2026}"));
        let controller_for_collection_search = controller.clone();
        cx.subscribe(
            &collection_search_input,
            move |_this, input_entity, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    let value = input_entity.read(cx).value().to_string();
                    controller_for_collection_search.update(cx, |ctrl, cx| {
                        ctrl.set_collection_search_query(value, cx);
                    });
                }
            },
        )
        .detach();

        // Commit the pending "add file opener" row on Enter; on blur, commit if the
        // user typed something, otherwise discard the pending row (matches clicking
        // elsewhere in most inline-add UIs — no separate confirm step needed).
        let settings_for_ext = settings.clone();
        cx.subscribe(
            &file_opener_extension_input,
            move |_this, input_entity, event: &InputEvent, cx| match event {
                InputEvent::PressEnter { .. } => {
                    let value = input_entity.read(cx).value().to_string();
                    settings_for_ext
                        .update(cx, |ctrl, cx| ctrl.commit_pending_file_opener(&value, cx));
                }
                InputEvent::Blur => {
                    let value = input_entity.read(cx).value().to_string();
                    settings_for_ext.update(cx, |ctrl, cx| {
                        if value.trim().is_empty() {
                            ctrl.cancel_pending_file_opener(cx);
                        } else {
                            ctrl.commit_pending_file_opener(&value, cx);
                        }
                    });
                }
                _ => {}
            },
        )
        .detach();

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
                let service = cx.global::<ServiceFactory>().0.as_ref()(Some(tokens.clone()));
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
        let resize_state = cx.new(|_| ResizableState::default());

        cx.subscribe(&resize_state, move |_this, state, _event, cx| {
            let sizes = state.read(cx).sizes().clone();
            if sizes.len() >= 2 {
                let sidebar = sizes[0].as_f32();
                UiPrefs::load().save_sidebar_width(sidebar);
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
            root_focus,
            settings_focus,
            search_input,
            collection_name_input,
            file_opener_extension_input,
            publisher_search_input,
            collection_search_input,
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
        // `Root` (gpui-component) tracks open dialogs/sheets/notifications as state but
        // does not render them itself — the top-level app view must compose these layers
        // explicitly, or `window.open_dialog()` / `open_alert_dialog()` / `push_notification()`
        // calls silently have no visible effect. See gpui-component's `StoryRoot` example.
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

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

        let publisher_search = crate::ui::views::sidebar_view::SidebarSectionSearch {
            open: snap.publisher_search_open,
            query: snap.publisher_search_query,
            input: self.publisher_search_input.clone(),
        };
        let collection_search = crate::ui::views::sidebar_view::SidebarSectionSearch {
            open: snap.collection_search_open,
            query: snap.collection_search_query,
            input: self.collection_search_input.clone(),
        };
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
            publisher_search,
            collection_search,
        );
        let toolbar = render_toolbar(
            &filter,
            snap.filter_count,
            matched_count,
            total_count,
            &snap.search_query,
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
        let cover_image = selected_item.as_ref().and_then(|item| {
            cx.global::<crate::ui::library::cover::CoverCache>()
                .get(&item.id)
        });
        let panel = render_detail_panel(
            selected_item.as_ref(),
            settings_snap.storage_root_path.clone(),
            lib_entity,
            colors,
            cover_image,
            snap.detail_panel_width,
        );

        let surface = colors.surface;
        let text_primary = colors.text_primary;

        // Settings overlay and detail panel are rendered inside the main content area
        // so the sidebar remains visible behind them.
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
                .child(self.catalog_view.clone())
                .child(panel);

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
                    self.file_opener_extension_input.clone(),
                    settings_snap.pending_file_opener,
                );
                content = content.child(overlay);
            }

            content
        };

        let sidebar_col = div().size_full().relative().child(sidebar);

        // The activity panel overlay is rendered at the root level (below), not
        // nested inside `sidebar_col`. Both `sidebar_col` and `main_content` are
        // siblings inside the resizable layout, painted in DOM order — an
        // absolute-positioned child of `sidebar_col` that visually extends past
        // the sidebar's width would be painted *under* `main_content`'s opaque
        // background and appear clipped by the catalog area. Rendering it as a
        // root-level sibling after the resizable layout guarantees it paints on
        // top of everything.
        let activity_overlay = activity_snap
            .panel_open
            .then(|| render_activity_panel(&activity_snap, activity_entity, colors));

        let settings_for_action = self.settings.clone();
        let controller_for_reload = self.controller.clone();
        let controller_for_add = self.controller.clone();
        let collection_input_for_add = self.collection_name_input.clone();
        let activity_for_show = self.activity.clone();
        let sidebar_initial = self.sidebar_width;

        div()
            .size_full()
            .bg(surface)
            .text_color(text_primary)
            .relative()
            .track_focus(&self.root_focus)
            .on_action(move |_: &ShowSettings, _, cx| {
                settings_for_action.update(cx, |ctrl, cx| ctrl.open(cx));
            })
            .on_action(move |_: &ReloadCatalog, _, cx| {
                controller_for_reload.update(cx, |ctrl, cx| ctrl.reload_catalog(cx));
            })
            .on_action(move |_: &AddCollection, window, cx| {
                let ctrl = controller_for_add.clone();
                let input = collection_input_for_add.clone();
                window.open_dialog(cx, move |dialog, _window, _cx| {
                    let ctrl = ctrl.clone();
                    let input = input.clone();
                    dialog
                        .close_button(false)
                        .overlay_closable(true)
                        .w(px(320.))
                        // Visible Cancel/Create buttons are rendered via `.footer(...)`
                        // below (wrapped in `DialogClose`/`DialogAction`, which dispatch
                        // the same `CancelDialog`/`ConfirmDialog` actions Escape/Enter
                        // use), so the callbacks registered here run regardless of
                        // whether the user clicks a button or uses the keyboard.
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
                            let input = input.clone();
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
                        })
                        .footer(
                            DialogFooter::new()
                                .px_4()
                                .pb_4()
                                .child(
                                    DialogClose::new().child(
                                        Button::new("cancel-collection")
                                            .label(t!("collections.add_dialog_cancel")),
                                    ),
                                )
                                .child(
                                    DialogAction::new().child(
                                        Button::new("confirm-collection")
                                            .primary()
                                            .label(t!("collections.add_dialog_confirm")),
                                    ),
                                ),
                        )
                });
            })
            .on_action(move |_: &ShowActivity, _, cx| {
                activity_for_show.update(cx, |a, cx| a.toggle_panel(cx));
            })
            .on_action(|_: &ShowAlertHistory, _, _cx| {
                tracing::info!("ShowAlertHistory action triggered");
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
                    ),
            )
            .children(activity_overlay)
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}
