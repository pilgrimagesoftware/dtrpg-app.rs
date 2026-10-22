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

    /// Attaches the publisher rule input state created by the root view.
    pub fn set_publisher_rule_input(&mut self, input: Entity<InputState>) {
        self.publisher_rule_input = Some(input);
    }

    /// Attaches the collection and size rule inputs created by the root view.
    pub fn set_collection_rule_input(&mut self, input: Entity<InputState>) {
        self.collection_rule_input = Some(input);
    }

    pub fn set_size_rule_input(&mut self, input: Entity<InputState>) {
        self.size_rule_input = Some(input);
    }
}
