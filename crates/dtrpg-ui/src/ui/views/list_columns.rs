//! The columns of the catalog's list layout: which are shown, how wide the
//! user has dragged them, how each sorts, and the menu that shows and hides
//! them.

use std::collections::HashMap;

use dtrpg_core::data::enums::ListColumn;
use dtrpg_core::util::sort::SortMethod;
use gpui::{Entity, Pixels};
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::table::Column;
use rust_i18n::t;

use crate::controllers::library::LibraryController;

/// The share of the table's width a fitted column may take at most.
const FIT_WIDTH_SHARE: f32 = 0.6;

/// How far, in pixels, a reported width may be from a column's own before the
/// user is taken to have dragged it.
const DRAG_TOLERANCE: f32 = 0.5;

/// The width of a fitted column that is not the content's: the cell's padding.
pub(super) const FIT_CELL_PADDING: f32 = 24.0;

/// How many of a column's longest texts are measured to find its widest. The
/// widest text on screen is almost always among the longest by characters, so
/// measuring a few spares shaping every item's text.
pub(super) const FIT_CANDIDATES: usize = 8;

/// The width of the widest of the `count` longest `texts`, as `measure` gives
/// it.
pub(super) fn widest_of_longest<'a>(texts: impl Iterator<Item = &'a str>, count: usize,
                                    measure: impl Fn(&str) -> Pixels)
                                    -> Pixels {
    let mut texts: Vec<&str> = texts.collect();
    texts.sort_unstable_by_key(|text| std::cmp::Reverse(text.chars().count()));
    texts.into_iter()
         .take(count)
         .map(measure)
         .fold(Pixels::ZERO, Pixels::max)
}

/// `ideal` limited to [`FIT_WIDTH_SHARE`] of `table_width`, when the table's
/// width is known.
pub(super) fn capped_width(ideal: Pixels, table_width: Option<Pixels>) -> Pixels {
    table_width.map_or(ideal, |width| ideal.min(width * FIT_WIDTH_SHARE))
}

/// The text in a column's heading.
fn heading(column: ListColumn) -> String {
    match column {
        ListColumn::Title => t!("catalog.col_title").to_string(),
        ListColumn::Publisher => t!("catalog.col_publisher").to_string(),
        ListColumn::System => t!("catalog.col_system").to_string(),
        ListColumn::Pages => t!("catalog.col_pages").to_string(),
        ListColumn::Size => t!("catalog.col_size").to_string(),
        ListColumn::Added => t!("catalog.col_added").to_string(),
        ListColumn::Tags => t!("catalog.col_tags").to_string(),
        ListColumn::Status | ListColumn::Open => String::new(),
    }
}

/// The table's definition of `column`, at its usual width.
fn table_column(column: ListColumn) -> Column {
    let defined = Column::new(column.key(), heading(column));
    match column {
        ListColumn::Title => defined.width(300.).min_width(150.).resizable(true),
        ListColumn::Publisher => defined.width(130.).resizable(true),
        ListColumn::System => defined.width(110.).resizable(true),
        ListColumn::Pages | ListColumn::Size => defined.width(60.).resizable(true),
        ListColumn::Added => defined.width(80.).resizable(true),
        ListColumn::Tags => defined.width(160.).resizable(true),
        ListColumn::Status => defined.width(24.).resizable(false).selectable(false),
        ListColumn::Open => defined.width(28.).resizable(false).selectable(false),
    }
}

/// The sort a click on `column`'s heading asks for, if it can sort.
pub(super) fn sort_method_for(column: ListColumn) -> Option<SortMethod> {
    match column {
        ListColumn::Title => Some(SortMethod::Title),
        ListColumn::Publisher
        | ListColumn::System
        | ListColumn::Pages
        | ListColumn::Size
        | ListColumn::Added => Some(SortMethod::Custom { col_key: column.key(), }),
        ListColumn::Tags | ListColumn::Status | ListColumn::Open => None,
    }
}

