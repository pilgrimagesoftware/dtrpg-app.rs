//! Attaches UI widget state entities (text inputs, font pickers) created by
//! the root view onto the controller, so later renders can read them back.

use super::*;

impl SettingsController {
    /// Attaches the email input state entity created by the root view.
    pub fn set_email_input(&mut self, input: Entity<InputState>) {
        self.email_input = Some(input);
    }

    /// Attaches the password input state entity created by the root view.
    pub fn set_password_input(&mut self, input: Entity<InputState>) {
        self.password_input = Some(input);
    }

    /// Attaches the storage path input state entity created by the root view.
    pub fn set_storage_path_input(&mut self, input: Entity<InputState>) {
        self.storage_path_input = Some(input);
    }

    /// Attaches the "Recently Updated window" number input state entity
    /// created by the root view.
    pub fn set_recently_updated_window_input(&mut self, input: Entity<InputState>) {
        self.recently_updated_window_input = Some(input);
    }

    /// Attaches the "Max concurrent downloads" number input state entity
    /// created by the root view.
    pub fn set_max_concurrent_downloads_input(&mut self, input: Entity<InputState>) {
        self.max_concurrent_downloads_input = Some(input);
    }

    /// Attaches the text scale number input state entity created by the root
    /// view.
    pub fn set_text_scale_input(&mut self, input: Entity<InputState>) {
        self.text_scale_input = Some(input);
    }

    /// Attaches the theme file editor state created by the root view.
    pub fn set_theme_editor_input(&mut self, input: Entity<EditorState>) {
        self.theme_editor_input = Some(input);
    }

    /// The theme file editor state, once the root view has attached it.
    #[must_use]
    pub fn theme_editor_input(&self) -> Option<Entity<EditorState>> {
        self.theme_editor_input.clone()
    }

    /// Sets, or with `None` clears, the message shown under the theme editor.
    pub fn set_theme_editor_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.theme_editor_error = error;
        cx.notify();
    }

    /// Attaches the placement rule editor's value field, created by the root
    /// view.
    pub fn set_rule_editor_input(&mut self, input: Entity<InputState>) {
        self.rule_editor.input = Some(input);
    }
}
