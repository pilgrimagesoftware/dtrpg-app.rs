//! The Storage settings page: where downloads are kept, the rules that choose
//! between locations, and how storing files behaves.

use dtrpg_core::data::storage::placement::PlacementRule;
use dtrpg_core::data::storage::roots::StorageRoot;
use dtrpg_core::platform::{Platform, PlatformIntegration, PlatformKind};
use gpui::{Entity, IntoElement, ParentElement, Styled, div, px};
use gpui_kit::component::input::InputState;
use rust_i18n::t;

use crate::controllers::settings::{RuleEditor, SettingsController};
use crate::data::theme::ColorTokens;
use crate::ui::views::storage::behavior::{BehaviorData, render_storage_behavior};
use crate::ui::views::storage::roots::render_storage_roots;
use crate::ui::views::storage::rules::{RulesData, render_storage_rules};
use crate::view_models::storage_display::StorageDisplay;

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
    /// Publisher names offered as suggestions for a rule's value.
    pub publisher_names:          Vec<String>,
    /// The placement rule editor: adding a rule or changing one.
    pub rule_editor:              RuleEditor,
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
                            editor:          page.rule_editor,
                            publisher_names: page.publisher_names, };
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
pub(crate) fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    match Platform.kind() {
        PlatformKind::MacOs => t!("detail.show_in_finder"),
        PlatformKind::Windows => t!("detail.show_in_explorer"),
        PlatformKind::Linux => t!("detail.show_in_files"),
    }
}