/// The column the list is sorted by, if it has one.
pub(super) fn column_sorted_by(sort: SortMethod) -> Option<ListColumn> {
    match sort {
        SortMethod::Title => Some(ListColumn::Title),
        SortMethod::Publisher => Some(ListColumn::Publisher),
        SortMethod::PageCount => Some(ListColumn::Pages),
        SortMethod::DateAdded => Some(ListColumn::Added),
        SortMethod::Custom { col_key } => ListColumn::from_key(col_key),
    }
}

/// The columns the table shows, and the widths the user has dragged them to.
///
/// Both list delegates hold one, so the headings and the cells of the grouped
/// and ungrouped lists always line up.
pub(super) struct ListColumnSet {
    shown:       Vec<ListColumn>,
    columns:     Vec<Column>,
    /// Index for index with `shown`: `None` means the usual width.
    user_widths: Vec<Option<Pixels>>,
    /// Index for index with `shown`: the width that fits the column's widest
    /// item, before it is limited to a share of the table. `None` means no fit
    /// was found, and the usual width stands.
    fitted:      Vec<Option<Pixels>>,
    /// The width the table was last drawn at, which limits the fitted widths.
    table_width: Option<Pixels>,
}

impl ListColumnSet {
    /// The columns shown when `chosen` are the optional ones chosen.
    pub(super) fn new(chosen: &[ListColumn]) -> Self {
        let shown = ListColumn::shown(chosen);
        Self { columns: shown.iter().map(|column| table_column(*column)).collect(),
               user_widths: vec![None; shown.len()],
               fitted: vec![None; shown.len()],
               table_width: None,
               shown }
    }

    /// How many columns are shown.
    pub(super) fn len(&self) -> usize {
        self.shown.len()
    }

    /// The column at `index`.
    pub(super) fn kind(&self, index: usize) -> Option<ListColumn> {
        self.shown.get(index).copied()
    }

    /// Where `column` is, if it is shown.
    pub(super) fn position_of(&self, column: ListColumn) -> Option<usize> {
        self.shown.iter().position(|shown| *shown == column)
    }

    /// The columns shown, in order.
    pub(super) fn shown(&self) -> &[ListColumn] {
        &self.shown
    }

    /// The table's definition of the column at `index`: at the width the user
    /// has dragged it to if they have, else at the width that fits its widest
    /// item (never more than a share of the table), else at its usual width.
    pub(super) fn column(&self, index: usize) -> Column {
        let column = self.columns[index].clone();
        let width = self.user_widths.get(index).copied().flatten().or_else(|| {
                                                                      self.fitted
                                .get(index)
                                .copied()
                                .flatten()
                                .map(|ideal| capped_width(ideal, self.table_width))
                                                                  });
        match width {
            Some(width) => column.width(width.as_f32()),
            None => column,
        }
    }

    /// Records the widths that fit each column's widest item. Widths for a
    /// different set of columns are ignored.
    pub(super) fn set_fitted(&mut self, widths: Vec<Option<Pixels>>) {
        if widths.len() == self.shown.len() {
            self.fitted = widths;
        }
    }

    /// Records the width the table was drawn at. Returns whether it changed,
    /// so a caller redraws only then.
    pub(super) fn set_table_width(&mut self, width: Pixels) -> bool {
        let changed = self.table_width != Some(width);
        self.table_width = Some(width);
        changed
    }

    /// Records the widths the table reports after the user drags a column.
    /// Only a column whose width differs from the one it had is the user's
    /// now; the others keep fitting their items. Widths for a different set of
    /// columns, as can arrive while the columns are changing, are ignored.
    pub(super) fn apply_widths(&mut self, widths: &[Pixels]) {
        if widths.len() != self.user_widths.len() {
            return;
        }
        for (index, width) in widths.iter().enumerate() {
            let current = self.column(index).width;
            if (width.as_f32() - current.as_f32()).abs() > DRAG_TOLERANCE {
                self.user_widths[index] = Some(*width);
            }
        }
    }

