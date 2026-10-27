//! The Agent & MCP page's Summaries settings: how summaries are made and how
//! much of the month's allowance is used.

use chrono::Local;
use dtrpg_core::credentials::{anthropic_key_status, save_anthropic_api_key};
use dtrpg_core::data::index::LibraryIndex;
use dtrpg_core::data::summary_settings::SummaryProviderKind;
use dtrpg_core::util::datetime::start_of_month_secs;
use gpui_kit::component::input::InputState;

use super::*;
use crate::view_models::summary_display::SummaryPage;

impl SettingsController {
    /// Saves which provider writes new summaries.
    pub fn set_summary_provider(&mut self, provider: SummaryProviderKind, cx: &mut Context<Self>) {
        self.summaries.save_provider(provider);
        cx.notify();
    }

    /// Saves the model new summaries are written with.
    pub fn set_summary_model(&mut self, model: String, cx: &mut Context<Self>) {
        self.summaries.save_model(model);
        cx.notify();
    }

    /// Saves the monthly token ceiling.
    pub fn set_summary_ceiling(&mut self, ceiling: u64, cx: &mut Context<Self>) {
        self.summaries.save_monthly_token_ceiling(ceiling);
        cx.notify();
    }

    /// Attaches the field the Anthropic API key is typed into, created by the
    /// root view.
    pub fn set_anthropic_key_input(&mut self, input: Entity<InputState>) {
        self.summary_page.key_input = Some(input);
    }

    /// Saves `key` as the Anthropic API key in the keychain, and returns
    /// whether it was saved, so the field can be cleared. An empty key is
    /// ignored: leaving the field alone must not remove a saved key. The key
    /// is never logged.
    pub fn save_anthropic_key(&mut self, key: &str, cx: &mut Context<Self>) -> bool {
        let key = key.trim();
        if key.is_empty() {
            return false;
        }
        let saved = match save_anthropic_api_key(key) {
            Ok(()) => true,
            Err(error) => {
                tracing::error!(%error, "could not save the Anthropic API key");
                false
            }
        };
        self.summary_page.key = anthropic_key_status();
        cx.notify();
        saved
    }

    /// Saves whether a summary is written when a download finishes.
    pub fn set_summary_auto(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.summaries.save_auto_summarize_on_download(enabled);
        cx.notify();
    }
}

/// What the Summaries card reads besides the settings: the month's token use
/// and where the API key can come from. Reads the index and the keychain, so it
/// runs when the page opens, never while drawing.
pub(super) fn read_summary_page(key_input: Option<Entity<InputState>>) -> SummaryPage {
    let tokens_used = LibraryIndex::open_and_migrate(&dtrpg_core::data::paths::index_dir())
        .and_then(|index| index.tokens_used_since(start_of_month_secs(Local::now())))
        .unwrap_or_default();
    SummaryPage { tokens_used,
                  key: anthropic_key_status(),
                  key_input }
}
