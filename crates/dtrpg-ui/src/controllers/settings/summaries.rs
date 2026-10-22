//! The Agent & MCP page's Summaries settings: how summaries are made and how
//! much of the month's allowance is used.

use chrono::Local;
use dtrpg_core::data::index::LibraryIndex;
use dtrpg_core::data::paths::cache_dir;
use dtrpg_core::data::summary_settings::SummaryProviderKind;
use dtrpg_core::util::datetime::start_of_month_secs;

use super::*;

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

    /// Saves whether a summary is written when a download finishes.
    pub fn set_summary_auto(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.summaries.save_auto_summarize_on_download(enabled);
        cx.notify();
    }
}

/// The tokens the summaries made since the start of this month have used, or 0
/// if the index cannot be read. Reads the index, so it runs when the page
/// opens, never while drawing.
pub(super) fn tokens_used_this_month() -> u64 {
    LibraryIndex::open_and_migrate(&cache_dir()).and_then(|index| {
                                                    index.tokens_used_since(start_of_month_secs(Local::now()))
                                                })
                                                .unwrap_or_default()
}
