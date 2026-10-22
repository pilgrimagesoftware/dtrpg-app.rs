//! The placement rule editor of the Storage page: opening it to add a rule or
//! change one, following what is typed, and saving the rule.

use std::collections::HashMap;
use std::sync::Arc;

use super::*;
use crate::view_models::rule_editor::{condition_from_text, value_text};

impl SettingsController {
    /// The state of the placement rule editor.
    #[must_use]
    pub const fn rule_editor(&self) -> &RuleEditor {
        &self.rule_editor
    }

    /// Opens the editor to add a rule (`editing` is `None`) or to change the
    /// rule at that index, and returns the text the value field should show.
    pub fn open_rule_editor(&mut self, editing: Option<usize>,
                            collection_names: &HashMap<u64, Arc<str>>, cx: &mut Context<Self>)
                            -> String {
        let rule = editing.and_then(|index| self.storage.rules().get(index).cloned());
        let default_root_id = self.default_root_id();
        let editor = &mut self.rule_editor;
        editor.open = true;
        editor.editing = rule.as_ref().and(editing);
        editor.value = String::new();
        if let Some(rule) = rule {
            editor.kind = RuleEditorType::of(&rule.condition);
            editor.target_root_id = rule.target_root_id;
            editor.value = value_text(&rule.condition, collection_names);
        }
        else if self.storage.root(editor.target_root_id).is_none() {
            editor.target_root_id = default_root_id;
        }
        cx.emit(SettingsChanged);
        self.rule_editor.value.clone()
    }

    /// Closes the editor without saving.
    pub fn close_rule_editor(&mut self, cx: &mut Context<Self>) {
        self.rule_editor.open = false;
        self.rule_editor.editing = None;
        cx.emit(SettingsChanged);
    }

    /// Chooses the kind of rule being edited.
    pub fn set_rule_editor_type(&mut self, kind: RuleEditorType, cx: &mut Context<Self>) {
        self.rule_editor.kind = kind;
        cx.emit(SettingsChanged);
    }

    /// Chooses the location the rule sends downloads to.
    pub fn set_rule_target_root(&mut self, id: u32, cx: &mut Context<Self>) {
        if self.storage.root(id).is_some() {
            self.rule_editor.target_root_id = id;
            cx.emit(SettingsChanged);
        }
    }

    /// Keeps the text of the value field as it is typed.
    pub fn set_rule_value(&mut self, value: String, cx: &mut Context<Self>) {
        self.rule_editor.value = value;
        cx.emit(SettingsChanged);
    }

    /// Saves the rule the editor describes, as a new rule or in place of the
    /// one being changed, and closes the editor. Returns `false`, leaving
    /// the editor open, when the text describes no rule.
    pub fn save_rule(&mut self, collection_names: &HashMap<u64, Arc<str>>,
                     cx: &mut Context<Self>)
                     -> bool {
        let editor = &self.rule_editor;
        let Some(condition) = condition_from_text(editor.kind, &editor.value, collection_names)
        else {
            return false;
        };
        match editor.editing {
            Some(index) => self.storage
                               .replace_rule(index, condition, editor.target_root_id),
            None => self.storage.add_rule(condition, editor.target_root_id),
        }
        self.close_rule_editor(cx);
        true
    }
}
