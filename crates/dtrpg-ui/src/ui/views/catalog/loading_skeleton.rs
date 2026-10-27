//! The grey placeholders shown while the catalog loads for the first time.

use gpui_kit::component::skeleton::Skeleton;

use super::*;

/// How many placeholder rows or cards to draw.
const PLACEHOLDER_COUNT: usize = 8;

/// Title and subtitle bar widths for the placeholder rows, varied a little so
/// the rows do not look stamped out.
const ROW_BAR_WIDTHS: [(f32, f32); 4] = [(240.0, 150.0),
                                         (300.0, 190.0),
                                         (210.0, 160.0),
                                         (340.0, 130.0)];

/// Renders grey placeholders where the catalog will appear: rows for the
/// thumbnail layout and cards for the grid. The list layout has its own,
/// drawn by the table.
pub(super) fn render_loading_skeleton(presentation: CatalogPresentation, colors: &ColorTokens)
                                      -> AnyElement {
    match presentation {
        CatalogPresentation::Grid => placeholder_cards(),
        CatalogPresentation::List | CatalogPresentation::Thumbs => placeholder_rows(colors),
    }
}

fn placeholder_rows(colors: &ColorTokens) -> AnyElement {
    div().flex()
         .flex_col()
         .size_full()
         .children((0..PLACEHOLDER_COUNT).map(|row| {
                                             let (title_width, subtitle_width) =
                                                 ROW_BAR_WIDTHS[row % ROW_BAR_WIDTHS.len()];
                                             div().flex()
                            .items_center()
                            .gap(px(14.0))
                            .py(px(12.0))
                            .border_b_1()
                            .border_color(colors.border)
                            .child(Skeleton::new().w(px(44.0)).h(px(62.0)).rounded_md())
                            .child(div().flex()
                                        .flex_col()
                                        .gap(px(8.0))
                                        .child(Skeleton::new().w(px(title_width))
                                                              .h(px(12.0))
                                                              .rounded_md())
                                        .child(Skeleton::new().secondary()
                                                              .w(px(subtitle_width))
                                                              .h(px(10.0))
                                                              .rounded_md()))
                            .child(div().flex_1())
                            .child(Skeleton::new().secondary()
                                                  .w(px(80.0))
                                                  .h(px(10.0))
                                                  .rounded_md())
                                         }))
         .into_any_element()
}

fn placeholder_cards() -> AnyElement {
    div().flex()
         .flex_wrap()
         .gap(px(18.0))
         .children((0..PLACEHOLDER_COUNT).map(|_| {
                                             div().flex()
                                                  .flex_col()
                                                  .gap(px(8.0))
                                                  .child(Skeleton::new().w(px(150.0))
                                                                        .h(px(210.0))
                                                                        .rounded_md())
                                                  .child(Skeleton::new().w(px(120.0))
                                                                        .h(px(12.0))
                                                                        .rounded_md())
                                                  .child(Skeleton::new().secondary()
                                                                        .w(px(80.0))
                                                                        .h(px(10.0))
                                                                        .rounded_md())
                                         }))
         .into_any_element()
}
