//! Authentication state controller: owns `AuthState` and the active notice list.

use gpui::Context;

use crate::controllers::settings::SettingsTab;
use crate::data::auth_state::AuthState;
use crate::data::events::AuthStateChanged;
use crate::data::notification::{Notice, NoticeAction, NoticeKind};

/// Owns the application's authentication state and the derived notice list.
pub struct AuthStateController {
    state: AuthState,
    notices: Vec<Notice>,
}

impl AuthStateController {
    /// Creates a new controller.
    ///
    /// In debug builds the `DTRPG_AUTH_STATE_OVERRIDE` environment variable may be
    /// set to `"authenticated"`, `"expired"`, or `"unauthenticated"` to override the
    /// default initial state. In release builds the initial state is always
    /// `Unauthenticated`.
    pub fn new() -> Self {
        let state = initial_state();
        let notices = notices_for(state);
        Self { state, notices }
    }

    /// Returns the current authentication state.
    pub fn state(&self) -> AuthState {
        self.state
    }

    /// Returns all notices that have not been dismissed.
    pub fn active_notices(&self) -> Vec<&Notice> {
        self.notices.iter().filter(|n| !n.dismissed).collect()
    }

    /// Transitions to `state`, regenerates the notice list, and emits [`AuthStateChanged`].
    pub fn set_state(&mut self, state: AuthState, cx: &mut Context<Self>) {
        self.state = state;
        self.notices = notices_for(state);
        cx.emit(AuthStateChanged);
    }

    /// Marks the notice with the given `kind` as dismissed and emits [`AuthStateChanged`].
    ///
    /// No-op if no matching notice is found.
    pub fn dismiss_notice(&mut self, kind: NoticeKind, cx: &mut Context<Self>) {
        let before = self.notices.iter().filter(|n| !n.dismissed).count();
        for notice in self.notices.iter_mut() {
            if notice.kind == kind {
                notice.dismissed = true;
            }
        }
        let after = self.notices.iter().filter(|n| !n.dismissed).count();
        if after != before {
            cx.emit(AuthStateChanged);
        }
    }
}

impl Default for AuthStateController {
    fn default() -> Self {
        Self::new()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn initial_state() -> AuthState {
    #[cfg(debug_assertions)]
    {
        match std::env::var("DTRPG_AUTH_STATE_OVERRIDE")
            .as_deref()
            .unwrap_or("")
        {
            "authenticated" => return AuthState::Authenticated,
            "expired" => return AuthState::SessionExpired,
            _ => {}
        }
    }
    AuthState::Unauthenticated
}

fn notices_for(state: AuthState) -> Vec<Notice> {
    match state {
        AuthState::Authenticated => vec![],
        AuthState::Unauthenticated => vec![Notice {
            kind: NoticeKind::NotSignedIn,
            dismissed: false,
            action: NoticeAction::OpenSettings(SettingsTab::Account),
        }],
        AuthState::SessionExpired => vec![Notice {
            kind: NoticeKind::SessionExpired,
            dismissed: false,
            action: NoticeAction::OpenSettings(SettingsTab::Account),
        }],
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> AuthStateController {
        AuthStateController::new()
    }

    #[test]
    fn new_defaults_to_unauthenticated() {
        let ctrl = make();
        assert_eq!(ctrl.state(), AuthState::Unauthenticated);
    }

    #[test]
    fn unauthenticated_produces_one_notice() {
        let ctrl = make();
        assert_eq!(ctrl.active_notices().len(), 1);
        assert_eq!(ctrl.active_notices()[0].kind, NoticeKind::NotSignedIn);
    }

    #[test]
    fn authenticated_state_has_no_notices() {
        let mut ctrl = make();
        ctrl.state = AuthState::Authenticated;
        ctrl.notices = notices_for(AuthState::Authenticated);
        assert!(ctrl.active_notices().is_empty());
    }

    #[test]
    fn dismiss_notice_removes_only_target() {
        let mut ctrl = make();
        assert_eq!(ctrl.active_notices().len(), 1);
        for n in ctrl.notices.iter_mut() {
            if n.kind == NoticeKind::NotSignedIn {
                n.dismissed = true;
            }
        }
        assert!(ctrl.active_notices().is_empty());
    }

    #[test]
    fn expired_produces_session_expired_notice() {
        let notices = notices_for(AuthState::SessionExpired);
        assert_eq!(notices.len(), 1);
        assert_eq!(notices[0].kind, NoticeKind::SessionExpired);
    }
}