    /// Shows the columns for `chosen`, keeping the width of each column that
    /// stays. Returns whether anything changed.
    pub(super) fn set_chosen(&mut self, chosen: &[ListColumn]) -> bool {
        let shown = ListColumn::shown(chosen);
        if shown == self.shown {
            return false;
        }
        let kept: HashMap<ListColumn, Pixels> =
            self.shown
                .iter()
                .zip(&self.user_widths)
                .filter_map(|(column, width)| width.map(|width| (*column, width)))
                .collect();
        self.columns = shown.iter().map(|column| table_column(*column)).collect();
        self.user_widths = shown.iter()
                                .map(|column| kept.get(column).copied())
                                .collect();
        // The fit is for the old set of columns; it is found again for the new.
        self.fitted = vec![None; shown.len()];
        self.shown = shown;
        true
    }
}

/// Adds to `menu` an item for each column the user can show or hide, checked
/// when `chosen` has it, that shows or hides it when clicked.
pub(super) fn column_chooser_menu(menu: PopupMenu, chosen: &[ListColumn],
                                  controller: &Entity<LibraryController>)
                                  -> PopupMenu {
    let mut menu = menu;
    for column in ListColumn::CHOOSABLE {
        let controller = controller.clone();
        menu = menu.item(PopupMenuItem::new(heading(column)).checked(chosen.contains(&column))
                                                            .on_click(move |_, _, cx| {
                                                                controller.update(cx, |ctrl, cx| {
                                                                    ctrl.toggle_list_column(column,
                                                                                            cx);
                                                                });
                                                            }));
    }
    menu
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use gpui::px;

    use super::*;

    #[test]
    fn the_columns_follow_the_chosen_ones_with_status_and_title_first_and_open_last() {
        let columns = ListColumnSet::new(&[ListColumn::Tags, ListColumn::Size]);

        assert_eq!(columns.len(), 5);
        assert_eq!(columns.kind(0), Some(ListColumn::Status));
        assert_eq!(columns.kind(1), Some(ListColumn::Title));
        assert_eq!(columns.position_of(ListColumn::Size), Some(2));
        assert_eq!(columns.position_of(ListColumn::Tags), Some(3));
        assert_eq!(columns.kind(4), Some(ListColumn::Open));
        assert_eq!(columns.position_of(ListColumn::Pages), None);
    }

    #[test]
    fn a_dragged_width_stays_with_its_column_when_others_come_and_go() {
        let mut columns = ListColumnSet::new(&[ListColumn::Publisher, ListColumn::Size]);
        // Status, Title, Publisher, Size, Open.
        columns.apply_widths(&[px(24.), px(300.), px(200.), px(60.), px(28.)]);

        let changed = columns.set_chosen(&[ListColumn::Size, ListColumn::Tags]);

        assert!(changed);
        // Status, Title, Size, Tags, Open: Publisher's width is gone, Size's is
        // not.
        let size = columns.position_of(ListColumn::Size).unwrap();
        let tags = columns.position_of(ListColumn::Tags).unwrap();
        assert_eq!(columns.column(size).width, px(60.));
        assert_eq!(columns.column(tags).width, px(160.));
    }

    #[test]
    fn choosing_the_same_columns_again_changes_nothing() {
        let mut columns = ListColumnSet::new(&ListColumn::DEFAULT_CHOSEN);

        assert!(!columns.set_chosen(&ListColumn::DEFAULT_CHOSEN));
    }

    #[test]
    fn widths_for_a_different_number_of_columns_are_ignored() {
        let mut columns = ListColumnSet::new(&[]);

        columns.apply_widths(&[px(500.)]);

        let title = columns.position_of(ListColumn::Title).unwrap();
        assert_eq!(columns.column(title).width, px(300.));
    }

    #[test]
    fn a_heading_sorts_by_its_column_and_the_sort_names_its_heading() {
        for column in ListColumn::shown(&ListColumn::CHOOSABLE) {
            match sort_method_for(column) {
                Some(method) => {
                    assert!(column.is_sortable());
                    assert_eq!(column_sorted_by(method), Some(column));
                }
                None => assert!(!column.is_sortable()),
            }
        }
    }

    #[test]
    fn the_older_named_sorts_light_up_their_columns() {
        assert_eq!(column_sorted_by(SortMethod::Publisher),
                   Some(ListColumn::Publisher));
        assert_eq!(column_sorted_by(SortMethod::PageCount),
                   Some(ListColumn::Pages));
        assert_eq!(column_sorted_by(SortMethod::DateAdded),
                   Some(ListColumn::Added));
        assert_eq!(column_sorted_by(SortMethod::Custom { col_key: "nonsense", }),
                   None);
    }

    #[test]
    fn the_widest_of_the_longest_texts_sets_the_width() {
        let measure = |text: &str| px(text.chars().count() as f32 * 7.0);

        let widest = widest_of_longest(["ab", "abcdef", "abc", "abcd"].into_iter(), 2, measure);

        assert_eq!(widest, px(42.));
    }

    #[test]
    fn no_texts_need_no_width() {
        assert_eq!(widest_of_longest(std::iter::empty(), 4, |_| px(10.)),
                   px(0.));
    }

    #[test]
    fn a_fitted_width_is_limited_to_sixty_percent_of_the_table() {
        assert_eq!(capped_width(px(700.), Some(px(1000.))), px(600.));
        assert_eq!(capped_width(px(200.), Some(px(1000.))), px(200.));
        assert_eq!(capped_width(px(700.), None), px(700.));
    }

    #[test]
    fn a_column_uses_its_fitted_width_until_the_user_drags_it() {
        let mut columns = ListColumnSet::new(&[ListColumn::Publisher]);
        let publisher = columns.position_of(ListColumn::Publisher).unwrap();
        let mut fitted = vec![None; columns.len()];
        fitted[publisher] = Some(px(210.));
        columns.set_fitted(fitted);
        assert_eq!(columns.column(publisher).width, px(210.));

        let mut dragged = vec![px(24.); columns.len()];
        dragged[publisher] = px(150.);
        columns.apply_widths(&dragged);

        assert_eq!(columns.column(publisher).width, px(150.));
    }

    #[test]
    fn dragging_one_column_leaves_the_others_fitting() {
        let mut columns = ListColumnSet::new(&[ListColumn::Publisher, ListColumn::System]);
        let publisher = columns.position_of(ListColumn::Publisher).unwrap();
        let system = columns.position_of(ListColumn::System).unwrap();
        let mut fitted = vec![None; columns.len()];
        fitted[publisher] = Some(px(210.));
        fitted[system] = Some(px(95.));
        columns.set_fitted(fitted);

        // The table reports every column's width, one of them changed.
        let mut reported: Vec<_> = (0..columns.len()).map(|ix| columns.column(ix).width)
                                                     .collect();
        reported[publisher] = px(160.);
        columns.apply_widths(&reported);
        let mut refitted = vec![None; columns.len()];
        refitted[publisher] = Some(px(500.));
        refitted[system] = Some(px(120.));
        columns.set_fitted(refitted);

        assert_eq!(columns.column(publisher).width, px(160.));
        assert_eq!(columns.column(system).width, px(120.));
    }

    #[test]
    fn the_table_width_limits_a_fitted_column() {
        let mut columns = ListColumnSet::new(&[]);
        let title = columns.position_of(ListColumn::Title).unwrap();
        let mut fitted = vec![None; columns.len()];
        fitted[title] = Some(px(900.));
        columns.set_fitted(fitted);

        assert!(columns.set_table_width(px(1000.)));
        assert!(!columns.set_table_width(px(1000.)));

        assert_eq!(columns.column(title).width, px(600.));
    }

    #[test]
    fn showing_other_columns_forgets_the_fit() {
        let mut columns = ListColumnSet::new(&[ListColumn::Publisher]);
        let publisher = columns.position_of(ListColumn::Publisher).unwrap();
        let mut fitted = vec![None; columns.len()];
        fitted[publisher] = Some(px(210.));
        columns.set_fitted(fitted);

        columns.set_chosen(&[ListColumn::Publisher, ListColumn::Size]);

        let publisher = columns.position_of(ListColumn::Publisher).unwrap();
        assert_eq!(columns.column(publisher).width, px(130.));
    }
}
