//! Root view for a popped-out inspector: a separate window showing the same
//! inspector as the item's tab in the main window.

use std::sync::Arc;

use dtrpg_core::data::ui_state::UiState;
use gpui::{
    AppContext as _, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, div,
};

use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::controllers::tabs::TabsController;
use crate::data::events::LibraryChanged;
use crate::data::theme::LibriTheme;
use crate::ui::app::remember_window;
use crate::ui::library::cover::{CoverCache, detail_cache_key};
use crate::ui::views::detail_panel_view::render_detail_tab_content;

/// Root view for a popped-out inspector window.
pub struct InspectorWindowView {
    item_id:    Arc<str>,
    controller: Entity<LibraryController>,
    settings:   Entity<SettingsController>,
    /// Holds this window's item list state. It has no pop-out control, since
    /// the inspector is already in its own window.
    tabs:       Entity<TabsController>,
}

impl InspectorWindowView {
    /// Shows the inspector for `item_id`, kept up to date as the library
    /// changes.
    pub fn new(item_id: Arc<str>, controller: Entity<LibraryController>,
               settings: Entity<SettingsController>, window: &mut Window, cx: &mut Context<Self>)
               -> Self {
        let tabs = cx.new(|_| TabsController::new_for_pop_out_window());
        cx.subscribe_in(&controller,
                        window,
                        |this, _controller, _event: &LibraryChanged, window, cx| {
                            this.close_if_item_left_library(window, cx);
                        })
          .detach();
        cx.observe(&controller, |_this, _controller, cx| cx.notify())
          .detach();
        cx.observe_window_bounds(window, |this, window, _cx| {
              remember_window(&this.item_id, window);
          })
          .detach();
        Self { item_id,
               controller,
               settings,
               tabs }
    }

    /// Closes the window, and forgets it, once its item is no longer in the
    /// library. Waits for the catalog fetch to finish first: until then the
    /// item may simply not have arrived yet.
    fn close_if_item_left_library(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let controller = self.controller.read(cx);
        let item_has_left =
            !controller.is_catalog_loading() && controller.item_by_id(&self.item_id).is_none();
        if item_has_left {
            UiState::load().remove_inspector_window(&self.item_id);
            window.remove_window();
        }
        else {
            cx.notify();
        }
    }
}

impl Render for InspectorWindowView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;
        window.set_rem_size(theme.fonts.ui_text_size);

        let window_content = div().id("inspector-window")
                                  .size_full()
                                  .flex()
                                  .flex_col()
                                  .bg(colors.surface)
                                  .text_color(colors.text_primary);
        // Between the item leaving the library and the window closing, there
        // is nothing to show.
        let Some(item) = self.controller.read(cx).item_by_id(&self.item_id)
        else {
            return window_content;
        };

        let cache = cx.global::<CoverCache>();
        let cover_image = cache.get(&detail_cache_key(&item.id))
                               .or_else(|| cache.get(&item.id));
        let storage_root_path = self.settings.read(cx).snapshot().storage_root_path;
        window_content.child(render_detail_tab_content(&item,
                                                       storage_root_path,
                                                       self.controller.clone(),
                                                       &self.tabs,
                                                       colors,
                                                       cover_image,
                                                       window,
                                                       cx))
    }
}
