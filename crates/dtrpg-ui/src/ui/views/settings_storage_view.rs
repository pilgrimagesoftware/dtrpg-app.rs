//! The Storage settings page: where downloads are kept, the rules that choose
//! between locations, and how storing files behaves.

use dtrpg_core::data::storage::placement::PlacementRule;
use dtrpg_core::data::storage::roots::StorageRoot;
use dtrpg_core::data::storage::validate_writable;
use dtrpg_core::platform::{Platform, PlatformIntegration, PlatformKind};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};
use gpui_kit::component::IconName;
use gpui_kit::component::Selectable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use rust_i18n::t;

use crate::controllers::settings::{RuleEditorType, SettingsController};
use crate::data::theme::ColorTokens;
use crate::view_models::storage_display::StorageDisplay;

#[path = "storage_behavior_view.rs"]
mod storage_behavior_view;
#[path = "storage_roots_view.rs"]
mod storage_roots_view;
#[path = "storage_rules_view.rs"]
mod storage_rules_view;

use storage_behavior_view::{BehaviorData, render_storage_behavior};
use storage_roots_view::render_storage_roots;
use storage_rules_view::{RuleInputs, RulesData, render_storage_rules};

/// Lower bound for the concurrency field: 0 would mean thumbnails and
/// downloads never start.
pub(crate) const MIN_CONCURRENT_DOWNLOADS: usize = 1;
/// Upper bound for the concurrency field. There is no bandwidth throttling
/// (see the change's design non-goals), so this caps how aggressively a
/// misconfigured value could saturate the connection.
pub(crate) const MAX_CONCURRENT_DOWNLOADS: usize = 5;

/// Everything the Storage page shows, gathered once per frame.
pub struct StoragePageData {
    /// The locations downloads can be kept in.
    pub roots:                    Vec<StorageRoot>,
    /// The placement rules, in the order they are tried.
    pub rules:                    Vec<PlacementRule>,
    /// Space, counts and names used to describe the locations and rules.
    pub display:                  StorageDisplay,
    /// `false` when the default location's folder is missing.
    pub root_path_exists:         bool,
    /// Whether a download that would not fit is refused.
    pub check_free_space:         bool,
    /// Whether collections are mirrored as folders of symlinks.
    pub create_collections:       bool,
    /// How many downloads run at once.
    pub max_concurrent_downloads: usize,
    /// The field that edits `max_concurrent_downloads`, once attached.
    pub max_concurrent_input:     Option<Entity<InputState>>,
    /// The "Add rule…" value field for a publisher rule.
    pub publisher_rule_input:     Option<Entity<InputState>>,
    /// The value field for a collection rule.
    pub collection_rule_input:    Option<Entity<InputState>>,
    /// The value field for a size rule.
    pub size_rule_input:          Option<Entity<InputState>>,
    /// The location a new rule will send downloads to.
    pub rule_target_root_id:      u32,
    /// Publisher names offered as suggestions for a new rule.
    pub publisher_names:          Vec<String>,
    /// The kind of rule the editor is adding.
    pub rule_editor_type:         RuleEditorType,
    /// Whether the "Add rule…" editor is showing.
    pub rule_editor_open:         bool,
}

/// Renders the Storage page: the Locations, Placement rules and Behavior
/// sections, and a warning above them when the default folder is missing.
pub fn render_storage_section(entity: Entity<SettingsController>, colors: &ColorTokens,
                              page: StoragePageData)
                              -> impl IntoElement + 'static + use<> {
    let missing_warning = (!page.root_path_exists).then(|| {
                                                      div().rounded(px(6.0))
             .px(px(10.0))
             .py(px(6.0))
             .bg(colors.warning_bg)
             .text_xs()
             .text_color(colors.warning_text)
             .child(format!("\u{26A0} {}", t!("settings.storage_missing")))
                                                  });
    let rules = RulesData { rules:           page.rules,
                            roots:           page.roots.clone(),
                            display:         page.display.clone(),
                            editor_open:     page.rule_editor_open,
                            editor_type:     page.rule_editor_type,
                            target_root_id:  page.rule_target_root_id,
                            publisher_names: page.publisher_names,
                            inputs:          RuleInputs { publisher:  page.publisher_rule_input,
                                                          collection: page.collection_rule_input,
                                                          size:       page.size_rule_input, }, };
    let behavior = BehaviorData { check_free_space:         page.check_free_space,
                                  create_collections:       page.create_collections,
                                  max_concurrent_downloads: page.max_concurrent_downloads,
                                  max_concurrent_input:     page.max_concurrent_input, };
    div().flex()
         .flex_col()
         .gap(px(24.0))
         .p(px(24.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(t!("settings.storage_title").to_string()))
         .children(missing_warning)
         .child(render_storage_roots(page.roots, &page.display, entity.clone(), colors))
         .child(render_storage_rules(rules, entity.clone(), colors))
         .child(render_storage_behavior(behavior, entity, colors))
}

/// The label for showing a folder in this platform's file manager.
fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    match Platform.kind() {
        PlatformKind::MacOs => t!("detail.show_in_finder"),
        PlatformKind::Windows => t!("detail.show_in_explorer"),
        PlatformKind::Linux => t!("detail.show_in_files"),
    }
}
