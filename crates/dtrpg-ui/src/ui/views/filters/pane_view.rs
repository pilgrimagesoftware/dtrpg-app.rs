//! The filters column and the catalog beside it, in a view of their own.
//!
//! The divider between them keeps its state in the view that draws it, and
//! every move of the divider redraws that view. Drawn from the library window's
//! root view, a drag rebuilt the whole window on each move (the sidebar, the
//! toolbar, the status bar and the column's rows). Here a drag redraws only
//! this view, from the facets it already holds: they are worked out when the
//! library changes, not while the divider moves.

use dtrpg_core::library::Facets;
use dtrpg_core::util::search::SearchQuery;
use gpui::prelude::*;
use gpui::{Context, Entity, Window, div, px};
use gpui_kit::component::input::InputState;
use gpui_kit::component::resizable::{h_resizable, resizable_panel};

use crate::controllers::library::LibraryController;
use crate::data::events::LibraryChanged;
use crate::data::theme::LibriTheme;
use crate::ui::views::catalog_view::CatalogView;
use crate::ui::views::filters_column::{render_filters_column, without_hidden_values};

/// The narrowest the filters column can be dragged to.
const FILTERS_MIN_WIDTH: f32 = 160.0;

/// The widest the filters column can be dragged to.
const FILTERS_MAX_WIDTH: f32 = 420.0;

/// The width the filters column starts at.
const FILTERS_DEFAULT_WIDTH: f32 = 220.0;

/// The filters column, when it is shown, beside the catalog.
pub struct FiltersPaneView {
    controller:   Entity<LibraryController>,
    search_input: Entity<InputState>,
    catalog_view: Entity<CatalogView>,
    /// Whether the column is shown.
    open:         bool,
    /// The facets the column lists, with the hidden values left out.
    facets:       Facets,
    /// The search the column shows as checked boxes.
    query:        SearchQuery,
    /// The text `query` was parsed from, to tell when the search changed.
    query_text:   String,
}

impl FiltersPaneView {
    /// A pane showing `catalog_view`, with the column beside it once opened.
    pub fn new(cx: &mut Context<Self>, controller: Entity<LibraryController>,
               search_input: Entity<InputState>, catalog_view: Entity<CatalogView>)
               -> Self {
        cx.subscribe(&controller,
                     |this, _controller, _event: &LibraryChanged, cx| {
                         this.refresh(cx);
                     })
          .detach();
        let mut pane = Self { controller,
                              search_input,
                              catalog_view,
                              open: false,
                              facets: Facets::default(),
                              query: SearchQuery::default(),
                              query_text: String::new() };
        pane.refresh(cx);
        pane
    }

    /// Shows or hides the column.
    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.open != open {
            self.open = open;
            cx.notify();
        }
    }

    /// Reads the facets and the search from the controller, and redraws if
    /// either changed. The library changes often during a load; most of those
    /// leave the column as it was.
    fn refresh(&mut self, cx: &mut Context<Self>) {
        let controller = self.controller.read(cx);
        let facets = without_hidden_values(&controller.visible_facets(),
                                           controller.filter_value_choices());
        let facets_changed = facets != self.facets;
        let query_changed = controller.search_query() != self.query_text;
        if query_changed {
            self.query_text = controller.search_query().to_owned();
            self.query = SearchQuery::parse(&self.query_text);
        }
        if facets_changed {
            self.facets = facets;
        }
        if facets_changed || query_changed {
            cx.notify();
        }
    }
}

impl Render for FiltersPaneView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.global::<LibriTheme>().colors.clone();
        let column = self.open.then(|| {
                                  render_filters_column(&self.facets,
                                                        &self.query,
                                                        &self.search_input,
                                                        &self.controller,
                                                        &colors)
                              });
        div().flex_1()
             .min_h_0()
             .flex()
             .child(h_resizable("filters-layout")
                .child(resizable_panel().visible(self.open)
                                        .size(px(FILTERS_DEFAULT_WIDTH))
                                        .size_range(px(FILTERS_MIN_WIDTH)..px(FILTERS_MAX_WIDTH))
                                        .flex_none()
                                        .children(column))
                .child(resizable_panel().child(div().flex_1()
                                                     .min_w_0()
                                                     .min_h_0()
                                                     .flex()
                                                     .flex_col()
                                                     .child(self.catalog_view.clone()))))
    }
}
