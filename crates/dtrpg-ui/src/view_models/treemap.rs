//! A treemap: rectangles whose areas are in proportion to their values, laid
//! out with the squarified algorithm so they stay close to square and easy to
//! compare. No window and no I/O, so it can be tested directly.

use dtrpg_core::library::insights::Share;

/// A rectangle as shares of the whole, each from `0.0` to `1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Distance from the left edge.
    pub x: f64,
    /// Distance from the top edge.
    pub y: f64,
    /// Width.
    pub w: f64,
    /// Height.
    pub h: f64,
}

impl Rect {
    const EMPTY: Self = Self { x: 0.0,
                               y: 0.0,
                               w: 0.0,
                               h: 0.0, };
}

#[cfg(test)]
impl Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

/// One tile of a treemap: what it stands for, how much, and where it goes.
#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    /// The name of what the tile stands for.
    pub label: String,
    /// Its value, in megabytes.
    pub mb:    f64,
    /// Where it goes, as shares of the whole.
    pub rect:  Rect,
}

/// The largest ratio of a row's longer side to its shorter side if `areas` were
/// laid in a row along a side of length `side`.
fn worst_ratio(areas: &[f64], side: f64) -> f64 {
    let sum: f64 = areas.iter().sum();
    let largest = areas.iter().copied().fold(f64::MIN, f64::max);
    let smallest = areas.iter().copied().fold(f64::MAX, f64::min);
    let side_squared = side * side;
    (side_squared * largest / (sum * sum)).max(sum * sum / (side_squared * smallest))
}

/// Lays out rectangles for `values`, in order, inside a box of the given aspect
/// ratio (`width / height`), as shares of that box. Largest first gives the
/// best result. A value that is not above zero gets an empty rectangle.
///
/// # Examples
///
/// ```
/// use dtrpg_ui::view_models::treemap::layout;
///
/// let rects = layout(&[6.0, 3.0, 1.0], 2.0);
/// let total: f64 = rects.iter().map(|rect| rect.w * rect.h).sum();
/// assert!((total - 1.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn layout(values: &[f64], aspect: f64) -> Vec<Rect> {
    let mut rects = vec![Rect::EMPTY; values.len()];
    let kept: Vec<usize> = (0..values.len()).filter(|&i| values[i] > 0.0).collect();
    let total: f64 = kept.iter().map(|&i| values[i]).sum();
    if kept.is_empty() || aspect <= 0.0 {
        return rects;
    }
    // Work in a box `aspect` wide and one tall, so the areas add up to
    // `aspect`.
    let areas: Vec<f64> = kept.iter().map(|&i| values[i] / total * aspect).collect();
    let mut remaining = Rect { x: 0.0,
                               y: 0.0,
                               w: aspect,
                               h: 1.0, };
    let mut start = 0;
    while start < areas.len() {
        let side = remaining.w.min(remaining.h);
        let mut end = start + 1;
        let mut best = worst_ratio(&areas[start..end], side);
        while end < areas.len() {
            let next = worst_ratio(&areas[start..=end], side);
            if next > best {
                break;
            }
            best = next;
            end += 1;
        }
        let row_sum: f64 = areas[start..end].iter().sum();
        if remaining.w >= remaining.h {
            // A column down the left edge.
            let width = row_sum / remaining.h;
            let mut y = remaining.y;
            for (offset, area) in areas[start..end].iter().enumerate() {
                let height = area / width;
                rects[kept[start + offset]] = Rect { x: remaining.x,
                                                     y,
                                                     w: width,
                                                     h: height };
                y += height;
            }
            remaining.x += width;
            remaining.w -= width;
        }
        else {
            // A row along the top edge.
            let height = row_sum / remaining.w;
            let mut x = remaining.x;
            for (offset, area) in areas[start..end].iter().enumerate() {
                let width = area / height;
                rects[kept[start + offset]] = Rect { x,
                                                     y: remaining.y,
                                                     w: width,
                                                     h: height };
                x += width;
            }
            remaining.y += height;
            remaining.h -= height;
        }
        start = end;
    }
    // Back to shares of the box.
    for rect in &mut rects {
        rect.x /= aspect;
        rect.w /= aspect;
    }
    rects
}

/// The tiles for `shares` (largest first): the first `limit` each get a tile,
/// and the rest are added up into one tile called `other_label`, so a long tail
/// of small publishers does not become a dust of unreadable specks.
#[must_use]
pub fn tiles(shares: &[Share], limit: usize, other_label: &str, aspect: f64) -> Vec<Tile> {
    let mut entries: Vec<(String, f64)> = shares.iter()
                                                .take(limit)
                                                .map(|share| (share.label.clone(), share.mb))
                                                .collect();
    let rest: f64 = shares.iter().skip(limit).map(|share| share.mb).sum();
    if rest > 0.0 {
        entries.push((other_label.to_owned(), rest));
    }
    let rects = layout(&entries.iter().map(|(_, mb)| *mb).collect::<Vec<_>>(),
                       aspect);
    entries.into_iter()
           .zip(rects)
           .map(|((label, mb), rect)| Tile { label, mb, rect })
           .collect()
}

#[cfg(test)]
mod tests;
